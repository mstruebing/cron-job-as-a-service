use dotenv;

mod database;
mod job;
mod secret;
mod user;

use postgres::Error;
use shared::utils;

// Contains nonsense currently, just to test these funcs :)
fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    reset_db()?;

    // test save
    save_user()?;
    delete_user(Some(1))?;

    // prepare test delete
    save_user()?;
    let secret = create_secret(Some(2));
    let job = create_job(Some(2));

    // test delete
    secret.delete()?;
    job.delete()?;
    delete_user(Some(2))?;

    // test update
    save_user()?;
    update_secret(Some(3))?;
    update_job(Some(3))?;
    update_user(Some(3))?;

    // Cleanup
    delete_user(Some(3))?;

    // Test job execution
    let job = job::Job::new(
        None,
        "",
        "echo hello $hello > world.txt",
        0,
        0,
        vec![create_secret(None)],
    );

    job.run()?;

    println!("{}, ", utils::get_next_run("* * * * *"));

    save_user()?;

    Ok(())
}

pub fn reset_db() -> Result<(), Error> {
    database::reset()?;
    Ok(())
}

pub fn save_user() -> Result<user::User, Error> {
    let user = create_user(None).save()?;
    Ok(user)
}

pub fn delete_user(id: Option<i32>) -> Result<(), Error> {
    create_user(id).delete()?;
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
        "echo hello $hello > world.txt",
        utils::get_current_timestamp(),
        utils::get_current_timestamp() + 1,
        vec![create_secret(None)],
    )
}

pub fn create_secret(id: Option<i32>) -> secret::Secret {
    secret::Secret::new(id, "hello", "world")
}

pub fn update_secret(id: Option<i32>) -> Result<(), Error> {
    let mut secret = create_secret(id);
    secret.key = "CHANGED";
    secret.value = "SECRET";
    secret.save(id.unwrap())?;
    Ok(())
}

pub fn update_job(id: Option<i32>) -> Result<(), Error> {
    let mut job = create_job(id);
    job.command = "CHANGED COMMAND";
    job.update(id.unwrap())?;

    Ok(())
}

pub fn update_user(id: Option<i32>) -> Result<(), Error> {
    let mut user = create_user(id);
    user.email = "changed@mail.com";
    user.save()?;

    Ok(())
}
