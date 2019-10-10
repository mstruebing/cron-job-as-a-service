use postgres::Error;

use crate::database;
use crate::job::Job;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub email: &'static str,
    pub password: &'static str,
    pub jobs: Vec<Job>,
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

    pub fn add_job(mut self, job: Job) -> Self {
        self.jobs.push(job);
        self
    }

    pub fn remove_job(mut self, job: Job) -> Result<Self, Error> {
        let mut new_jobs: Vec<Job> = Vec::with_capacity(self.jobs.len() - 1);

        for j in self.jobs {
            if job.id.unwrap() != j.id.unwrap() {
                new_jobs.push(j);
            } else {
                j.delete()?;
            }
        }

        self.jobs = new_jobs;
        Ok(self)
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

    pub fn save_new(mut self) -> Result<User, Error> {
        let connection = database::connection()?;

        for row in &connection.query(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id;",
            &[&self.email, &self.password],
        )? {
            let id: i32 = row.get(0);
            self.id = Some(id);
        }

        for (index, job) in self.jobs.clone().iter().enumerate() {
            self.jobs[index] = job.clone().save(self.id.unwrap())?;
        }

        Ok(self)
    }

    pub fn update(mut self) -> Result<User, Error> {
        let connection = database::connection()?;

        let query = "UPDATE users SET (email, password) = ($1, $2) WHERE id = $3;";
        connection.execute(query, &[&self.email, &self.password, &self.id.unwrap()])?;

        for (index, job) in self.jobs.clone().iter().enumerate() {
            self.jobs[index] = job.clone().save(self.id.unwrap())?;
        }

        Ok(self)
    }

    pub fn save(self) -> Result<User, Error> {
        match self.id {
            None => self.save_new(),
            Some(_) => self.update(),
        }
    }

    pub fn delete(self) -> Result<(), Error> {
        match self.id {
            Some(id) => {
                let connection = database::connection()?;
                connection.execute("DELETE FROM users WHERE id = $1", &[&id])?;
                Ok(())
            }
            None => Ok(()),
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

        let secrets = vec![Secret::new(None, "hello", "world")];
        let job = Job::new(None, "0 * * * *", "echo $hello", 0, 1, secrets);
        let jobs = vec![job.clone()];
        let user = User::new(None, email, password, jobs);

        assert_eq!(user.email, email);
        assert_eq!(user.password, password);
        assert_eq!(user.jobs, vec![job]);
    }

    #[test]
    fn test_add_job() {
        let email = "someone@example.com";
        let password = "pa$$word";

        let jobs = vec![Job::new(None, "0 * * * *", "echo $hello", 0, 1, vec![])];
        let user = User::new(None, email, password, jobs);

        let job = Job::new(None, "0 * * * *", "echo $hello Motherfucker", 0, 1, vec![]);
        let user = user.add_job(job);

        assert_eq!(user.jobs.len(), 2);
        assert_eq!(user.jobs[0].command, "echo $hello");
        assert_eq!(user.jobs[1].command, "echo $hello Motherfucker");
    }
}
