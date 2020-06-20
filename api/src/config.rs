use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

lazy_static! {
    pub static ref CFG: Settings = Settings::new().unwrap_or_else(handle_error);
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct External {
    pub sentry: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Http {
    pub address: String,
    pub domain: String,
    pub session: Session,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Logger {
    pub pretty: bool,
    pub level: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Redis {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Session {
    pub age: i64,
    pub name: String,
    pub secure: bool,
    pub key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub external: External,
    pub redis: Redis,
    pub http: Http,
    pub logger: Logger,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::new();

        // Load from environment
        c.merge(Environment::with_prefix("api").separator("_"))?;

        // Load from file
        c.merge(File::with_name("settings").required(false))?;

        // Deserialize into settings
        c.try_into()
    }
}

/// Handle errors generated from parsing configuration
pub fn handle_error(result: ConfigError) -> Settings {
    match result {
        ConfigError::Frozen => unreachable!(),
        ConfigError::NotFound(file) => println!("Configuration file '{}' does not exist", file),
        ConfigError::PathParse(kind) => {
            println!("Configuration path could not be parsed '{:?}'", kind)
        }
        ConfigError::FileParse { uri: _, cause } => println!("Failed to parse file {}", cause),
        ConfigError::Type {
            origin: _,
            unexpected,
            expected,
            key,
        } => println!(
            "Expected {} for {}, got {}",
            expected,
            key.unwrap_or("<no key>".to_string()),
            unexpected
        ),
        ConfigError::Message(msg) => println!("Failed to parse configuration: {}", msg),
        ConfigError::Foreign(err) => {
            println!("Unexpected error while parsing configuration: {}", err)
        }
    };
    std::process::exit(1);
}
