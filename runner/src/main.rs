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
fn get_next_jobs() -> Result<Vec<Job>> {
    let connection = database::connection()?;
    let mut jobs = vec![];

    for row in &connection.query(
        "SELECT id, command, next_run, last_run, schedule
        FROM jobs
        WHERE jobs.next_run > $1
        ORDER BY jobs.next_run;",
        &[&utils::get_current_timestamp()],
    )? {
        let job = utils::convert_row_to_job(row).to_owned();
        let job = job.clone().secrets(get_secrets_for_job(&job)?);

        jobs.push(job);
    }

    Ok(jobs)
}

fn get_secrets_for_job(job: &Job) -> Result<Vec<Secret>> {
    let connection = database::connection()?;
    let mut secrets = vec![];

    for row in &connection.query(
        "SELECT id, key, value
        FROM secrets
        WHERE job_id = $1",
        &[&job.id.unwrap()],
    )? {
        secrets.push(utils::convert_row_to_secret(row));
    }

    Ok(secrets)
}
