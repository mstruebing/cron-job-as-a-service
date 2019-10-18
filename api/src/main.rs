// Own modules
use shared::database;
use shared::error::Result;
use shared::model::job::Job;
use shared::model::secret::Secret;
use shared::model::user::User;
use shared::repository::job;
use shared::repository::secret;
use shared::repository::user;
use shared::utils;

// Contains nonsense currently, just to test these funcs :)
fn main() -> Result<()> {
    test_save_delete()?;
    save_something_for_worker()?;

    Ok(())
}

fn test_save_delete() -> Result<()> {
    database::reset()?;
    let user = User::new();
    let user = user::save(user)?;

    let job = Job::new();
    let job = job::save(job, user.id.unwrap())?;

    let secret = Secret::new();
    secret::save(secret.clone(), job.id.unwrap())?;

    let user = user.id(Some(1)).email("someone@example.com");
    let user = user::update(user)?;

    let job = job.id(Some(1)).command("echo hello");
    job::update(job.clone())?;

    let secret = secret.id(Some(1)).key("hello").value("world");
    secret::update(secret.clone())?;

    secret::delete(secret)?;
    job::delete(job)?;
    user::delete(user)?;

    Ok(())
}

fn save_something_for_worker() -> Result<()> {
    let current_timestamp = utils::get_current_timestamp();
    let schedule = "* * * * * ";
    let jobs = vec![Job::new()
        .schedule(schedule)
        .command("echo $(date +%s) $hello >> world.txt")
        .last_run(current_timestamp)
        .next_run(utils::get_next_run(schedule))
        .secrets(vec![
            Secret::new().key("hello").value("world"),
            Secret::new().key("world").value("hello"),
        ])];
    let user = User::new()
        .email("someone@example.com")
        .password("pass")
        .jobs(jobs);

    user::save(user)?;

    Ok(())
}
