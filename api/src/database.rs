use crate::{config::CFG, errors::ApiError};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let manager = ConnectionManager::<PgConnection>::new(&CFG.database.url);
        match Pool::new(manager) {
            Ok(pool) => pool,
            Err(err) => {
                println!("Failed to connect to database: {}", err);
                std::process::exit(1);
            }
        }
    };
}

embed_migrations!();

/// Connect to the database
pub fn connect() {
    lazy_static::initialize(&POOL);
    let conn = connection().unwrap_or_else(|e| {
        println!("Failed to retrieve database connection: {}", e);
        std::process::exit(1);
    });
    embedded_migrations::run(&conn).unwrap();
}

/// Retrieve a database connection
pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get().map_err(|e| {
        ApiError::new(
            500,
            format!("Failed to retrieve database connection: {}", e),
        )
    })
}
