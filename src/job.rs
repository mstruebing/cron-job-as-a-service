use crate::secret::Secret;

#[derive(Debug)]
pub struct Job {
    // TODO: Add command history
    schedule: &'static str,
    command: &'static str,
    last_run: u64,
    next_run: u64,
    last_run_succeeded: bool,
    secrets: Vec<Secret>,
}

impl Job {
    pub fn new(
        schedule: &'static str,
        command: &'static str,
        last_run: u64,
        next_run: u64,
        last_run_succeeded: bool,
        secrets: Vec<Secret>,
    ) -> Self {
        Job {
            schedule,
            command,
            last_run,
            next_run,
            last_run_succeeded,
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
            true,
            vec![Secret::new("hello", "world")],
        );

        assert_eq!(job.schedule, "0 * * * *");
        assert_eq!(job.command, "echo $hello");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 1);
        assert_eq!(job.last_run_succeeded, true);
        assert_eq!(job.secrets, vec![Secret::new("hello", "world")]);
    }
}
