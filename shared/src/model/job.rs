// internal
use crate::model::secret::Secret;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Job {
    pub id: Option<i32>,
    pub schedule: &'static str,
    pub command: &'static str,
    pub last_run: i32,
    pub next_run: i32,
    pub secrets: Vec<Secret>,
}

impl Job {
    pub fn new() -> Self {
        Job::default()
    }

    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    pub fn schedule(mut self, schedule: &'static str) -> Self {
        self.schedule = schedule;
        self
    }

    pub fn command(mut self, command: &'static str) -> Self {
        self.command = command;
        self
    }

    pub fn last_run(mut self, last_run: i32) -> Self {
        self.last_run = last_run;
        self
    }

    pub fn next_run(mut self, next_run: i32) -> Self {
        self.next_run = next_run;
        self
    }

    pub fn secrets(mut self, secrets: Vec<Secret>) -> Self {
        self.secrets = secrets;
        self
    }

    pub fn add_secret(mut self, secret: Secret) -> Self {
        self.secrets.push(secret);
        self
    }

    pub fn remove_secret(mut self, secret: Secret) -> Self {
        let index = self.secrets.iter().position(|x| *x == secret).unwrap();
        self.secrets.remove(index);
        self
    }

    pub fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS jobs;"
    }
    pub fn create_table_query() -> &'static str {
        "CREATE TABLE jobs (
            id SERIAL PRIMARY KEY NOT NULL,
            user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL ,
            schedule TEXT NOT NULL,
            command TEXT NOT NULL,
            last_run INTEGER,
            next_run INTEGER NOT NULL
            );"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let job = Job::new();

        assert_eq!(job.id, None);
        assert_eq!(job.schedule, "");
        assert_eq!(job.command, "");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 0);
        assert_eq!(job.secrets, vec![]);

        let job = job.id(Some(3));
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "");
        assert_eq!(job.command, "");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 0);
        assert_eq!(job.secrets, vec![]);

        let job = job.schedule("abc");
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 0);
        assert_eq!(job.secrets, vec![]);

        let job = job.command("def");
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "def");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 0);
        assert_eq!(job.secrets, vec![]);

        let job = job.last_run(1);
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "def");
        assert_eq!(job.last_run, 1);
        assert_eq!(job.next_run, 0);
        assert_eq!(job.secrets, vec![]);

        let job = job.next_run(2);
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "def");
        assert_eq!(job.last_run, 1);
        assert_eq!(job.next_run, 2);
        assert_eq!(job.secrets, vec![]);

        let job = job.secrets(vec![Secret::new().id(None), Secret::new().id(Some(1))]);
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "def");
        assert_eq!(job.last_run, 1);
        assert_eq!(job.next_run, 2);
        assert_eq!(job.secrets.len(), 2);
        assert_eq!(job.secrets[0], Secret::new().id(None));
        assert_eq!(job.secrets[1], Secret::new().id(Some(1)));

        let job = job.remove_secret(Secret::new().id(None));
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "def");
        assert_eq!(job.last_run, 1);
        assert_eq!(job.next_run, 2);
        assert_eq!(job.secrets.len(), 1);
        assert_eq!(job.secrets[0], Secret::new().id(Some(1)));

        let job = job.add_secret(Secret::new().id(None));
        assert_eq!(job.id, Some(3));
        assert_eq!(job.schedule, "abc");
        assert_eq!(job.command, "def");
        assert_eq!(job.last_run, 1);
        assert_eq!(job.next_run, 2);
        assert_eq!(job.secrets.len(), 2);
        assert_eq!(job.secrets[0], Secret::new().id(Some(1)));
        assert_eq!(job.secrets[1], Secret::new().id(None));
    }
}
