// stdlib
use std::env;

// modules
use dotenv::dotenv;
use postgres::{Connection, TlsMode};

// internal
use crate::error::{Error, Result};
use crate::model::job::Job;
use crate::model::secret::Secret;
use crate::model::user::User;

pub fn connection() -> Result<Connection> {
    dotenv()?;
    Connection::connect(
        format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("DATABASE_USER").unwrap(),
            env::var("DATABASE_PASSWORD").unwrap(),
            env::var("DATABASE_HOST").unwrap(),
            env::var("DATABASE_PORT").unwrap(),
            env::var("DATABASE_NAME").unwrap(),
        ),
        TlsMode::None,
    )
    .map_err(Error::Postgres)
}

pub fn reset() -> Result<()> {
    let connection = connection()?;
    connection.execute(Secret::drop_table_query(), &[])?;
    connection.execute(Job::drop_table_query(), &[])?;
    connection.execute(User::drop_table_query(), &[])?;

    connection.execute(User::create_table_query(), &[])?;
    connection.execute(Job::create_table_query(), &[])?;
    connection.execute(Secret::create_table_query(), &[])?;

    Ok(())
}
