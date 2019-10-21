// // stdlib
use std::process::Command;

// modules
use chan::chan_select;
use diesel::prelude::*;

// // Own modules
use shared::database;
use shared::error::Result;
use shared::logger::log;
use shared::models::job::Job;
use shared::schema::jobs;
use shared::utils;

fn main() {
    // TODO: is this a good tick interval?
    let tick = chan::tick_ms(1000);

    loop {
        chan_select! {
            tick.recv() => {
                let current_timestamp = utils::get_current_timestamp();
                let jobs = get_jobs_to_run(current_timestamp);
                for job in jobs {
                    if job.next_run == current_timestamp {
                        let result = run(job);
                        match result {
                            Ok(job) => log(&format!("Successyull ran job: {:?}", job)),
                            Err(err) => log(&format!("ERROR: {:?}", err)),
                        }
                    }
                }
            },
        }
    }
}

pub fn run(job: Job) -> Result<Job> {
    let secrets = job.secrets();
    let args = if !secrets.is_empty() {
        let args = secrets
            .iter()
            .fold(String::new(), |acc, next| acc + " " + &next.get_as_string());
        format!("{}; {}", args, job.command)
    } else {
        job.command.to_string()
    };

    Command::new("sh").arg("-c").arg(args).spawn()?;

    let job = job.last_run(utils::get_current_timestamp());
    let next_run = utils::get_next_run(&job.schedule);
    let job = job.next_run(next_run);
    let connection = database::establish_connection();
    diesel::update(jobs::dsl::jobs.find(job.id))
        .set(&job)
        .execute(&connection)
        .expect("Error updating job");

    Ok(job)
}

fn get_jobs_to_run(current_timestamp: i32) -> Vec<Job> {
    let connection = database::establish_connection();

    let jobs = jobs::dsl::jobs
        .filter(jobs::dsl::next_run.eq(current_timestamp))
        .load::<Job>(&connection)
        .expect("Error loading jobs");

    jobs
}
