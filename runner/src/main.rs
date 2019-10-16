// stdlib
use std::process::Command;

// Own modules
use shared::database;
use shared::error::Result;
use shared::model::job::Job;
use shared::model::secret::Secret;
use shared::utils;

fn main() -> Result<()> {
    let jobs = get_next_jobs()?;

    for job in jobs {
        run(job)?;
    }

    Ok(())
}

pub fn run(job: Job) -> Result<()> {
    let mut args: String = "".to_owned();

    for secret in job.secrets.clone() {
        args = args + &secret.get_as_string();
    }

    if !job.secrets.is_empty() {
        args = format!("{}; {}", args, job.command);
    } else {
        args = job.command.to_string();
    }

    Command::new("sh").arg("-c").arg(args).spawn()?;

    Ok(())
}

// TODO: Add limit
// TOOD: Get complete job
// TODO: Get command
fn get_next_jobs() -> Result<Vec<Job>> {
    let connection = database::connection()?;
    let mut jobs = vec![];

    for row in &connection.query(
        "SELECT id, command, command FROM jobs WHERE next_run > $1 ORDER BY next_run",
        &[&utils::get_current_timestamp()],
    )? {
        println!("{:?}", row);
        let id: Option<i32> = Some(row.get(0));

        jobs.push(
            Job::new()
                .id(id)
                .command("echo $(date +%s) $hello >> world.txt")
                .secrets(vec![Secret::new().key("hello").value("world")]),
        );
    }

    Ok(jobs)
}
