use crate::model::job::Job;

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
