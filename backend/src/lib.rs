use async_graphql::http::playground_source;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use auth::{Auth0Validator, Claims};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, Cookie},
    middleware::Next,
    response::{self, IntoResponse, Response},
    routing::{get, post},
    Extension, Router, TypedHeader,
};
use axum_macros::debug_handler;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use bytes::Bytes;
use color_eyre::Result;
use filestore::FileStore;
use http::{HeaderValue, Request, StatusCode};
use human_bytes::human_bytes;
use image::ImageFormat;
use mime_guess::Mime;
use schema::Schema;
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::{io::Cursor, str::FromStr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{error_span, info};
use url::Url;

use tower_http::compression::CompressionLayer;

use crate::filestore::File;
use image::io::Reader as ImageReader;
mod auth;
pub mod config;
mod filestore;
mod schema;

#[derive(Clone)]
struct AppState {
    auth_validator: Auth0Validator,
    schema: Schema,
    filestore: Arc<FileStore>,
}

pub async fn run(settings: config::Settings) -> Result<()> {
    // Set up database
    let options = PgConnectOptions::from_url(&Url::parse(&settings.database_url.expose_secret())?)?;
    let _db_pool = PgPoolOptions::new().connect_with(options).await?;

    let mut filestore = filestore::FileStore::new(settings.application.file_storage_path)?;

    let auth_validator =
        auth::Auth0Validator::new(settings.auth.authority, settings.auth.audience).await?;

    // Convert images to webp
    filestore.register_transformer(Box::new(|file| {
        let format = ImageFormat::from_mime_type(&file.mime_type)?;

        let img = ImageReader::with_format(Cursor::new(file.bytes.clone()), format)
            .decode()
            .ok()?;

        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP)
            .ok()?;

        let bytes = Bytes::from(bytes);

        let original_filesize = file.bytes.len();
        let webp_filesize = bytes.len();

        info!(
            "WebP compression {} filesize from {} to {} ({:+.2}%)",
            if webp_filesize < original_filesize {
                "reduced"
            } else {
                "increased"
            },
            human_bytes(original_filesize as f64),
            human_bytes(webp_filesize as f64),
            100.0 * (webp_filesize as f64 - original_filesize as f64) / original_filesize as f64
        );

        // Only use the webp if it's smaller
        if webp_filesize < original_filesize {
            Some(File {
                bytes,
                mime_type: Mime::from_str("image/webp").unwrap(),
            })
        } else {
            None
        }
    }));

    let filestore = Arc::new(filestore);

    let schema = schema::get_schema();

    let state = AppState {
        auth_validator,
        schema,
        filestore: filestore.clone(),
    };

    let allowed_origins = AllowOrigin::list(
        settings
            .application
            .allowed_origins
            .iter()
            .map(|origin| HeaderValue::from_str(origin.as_ref()))
            .collect::<Result<Vec<_>, _>>()?,
    );

    info!("Allowed origins: {:?}", allowed_origins);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/upload", post(upload_file))
        .nest_service("/files", filestore.service())
        .layer(
            ServiceBuilder::new()
                .layer(RequestIdLayer)
                .layer(
                    TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                        // We get the request id from the extensions
                        let request_id = request
                            .extensions()
                            .get::<RequestId>()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| "unknown".into());
                        // And then we put it along with other information into the `request` span
                        error_span!(
                            "request",
                            id = %request_id,
                            method = %request.method(),
                            uri = %request.uri(),
                        )
                    }),
                )
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_methods([http::Method::POST, http::Method::GET])
                        .allow_credentials(true)
                        .allow_origin(allowed_origins) // TODO  change this to a specific origin
                        .allow_headers([http::header::CONTENT_TYPE, http::header::COOKIE]),
                )
                .layer(axum::middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .with_state(state);

    let listener = std::net::TcpListener::bind(format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))?;

    let addr = listener.local_addr()?;
    tracing::debug!("listening on http://{}", addr);
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn auth_middleware<B>(
    State(state): State<AppState>,
    auth_cookie: Option<TypedHeader<Cookie>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    // Extract the Bearer token from the Authorization header
    if let Some(access_token) = auth_cookie
        .map(|TypedHeader(c)| c.get("f1_warre_dev_access_token").map(ToOwned::to_owned))
        .flatten()
    {
        match state.auth_validator.validate_token(&access_token).await {
            Ok(claims) => {
                // Add the claims to the request extensions
                tracing::debug!("Authenticated user: {}", claims.sub);
                req.extensions_mut().insert(claims);
            }
            Err(err) => {
                tracing::error!("Error while validating token: {}", err);
            }
        }
    }

    next.run(req).await
}

async fn health_check(claim: Option<Extension<Claims>>) -> String {
    if let Some(Extension(claims)) = claim {
        format!("Ok\nLogged in as {}", claims.sub)
    } else {
        "Ok\nNot logged in".to_owned()
    }
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[derive(TryFromMultipart)]
struct UploadFileRequest {
    file: FieldData<Bytes>,
}

async fn upload_file(
    State(state): State<AppState>,
    TypedMultipart(multipart): TypedMultipart<UploadFileRequest>,
) -> Result<String, (StatusCode, String)> {
    let mime_type = Mime::from_str(&multipart.file.metadata.content_type.ok_or((
        StatusCode::BAD_REQUEST,
        "No content-type provided".to_owned(),
    ))?)
    .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid content-type".to_owned()))?;

    match state
        .filestore
        .upload(mime_type, multipart.file.contents)
        .await
    {
        Ok(url) => Ok(url),
        Err(err) => {
            tracing::error!("Error while uploading file: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to upload file".to_string(),
            ))
        }
    }
}

#[debug_handler]
async fn graphql_handler(
    state: State<AppState>,
    claims: Option<Extension<Claims>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    tracing::info!("Claims: {:?}", claims);
    let req = req.into_inner();
    let req = if let Some(Extension(claims)) = claims {
        req.data(claims)
    } else {
        req
    };
    state.schema.execute(req).await.into()
}
