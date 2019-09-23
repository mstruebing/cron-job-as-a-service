mod job;
mod secret;
mod user;

fn main() {
    let secret = secret::Secret::new("hello", "world");
    secret.show();

    let secret = secret::Secret::new("hello", "world");
    let job = job::Job::new("0 * * * *", "echo $hello", 0, 1, true, vec![secret]);
    let user = user::User::new("max@mustermann.de", "abcdefg1", vec![job]);

    print!("{:?}", user);
}
