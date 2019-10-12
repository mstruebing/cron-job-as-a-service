// Other modules
use std::process::Command;

// Own modules
use shared::error::Result;
use shared::model::job::Job;

fn main() {
    println!("Hello, world!");

    Job::new();
}

pub fn run(job: Job) -> Result<()> {
    let mut args: String = "".to_owned();

    for secret in job.secrets {
        args = args + &secret.get_as_string();
    }

    let args = format!("{}; {}", args, job.command);
    Command::new("sh").arg("-c").arg(args).output()?;
    Ok(())
}
