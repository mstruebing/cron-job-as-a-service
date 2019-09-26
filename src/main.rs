mod database;
mod job;
mod secret;
mod user;
mod utils;

use postgres::Error;

// Contains nonsense currently, just to test these funcs :)
fn main() -> Result<(), Error> {
    let secret = secret::Secret::new("hello", "world");
    let job = job::Job::new(None, "0 * * * *", "echo $hello", 0, 1, vec![secret.clone()]);
    let mut user = user::User::new(None, "someone@example.com", "abcdefg1", vec![job.clone()]);

    let job = job::Job::new(
        None,
        "0 * * * *",
        "echo $hello Motherfucker",
        utils::get_current_timestamp(),
        utils::get_current_timestamp() + 1,
        vec![secret.clone()],
    );
    user.add_job(job);

    database::reset()?;
    let mut user = user.save()?;

    user.email = "changed@mail.com";
    user.save()?;

    let job = job::Job::new(
        None,
        "0 * * * *",
        "echo later added job",
        utils::get_current_timestamp(),
        utils::get_current_timestamp() + 1,
        vec![secret.clone()],
    );
    user.add_job(job);
    user.save()?;

    let job = job::Job::new(
        Some(1),
        "what",
        "what",
        utils::get_current_timestamp(),
        utils::get_current_timestamp() + 1,
        vec![],
    );

    user.remove_job(job.clone());
    user.add_job(job.clone());
    job.delete()?;
    user.save()?;

    let mut user_two = user.clone();
    user.delete()?;

    user_two.save()?;

    Ok(())
}
