#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use crate::config::CFG;
use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{middleware, App, HttpServer};
use std::str::FromStr;

mod config;
mod database;
mod errors;
mod models;
mod project_format;
mod redis;
mod routes;
mod schema;

// Log format string
static LOG_FORMAT: &str = "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Read environment
    dotenv::dotenv().ok();

    // Connect to dependent services
    database::connect();
    redis::connect();

    // Build server and bind
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new(LOG_FORMAT))
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Compress::default())
            .wrap(Cors::new()
                .allowed_origin("https://backendless.tech")
                .allowed_origin("http://localhost:3000")
                .supports_credentials()
                .finish())
            .wrap(configure_session())
            .configure(routes::authentication)
            .configure(routes::users)
            .configure(routes::projects)
            .configure(routes::deployments)
    })
    .server_hostname(&CFG.http.domain)
    .bind(&CFG.http.address)?;

    // Initialize Sentry
    initialize_sentry();

    // Run server
    server.run().await
}

/// Initialize sentry configuration
fn initialize_sentry() {
    let guard = sentry::init(CFG.external.sentry.clone());
    if guard.is_enabled() {
        sentry::integrations::panic::register_panic_handler();

        let logger = build_logger(&CFG.logger);
        let options = sentry::integrations::log::LoggerOptions {
            global_filter: Some(logger.filter()),
            ..Default::default()
        };
        sentry::integrations::log::init(Some(Box::new(logger)), options);
    }
}

/// Create the logger configuration
fn build_logger(logger: &config::Logger) -> env_logger::Logger {
    // Choose logger type
    let mut builder = if logger.pretty {
        pretty_env_logger::formatted_builder()
    } else {
        env_logger::Builder::from_default_env()
    };

    // Set log level
    let level = log::LevelFilter::from_str(logger.level.as_str()).unwrap_or(log::LevelFilter::Info);
    builder.filter_level(level);

    // Set time precision
    builder.format_timestamp_millis();

    return builder.build();
}

/// Configure the session store
fn configure_session() -> RedisSession {
    // Parse the duration
    let max_age = chrono::Duration::hours(CFG.http.session.age);

    // Ensure proper key length
    let bytes = CFG.http.session.key.as_bytes();
    if bytes.len() < 32 {
        println!("Session key must be greater than 32 bytes");
        std::process::exit(1);
    }

    // Remove redis:// from start
    let connection_string = CFG.redis.address.get(8..).unwrap_or_else(|| {
        println!("Invalid redis connection address");
        std::process::exit(1);
    });

    // Create session
    RedisSession::new(connection_string, &CFG.http.session.key.as_bytes())
        .cookie_same_site(actix_redis::SameSite::Strict)
        .cookie_path("/")
        .cookie_domain(&CFG.http.domain)
        .cookie_name(&CFG.http.session.name)
        .cookie_secure(CFG.http.session.secure)
        .cookie_max_age(max_age)
}
