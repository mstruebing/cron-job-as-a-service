// stdlib
use std::env;

// modules
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use dotenv::dotenv;

// own modules
use crate::error::Result;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<PgPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = init_pool(&database_url)?;
    Ok(pool)
}

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
