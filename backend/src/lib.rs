use async_graphql::http::playground_source;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use bytes::Bytes;
use color_eyre::Result;
use filestore::FileStore;
use http::{Request, StatusCode};
use human_bytes::human_bytes;
use image::ImageFormat;
use mime_guess::Mime;
use schema::Schema;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::{io::Cursor, str::FromStr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{error_span, info};
use url::Url;

use tower_http::compression::CompressionLayer;

use crate::filestore::File;
use image::io::Reader as ImageReader;
mod config;
mod filestore;
mod schema;

pub async fn run() -> Result<()> {
    let settings = config::get_settings()?;

    // Set up database
    let options = PgConnectOptions::from_url(&Url::parse(&settings.database_url)?)?;
    let db_pool = PgPoolOptions::new().connect_with(options).await?;

    let mut filestore = filestore::FileStore::new(settings.application.file_storage_path)?;

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

    let schema = schema::get_schema();

    let cors = CorsLayer::new()
        .allow_methods([http::Method::POST])
        .allow_origin(Any) // TODO  change this to a specific origin
        .allow_headers([http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/upload", post(upload_file))
        .nest_service("/files", filestore.service())
        .layer(cors)
        .layer(Extension(schema))
        .layer(Extension(Arc::new(filestore)))
        .layer(CompressionLayer::new())
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
        .layer(RequestIdLayer);

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

async fn health_check() -> &'static str {
    "Ok\n"
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(TryFromMultipart)]
struct UploadFileRequest {
    file: FieldData<Bytes>,
}

async fn upload_file(
    Extension(filestore): Extension<Arc<FileStore>>,
    TypedMultipart(multipart): TypedMultipart<UploadFileRequest>,
) -> Result<String, (StatusCode, String)> {
    let mime_type = Mime::from_str(&multipart.file.metadata.content_type.ok_or((
        StatusCode::BAD_REQUEST,
        "No content-type provided".to_owned(),
    ))?)
    .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid content-type".to_owned()))?;

    match filestore.upload(mime_type, multipart.file.contents).await {
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
