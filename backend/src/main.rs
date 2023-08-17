use async_graphql::http::playground_source;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use color_eyre::eyre::Result;
use http::Request;
use schema::Schema;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{error_span, subscriber::set_global_default, Level};
use tracing_log::LogTracer;

mod config;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LogTracer::init()?;
    // tracing_subscriber::fmt::init();
    // let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(LevelFilter::Debug).finish();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    set_global_default(subscriber).expect("Failed to set subscriber");

    let settings = config::get_settings()?;

    let schema = schema::get_schema();

    let cors = CorsLayer::new()
        .allow_methods([http::Method::POST])
        .allow_origin(Any) // TODO  change this to a specific origin
        .allow_headers([http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(cors)
        .layer(Extension(schema))
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
