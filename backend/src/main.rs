use color_eyre::Result;
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LogTracer::init()?;

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    set_global_default(subscriber).expect("Failed to set subscriber");

    backend::run().await
}
