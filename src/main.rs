mod database;
mod job;
mod secret;
mod user;

use postgres::Error;

fn main() -> Result<(), Error> {
    let secret = secret::Secret::new("hello", "world");
    let job = job::Job::new("0 * * * *", "echo $hello", 0, 1, vec![secret.clone()]);
    let mut user = user::User::new(None, "someone@example.com", "abcdefg1", vec![job.clone()]);

    let job = job::Job::new(
        "0 * * * *",
        "echo $hello Motherfucker",
        0,
        1,
        vec![secret.clone()],
    );
    user.add_job(job.clone());

    database::reset()?;
    let mut user = user.save()?;

    // TODO: jobs are getting saved two times :(
    user.email = "changed@mail.com";
    user.save()?;

    Ok(())
}
