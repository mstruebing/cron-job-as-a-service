mod job;
mod secret;
mod user;

fn main() {
    let secret = secret::Secret::new("hello", "world");
    secret.show();
    let job = job::Job::new("0 * * * *", "echo $hello", 0, 1, vec![secret.clone()]);
    let mut user = user::User::new("max@mustermann.de", "abcdefg1", vec![job.clone()]);

    let job = job::Job::new(
        "0 * * * *",
        "echo $hello Motherfucker",
        0,
        1,
        vec![secret.clone()],
    );
    user.add_job(job.clone());

    println!("{:?}", user);
}
