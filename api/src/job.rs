// own modules
use shared::database;
use shared::error::Result;
use shared::model::job::Job;

// internal
use crate::secret;

pub fn save(mut job: Job, user_id: i32) -> Result<Job> {
    let connection = database::connection()?;
    let query =
            "INSERT INTO jobs (user_id, schedule, command, last_run, next_run) VALUES ($1, $2, $3, $4, $5) RETURNING id;";
    let rows = connection.query(
        query,
        &[
            &user_id,
            &job.schedule,
            &job.command,
            &job.last_run,
            &job.next_run,
        ],
    )?;

    for row in rows.iter() {
        let id: i32 = row.get(0);
        job.id = Some(id);
    }

    for (index, secret) in job.secrets.clone().iter().enumerate() {
        job.secrets[index] = secret::save(secret.clone(), job.id.unwrap())?;
    }

    Ok(job)
}

pub fn update(mut job: Job, user_id: i32) -> Result<Job> {
    let connection = database::connection()?;
    let query = "UPDATE jobs SET user_id = $1, schedule = $2, command = $3, last_run = $4, next_run = $5 WHERE id = $6;";
    connection.execute(
        query,
        &[
            &user_id,
            &job.schedule,
            &job.command,
            &job.last_run,
            &job.next_run,
            &job.id.unwrap(),
        ],
    )?;

    for (index, secret) in job.secrets.clone().iter().enumerate() {
        job.secrets[index] = secret::save(secret.clone(), job.id.unwrap())?;
    }

    Ok(job)
}

pub fn delete(job: Job) -> Result<()> {
    match job.id {
        Some(id) => {
            let connection = database::connection()?;
            connection.execute("DELETE FROM jobs WHERE id = $1", &[&id])?;
            Ok(())
        }
        None => Ok(()),
    }
}
