// stdlib
use std::env;

// modules
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use dotenv::dotenv;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_pool(&database_url).expect("Failed to craeate pool")
}

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
