use crate::secret::Secret;

#[derive(Debug)]
pub struct Job {
    schedule: &'static str,
    pub command: &'static str,
    last_run: u64,
    next_run: u64,
    secrets: Vec<Secret>,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.schedule == other.schedule
            && self.command == other.command
            && self.last_run == other.last_run
            && self.next_run == other.next_run
            && self.secrets == other.secrets
    }
}

impl Clone for Job {
    fn clone(&self) -> Self {
        Job {
            schedule: self.schedule,
            command: self.command,
            last_run: self.last_run,
            next_run: self.next_run,
            secrets: self.secrets.clone(),
        }
    }
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
            schedule,
            command,
            last_run,
            next_run,
            secrets,
        }
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
