// Own modules
use shared::database;
use shared::error::Result;
use shared::model::job::Job;
use shared::model::secret::Secret;
use shared::model::user::User;

// internal
mod job;
mod secret;
mod user;

// Contains nonsense currently, just to test these funcs :)
fn main() -> Result<()> {
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
    job::update(job.clone(), user.id.unwrap())?;

    let secret = secret.id(Some(1)).key("hello").value("world");
    secret::update(secret.clone(), job.id.unwrap())?;

    secret::delete(secret)?;
    job::delete(job)?;
    user::delete(user)?;

    Ok(())
}
