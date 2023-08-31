use color_eyre::Result;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Needs to go first so dotenv gets called
    backend::config::load_dotenv();

    color_eyre::install()?;
    LogTracer::init()?;

    let subscriber = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).finish();

    set_global_default(subscriber).expect("Failed to set subscriber");

    let settings = backend::config::get_settings()?;

    backend::run(settings).await
}
