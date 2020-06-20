use crate::{config::CFG, errors::ApiError};
use redis::ConnectionLike;

type Pool = r2d2::Pool<RedisConnectionManager>;
pub type RedisConnection = r2d2::PooledConnection<RedisConnectionManager>;

lazy_static! {
    static ref POOL: Pool = {
        let manager = RedisConnectionManager::new(CFG.redis.address.clone()).unwrap_or_else(|e| {
            println!("Failed to connect to redis: {}", e);
            std::process::exit(1);
        });
        match Pool::new(manager) {
            Ok(pool) => pool,
            Err(err) => {
                println!("Failed to create redis pool: {}", err);
                std::process::exit(1);
            }
        }
    };
}

/// Connect to redis
pub fn connect() {
    lazy_static::initialize(&POOL);
    connection().unwrap_or_else(|e| {
        println!("Failed to retrieve redis connection: {}", e);
        std::process::exit(1);
    });
}

/// Retrieve a redis connection
pub fn connection() -> Result<RedisConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed to retrieve redis connection: {}", e)))
}

// This is the same implementation as r2d2-redis but with redis 0.16 instead of 0.15
// https://github.com/sorccu/r2d2-redis/blob/master/src/lib.rs#L85-L122
#[derive(Debug)]
pub struct RedisConnectionManager {
    connection_info: redis::ConnectionInfo,
}

impl RedisConnectionManager {
    pub fn new<T: redis::IntoConnectionInfo>(
        params: T,
    ) -> Result<RedisConnectionManager, redis::RedisError> {
        Ok(RedisConnectionManager {
            connection_info: params.into_connection_info()?,
        })
    }
}

impl r2d2::ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = redis::RedisError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match redis::Client::open(self.connection_info.clone()) {
            Ok(client) => client.get_connection(),
            Err(err) => Err(err),
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        redis::cmd("PING").query(conn)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_open()
    }
}
