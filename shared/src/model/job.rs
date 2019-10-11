use crate::model::secret::Secret;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: Option<i32>,
    pub schedule: &'static str,
    pub command: &'static str,
    pub last_run: i32,
    pub next_run: i32,
    secrets: Vec<Secret>,
}

impl Job {
    pub fn new(
        id: Option<i32>,
        schedule: &'static str,
        command: &'static str,
        last_run: i32,
        next_run: i32,
        secrets: Vec<Secret>,
    ) -> Self {
        Job {
            id,
            schedule,
            command,
            last_run,
            next_run,
            secrets,
        }
    }
}
