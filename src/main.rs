mod job;
mod secret;

fn main() {
    let secret = secret::Secret::new("hello", "world");
    secret.show();

    let mut secrets = Vec::new();
    secrets.push(secret::Secret::new("hello", "world"));

    let job = job::Job::new("0 * * * *", "echo $hello", 0, 1, true, secrets);

    println!("{:?}", job);
    println!("{:?}", secret);

    println!("Hello, world!");
}
