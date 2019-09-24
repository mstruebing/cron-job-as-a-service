use crate::job::Job;

#[derive(Debug)]
pub struct User {
    email: &'static str,
    password: &'static str,
    jobs: Vec<Job>,
}

impl User {
    pub fn new(email: &'static str, password: &'static str, jobs: Vec<Job>) -> Self {
        User {
            email,
            password,
            jobs,
        }
    }

    pub fn add_job(&mut self, job: Job) -> Self {
        self.jobs.push(job.clone());
        self.clone()
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
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
        let user = User::new(email, password, jobs.clone());

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
        let mut user = User::new(email, password, jobs.clone());

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
