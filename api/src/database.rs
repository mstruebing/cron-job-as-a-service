extern crate postgres;

use std::env;

use crate::job::Job;
use crate::secret::Secret;
use crate::user::User;
use postgres::{Connection, Error, TlsMode};

pub fn connection() -> Result<Connection, Error> {
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
}

pub fn reset() -> Result<(), Error> {
    let connection = connection()?;
    connection.execute(Secret::drop_table(), &[])?;
    connection.execute(Job::drop_table(), &[])?;
    connection.execute(User::drop_table(), &[])?;

    connection.execute(User::create_table(), &[])?;
    connection.execute(Job::create_table(), &[])?;
    connection.execute(Secret::create_table(), &[])?;

    Ok(())
}
