use postgres::Error;

use crate::database;
use crate::secret::Secret;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: Option<i32>,
    pub schedule: &'static str,
    pub command: &'static str,
    pub last_run: u64,
    pub next_run: u64,
    secrets: Vec<Secret>,
}

impl Job {
    pub fn new(
        schedule: &'static str,
        command: &'static str,
        last_run: u64,
        next_run: u64,
        secrets: Vec<Secret>,
    ) -> Self {
        Job {
            id: None,
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
            user_id INTEGER REFERENCES users(id) NOT NULL,
            schedule TEXT NOT NULL,
            command TEXT NOT NULL,
            last_run INTEGER,
            next_run INTEGER NOT NULL
            );"
    }

    fn save_new(user_id: i32, jobs: Vec<Job>) -> Result<Vec<Job>, Error> {
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
        }

        Ok(jobs)
    }

    // TODO: Implement
    fn update(_user_id: i32, jobs: Vec<Job>) -> Result<Vec<Job>, Error> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let job = Job::new(
            "0 * * * *",
            "echo $hello",
            0,
            1,
            vec![Secret::new("hello", "world")],
        );

        assert_eq!(job.schedule, "0 * * * *");
        assert_eq!(job.command, "echo $hello");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 1);
        assert_eq!(job.secrets, vec![Secret::new("hello", "world")]);
    }
}
