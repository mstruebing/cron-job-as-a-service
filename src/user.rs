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

    pub fn add_job(&mut self, job: Job) -> Self {
        self.jobs.push(job);
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

    fn save_new(&mut self) -> Result<User, Error> {
        let connection = database::connection();

        for row in &connection.query(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id;",
            &[&self.email, &self.password],
        )? {
            let id: i32 = row.get(0);
            self.id = Some(id);
        }

        self.jobs = Job::save(self.id.unwrap(), self.jobs.clone())?;

        Ok(self.clone())
    }

    fn update(&mut self) -> Result<User, Error> {
        let connection = database::connection();

        let query = "UPDATE users SET (email, password) = ($1, $2) WHERE id = $3;";
        connection.execute(query, &[&self.email, &self.password, &self.id.unwrap()])?;

        self.jobs = Job::save(self.id.unwrap(), self.jobs.clone())?;

        Ok(self.clone())
    }

    pub fn save(&mut self) -> Result<User, Error> {
        match self.id {
            None => self.save_new(),
            Some(_) => self.update(),
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
