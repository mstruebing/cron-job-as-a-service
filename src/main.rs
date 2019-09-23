mod secret;

fn main() {
    let secret = secret::Secret::new("hello", "world");
    secret.show();

    println!("Hello, world!");
}
