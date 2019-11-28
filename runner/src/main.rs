// // stdlib
use std::{process::Command, str};

// modules
use chan::chan_select;
use diesel::prelude::*;

// // Own modules
use shared::database::{establish_connection, PgPool};
use shared::error::Result;
use shared::logger::log;
use shared::models::job::{get_secrets, Job};
use shared::schema::jobs;
use shared::utils;

fn check_requirements() -> Result<()> {
    if !utils::is_installed("docker") {
        panic!("docker needs to be installed");
    }

    // TODO: This seems a bit hardcoded, unflexible and fragile to me
    // Make sure to build the docker image inside this program
    let output = Command::new("docker").arg("images").output()?;

    if !str::from_utf8(&output.stdout).unwrap().contains("caas") {
        panic!("caas image needed for runner");
    }

    Ok(())
}

fn main() -> Result<()> {
    check_requirements()?;

    // TODO: is this a good tick interval?
    let tick = chan::tick_ms(1000);
    let pool = establish_connection()?;

    loop {
        chan_select! {
            tick.recv() => {
                let current_timestamp = utils::get_current_timestamp();

                match get_jobs_to_run(&pool, current_timestamp) {
                    Ok(jobs) => {
                        for job in jobs {
                            match run(&pool, job) {
                                Ok(job) => log(&format!("Successfully ran job: {:?}",job)),
                                Err(err) => log(&format!("ERROR: {:?}", err)),
                            }
                        }
                    },
                    Err(err) => log(&format!("ERROR: {:?}", err))
                }
            },
        }
    }
}

pub fn run(pool: &PgPool, job: Job) -> Result<Job> {
    let secrets = get_secrets(&job, &pool)?;
    let args = if !secrets.is_empty() {
        let args = secrets
            .iter()
            .fold(String::new(), |acc, next| acc + " " + &next.get_as_string());
        format!("{}; {}", args, job.command)
    } else {
        job.command.to_string()
    };

    Command::new("docker")
        .arg("run")
        .arg("caas") // TODO: Build image in this program
        .arg(args)
        .spawn()?;

    let job = job.last_run(utils::get_current_timestamp());
    let next_run = utils::get_next_run(&job.schedule);
    let job = job.next_run(next_run);

    let connection = pool.get()?;
    diesel::update(jobs::dsl::jobs.find(job.id))
        .set(&job)
        .execute(&connection)?;

    Ok(job)
}

fn get_jobs_to_run(pool: &PgPool, current_timestamp: i32) -> Result<Vec<Job>> {
    let connection = pool.get()?;
    let jobs = jobs::dsl::jobs
        // TODO: is `eq` sufficient? maybe use `ge` greater-equal
        .filter(jobs::dsl::next_run.eq(current_timestamp))
        .load::<Job>(&connection)?;

    Ok(jobs)
}
