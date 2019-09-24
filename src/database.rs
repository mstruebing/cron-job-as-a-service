extern crate postgres;

use crate::job::Job;
use crate::secret::Secret;
use crate::user::User;
use postgres::{Connection, Error, TlsMode};

fn connection() -> Connection {
    // TODO: Use env variables/secrets
    Connection::connect(
        "postgres://postgres:postgres@localhost:5432/cronjob_as_a_service",
        TlsMode::None,
    )
    .unwrap()
}

pub fn reset() -> Result<(), Error> {
    connection().execute(Secret::drop_table(), &[])?;
    connection().execute(Job::drop_table(), &[])?;
    connection().execute(User::drop_table(), &[])?;

    connection().execute(User::create_table(), &[])?;
    connection().execute(Job::create_table(), &[])?;
    connection().execute(Secret::create_table(), &[])?;

    Ok(())
}
