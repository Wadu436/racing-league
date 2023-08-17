use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: Box<str>,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let base_dir = std::env::current_dir().unwrap();
    let config_dir = base_dir.join("config");

    // Environment
    let environment: Environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".to_owned())
        .as_str()
        .try_into()
        .expect("Failed to parse the `ENVIRONMENT` environment value.");
    let environment_source = format!("{}.yml", environment.as_str());

    let settings = Config::builder()
        .add_source(config::File::from(config_dir.join("base.yml")))
        .add_source(config::File::from(config_dir.join(environment_source)))
        .build()?;

    tracing::debug!(environment = environment.as_str(), "loading configuration");
    settings.try_deserialize::<Settings>()
}

#[derive(Debug)]
enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<&str> for Environment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "`{}` is not a supported environment. Use either `local` or `production`",
                other
            )),
        }
    }
}
