// internal
use crate::database;
use crate::error::Result;
use crate::model::job::Job;
use crate::repository::secret;

pub fn save(mut job: Job, user_id: i32) -> Result<Job> {
    let connection = database::connection()?;

    for row in &connection.query(
        "INSERT INTO jobs (user_id, schedule, command, last_run, next_run)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;",
        &[
            &user_id,
            &job.schedule,
            &job.command,
            &job.last_run,
            &job.next_run,
        ],
    )? {
        let id: i32 = row.get(0);
        job.id = Some(id);
    }

    for (index, secret) in job.secrets.clone().iter().enumerate() {
        job.secrets[index] = secret::save(secret.clone(), job.id.unwrap())?;
    }

    Ok(job)
}

pub fn update(mut job: Job) -> Result<Job> {
    let connection = database::connection()?;

    connection.execute(
        "UPDATE jobs SET schedule = $1, command = $2, last_run = $3, next_run = $4 WHERE id = $5;",
        &[
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
