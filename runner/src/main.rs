#[macro_use]
extern crate chan;

// stdlib
use std::process::Command;

// Own modules
use shared::database;
use shared::error::Result;
use shared::logger::{debug, log};
use shared::model::job::Job;
use shared::repository::job;
use shared::utils;

fn main() {
    // TODO: is this a good tick interval?
    let tick = chan::tick_ms(1000);

    loop {
        chan_select! {
                tick.recv() => {
                    let jobs = get_next_jobs();

                    match jobs {
                        Ok(jobs) => {
                            for job in jobs {
                                let current_timestamp = utils::get_current_timestamp();
                                debug(&format!("current_timestamp: {}, next_run: {}, diff: {}", current_timestamp, job.next_run, job.next_run - current_timestamp));

                                if job.next_run == current_timestamp {
                                    let result = run(job);
                                    match result {
                                        Ok(job) => log(&format!("Successyull ran job: {}", job.id.unwrap())),
                                        Err(err) => log(&format!("ERROR: {:?}", err)),
                                    }
                                } else {
                                    debug("No job to execute")
                                }
                            }

                        },
                        Err(err) => log(&format!("ERROR: {:?}", err))
                    }

                }
        ,
            }
    }
}

pub fn run(job: Job) -> Result<Job> {
    let mut args: String = "".to_owned();

    for secret in job.secrets.clone() {
        args = args + " " + &secret.get_as_string();
    }

    if !job.secrets.is_empty() {
        args = format!("{}; {}", args, job.command);
    } else {
        args = job.command.to_string();
    }

    debug(&format!("job: {}, command: {}", job.id.unwrap(), args));
    Command::new("sh").arg("-c").arg(args).spawn()?;

    let job = job.last_run(utils::get_current_timestamp());
    let job = job.clone().next_run(utils::get_next_run(&job.schedule));
    let job = job::update(job)?;

    Ok(job)
}

// TODO: Add limit
fn get_next_jobs() -> Result<Vec<Job>> {
    let connection = database::connection()?;
    let mut jobs = vec![];

    for row in &connection.query(
        "SELECT id, command, next_run, last_run, schedule
        FROM jobs
        WHERE jobs.next_run >= $1
        ORDER BY jobs.next_run;",
        &[&utils::get_current_timestamp()],
    )? {
        let job = utils::convert_row_to_job(row).to_owned();
        let job = job.clone().secrets(utils::get_secrets_for_job(&job)?);

        jobs.push(job);
    }

    Ok(jobs)
}
