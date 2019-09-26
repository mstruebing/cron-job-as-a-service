mod database;
mod job;
mod secret;
mod user;
mod utils;

use postgres::Error;

// Contains nonsense currently, just to test these funcs :)
fn main() -> Result<(), Error> {
    reset_db()?;
    save_user()?;
    delete_user()?;

    Ok(())
}

pub fn reset_db() -> Result<(), Error> {
    database::reset()?;
    Ok(())
}

pub fn save_user() -> Result<(), Error> {
    create_user(None).save()?;
    Ok(())
}

pub fn delete_user() -> Result<(), Error> {
    create_user(Some(1)).delete()?;
    Ok(())
}

pub fn create_user(id: Option<i32>) -> user::User {
    user::User::new(
        id,
        "someone@example.com",
        "abcdefg1",
        vec![create_job(None)],
    )
}

pub fn create_job(id: Option<i32>) -> job::Job {
    job::Job::new(
        id,
        "0 * * * *",
        "echo $hello",
        utils::get_current_timestamp(),
        utils::get_current_timestamp() + 1,
        vec![create_secret(None)],
    )
}

pub fn create_secret(id: Option<i32>) -> secret::Secret {
    secret::Secret::new(id, "hello", "world")
}
