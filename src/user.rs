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
        let job = Job::new("0 * * * *", "echo $hello", 0, 1, true, secrets);
        let jobs = vec![job.clone()];
        let user = User::new(email, password, jobs.clone());

        assert_eq!(user.email, email);
        assert_eq!(user.password, password);
        assert_eq!(user.jobs, vec![job]);
    }
}
