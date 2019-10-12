// internal
use crate::model::job::Job;

#[derive(Debug, Default)]
pub struct User {
    pub id: Option<i32>,
    pub email: &'static str,
    pub password: &'static str,
    pub jobs: Vec<Job>,
}

impl User {
    pub fn new() -> Self {
        User::default()
    }

    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    pub fn email(mut self, email: &'static str) -> Self {
        self.email = email;
        self
    }

    pub fn password(mut self, password: &'static str) -> Self {
        self.password = password;
        self
    }

    pub fn jobs(mut self, jobs: Vec<Job>) -> Self {
        self.jobs = jobs;
        self
    }

    pub fn add_job(mut self, job: Job) -> Self {
        self.jobs.push(job);
        self
    }

    pub fn remove_job(mut self, job: Job) -> Self {
        let index = self.jobs.iter().position(|x| *x == job).unwrap();
        self.jobs.remove(index);
        self
    }

    pub fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS users;"
    }

    pub fn create_table_query() -> &'static str {
        "CREATE TABLE users (
            id SERIAL PRIMARY KEY NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
            );"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let user = User::new();
        assert_eq!(user.id, None);
        assert_eq!(user.email, "");
        assert_eq!(user.password, "");
        assert_eq!(user.jobs, vec![]);

        let user = user.id(Some(3));
        assert_eq!(user.id, Some(3));
        assert_eq!(user.email, "");
        assert_eq!(user.password, "");
        assert_eq!(user.jobs, vec![]);

        let user = user.email("someone@example.com");
        assert_eq!(user.id, Some(3));
        assert_eq!(user.email, "someone@example.com");
        assert_eq!(user.password, "");
        assert_eq!(user.jobs, vec![]);

        let user = user.password("pa$$");
        assert_eq!(user.id, Some(3));
        assert_eq!(user.email, "someone@example.com");
        assert_eq!(user.password, "pa$$");
        assert_eq!(user.jobs, vec![]);

        let user = user.jobs(vec![Job::new().id(None), Job::new().id(Some(1))]);
        assert_eq!(user.id, Some(3));
        assert_eq!(user.email, "someone@example.com");
        assert_eq!(user.password, "pa$$");
        assert_eq!(user.jobs.len(), 2);
        assert_eq!(user.jobs[0], Job::new().id(None));
        assert_eq!(user.jobs[1], Job::new().id(Some(1)));

        let user = user.remove_job(Job::new().id(None));
        assert_eq!(user.id, Some(3));
        assert_eq!(user.email, "someone@example.com");
        assert_eq!(user.password, "pa$$");
        assert_eq!(user.jobs.len(), 1);
        assert_eq!(user.jobs[0], Job::new().id(Some(1)));

        let user = user.add_job(Job::new().id(None));
        assert_eq!(user.id, Some(3));
        assert_eq!(user.email, "someone@example.com");
        assert_eq!(user.password, "pa$$");
        assert_eq!(user.jobs.len(), 2);
        assert_eq!(user.jobs[0], Job::new().id(Some(1)));
        assert_eq!(user.jobs[1], Job::new().id(None));
    }
}
