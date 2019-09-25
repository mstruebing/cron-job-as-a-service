use crate::job::Job;

use crate::database;
use postgres::Error;

#[derive(Debug)]
pub struct User {
    id: Option<i32>,
    email: &'static str,
    password: &'static str,
    jobs: Vec<Job>,
}

impl User {
    pub fn new(
        id: Option<i32>,
        email: &'static str,
        password: &'static str,
        jobs: Vec<Job>,
    ) -> Self {
        User {
            id,
            email,
            password,
            jobs,
        }
    }

    pub fn add_job(&mut self, job: Job) -> Self {
        self.jobs.push(job.clone());
        self.clone()
    }

    pub fn drop_table() -> &'static str {
        "DROP TABLE IF EXISTS users;"
    }

    pub fn create_table() -> &'static str {
        "CREATE TABLE users (
            id SERIAL PRIMARY KEY NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
            );"
    }

    pub fn save_new_user(&mut self) -> Result<User, Error> {
        let connection = database::connection();

        for row in &connection.query(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id;",
            &[&self.email, &self.password],
        )? {
            let id: i32 = row.get(0);
            self.id = Some(id);
        }

        if self.jobs.len() > 0 {
            let mut query: String =
                "INSERT INTO jobs (user_id, schedule, command, last_run, next_run) VALUES "
                    .to_owned();
            for (index, job) in self.jobs.iter().enumerate() {
                let job_values: &str = &format!(
                    "({}, '{}', '{}', {}, {})",
                    self.id.unwrap(),
                    job.schedule,
                    job.command,
                    job.last_run,
                    job.next_run,
                );

                if index == 0 {
                    query.push_str(job_values);
                } else {
                    query.push_str(", ");
                    query.push_str(job_values);
                }

                if index == self.jobs.len() - 1 {
                    query.push_str(" RETURNING id;");
                }
            }

            let rows = &connection.query(&query, &[])?;
            for (index, row) in rows.iter().enumerate() {
                let id: i32 = row.get(0);

                self.jobs[index].id = Some(id);
            }
        }

        Ok(self.clone())
    }

    // TODO: Implement
    pub fn update_user(&mut self) -> Result<User, Error> {
        Ok(self.clone())
    }

    pub fn save(&mut self) -> Result<User, Error> {
        match self.id {
            None => return self.save_new_user(),
            Some(_) => return self.update_user(),
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            id: self.id,
            email: self.email,
            password: self.password,
            jobs: self.jobs.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secret::Secret;

    #[test]
    fn test_new() {
        let email = "someone@example.com";
        let password = "pa$$word";

        let secrets = vec![Secret::new("hello", "world")];
        let job = Job::new("0 * * * *", "echo $hello", 0, 1, secrets);
        let jobs = vec![job.clone()];
        let user = User::new(None, email, password, jobs.clone());

        assert_eq!(user.email, email);
        assert_eq!(user.password, password);
        assert_eq!(user.jobs, vec![job]);
    }

    #[test]
    fn test_add_job() {
        let email = "someone@example.com";
        let password = "pa$$word";

        let secrets = vec![Secret::new("hello", "world")];
        let job = Job::new("0 * * * *", "echo $hello", 0, 1, secrets.clone());
        let jobs = vec![job.clone()];
        let mut user = User::new(None, email, password, jobs.clone());

        let job = Job::new(
            "0 * * * *",
            "echo $hello Motherfucker",
            0,
            1,
            secrets.clone(),
        );

        user.add_job(job);

        assert_eq!(user.jobs.len(), 2);
        assert_eq!(user.jobs[0].command, "echo $hello");
        assert_eq!(user.jobs[1].command, "echo $hello Motherfucker");
    }
}
