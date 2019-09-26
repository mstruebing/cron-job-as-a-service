use postgres::Error;
use std::process::Command;

use crate::database;
use crate::secret::Secret;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: Option<i32>,
    pub schedule: &'static str,
    pub command: &'static str,
    pub last_run: i32,
    pub next_run: i32,
    secrets: Vec<Secret>,
}

impl Job {
    pub fn new(
        id: Option<i32>,
        schedule: &'static str,
        command: &'static str,
        last_run: i32,
        next_run: i32,
        secrets: Vec<Secret>,
    ) -> Self {
        Job {
            id,
            schedule,
            command,
            last_run,
            next_run,
            secrets,
        }
    }

    pub fn drop_table() -> &'static str {
        "DROP TABLE IF EXISTS jobs;"
    }
    pub fn create_table() -> &'static str {
        "CREATE TABLE jobs (
            id SERIAL PRIMARY KEY NOT NULL,
            user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL ,
            schedule TEXT NOT NULL,
            command TEXT NOT NULL,
            last_run INTEGER,
            next_run INTEGER NOT NULL
            );"
    }

    pub fn save_new(user_id: i32, jobs: Vec<Job>) -> Result<Vec<Job>, Error> {
        if jobs.is_empty() {
            return Ok(jobs);
        }

        let mut jobs = jobs.clone();
        let mut query: String =
            "INSERT INTO jobs (user_id, schedule, command, last_run, next_run) VALUES ".to_owned();

        for (index, job) in jobs.iter().enumerate() {
            let job_values: &str = &format!(
                "({}, '{}', '{}', {}, {})",
                user_id, job.schedule, job.command, job.last_run, job.next_run,
            );

            if index == 0 {
                query.push_str(job_values);
            } else {
                query.push_str(", ");
                query.push_str(job_values);
            }

            if index == jobs.len() - 1 {
                query.push_str(" RETURNING id;");
            }
        }

        let connection = database::connection();
        let rows = &connection.query(&query, &[])?;

        for (index, row) in rows.iter().enumerate() {
            let id: i32 = row.get(0);
            jobs[index].id = Some(id);

            Secret::save(jobs[index].id.unwrap(), jobs[index].secrets.clone())?;
        }

        Ok(jobs)
    }

    pub fn update(user_id: i32, jobs: Vec<Job>) -> Result<Vec<Job>, Error> {
        if jobs.is_empty() {
            return Ok(jobs);
        }

        for job in jobs.clone() {
            let query = "UPDATE jobs SET user_id = $1, schedule = $2, command = $3, last_run = $4, next_run = $5 WHERE id = $6;";

            let connection = database::connection();
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

            // TODO: n+1
            Secret::save(job.id.unwrap(), job.secrets.clone())?;
        }

        Ok(jobs)
    }

    pub fn save(user_id: i32, jobs: Vec<Job>) -> Result<Vec<Job>, Error> {
        if jobs.is_empty() {
            return Ok(jobs);
        }

        let mut new_jobs: Vec<Job> = Vec::with_capacity(jobs.len());
        let mut existing_jobs: Vec<Job> = Vec::with_capacity(jobs.len());

        for job in jobs {
            match job.id {
                Some(_) => existing_jobs.push(job),
                None => new_jobs.push(job),
            }
        }

        let new_jobs = Job::save_new(user_id, new_jobs)?;
        let existing_jobs = Job::update(user_id, existing_jobs)?;

        let mut concat = Vec::with_capacity(new_jobs.len() + existing_jobs.len());
        concat.extend(new_jobs);
        concat.extend(existing_jobs);

        Ok(concat)
    }

    pub fn delete(self) -> Result<(), Error> {
        match self.id {
            Some(id) => {
                let connection = database::connection();
                connection.execute("DELETE FROM jobs WHERE id = $1", &[&id])?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn execute(self) -> Result<(), Error> {
        // TODO: use alpine docker container
        let args = format!("{}; {}", Secret::get_as_string(self.secrets), self.command);
        Command::new("sh").arg("-c").arg(args).output()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let job = Job::new(
            None,
            "0 * * * *",
            "echo $hello",
            0,
            1,
            vec![Secret::new(None, "hello", "world")],
        );

        assert_eq!(job.schedule, "0 * * * *");
        assert_eq!(job.command, "echo $hello");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 1);
        assert_eq!(job.secrets, vec![Secret::new(None, "hello", "world")]);
    }
}
