use postgres::Error;
use std::process::Command;

use crate::database;
use crate::secret::Secret;

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

    pub fn drop_table() -> &'static str {
        "DROP TABLE IF EXISTS jobs;"
    }
    pub fn create_table() -> &'static str {
        "CREATE TABLE jobs (
            id SERIAL PRIMARY KEY NOT NULL,
            user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL ,
            schedule TEXT NOT NULL,
            command TEXT NOT NULL,
            last_run INTEGER,
            next_run INTEGER NOT NULL
            );"
    }

    pub fn save_new(mut self, user_id: i32) -> Result<Self, Error> {
        let connection = database::connection()?;
        let query =
            "INSERT INTO jobs (user_id, schedule, command, last_run, next_run) VALUES ($1, $2, $3, $4, $5) RETURNING id;";
        let rows = connection.query(
            query,
            &[
                &user_id,
                &self.schedule,
                &self.command,
                &self.last_run,
                &self.next_run,
            ],
        )?;

        for row in rows.iter() {
            let id: i32 = row.get(0);
            self.id = Some(id);
        }

        for (index, secret) in self.secrets.clone().iter().enumerate() {
            self.secrets[index] = secret.save(self.id.unwrap())?;
        }

        Ok(self)
    }

    pub fn update(mut self, user_id: i32) -> Result<Self, Error> {
        let connection = database::connection()?;
        let query = "UPDATE jobs SET user_id = $1, schedule = $2, command = $3, last_run = $4, next_run = $5 WHERE id = $6;";
        connection.execute(
            query,
            &[
                &user_id,
                &self.schedule,
                &self.command,
                &self.last_run,
                &self.next_run,
                &self.id.unwrap(),
            ],
        )?;

        for (index, secret) in self.secrets.clone().iter().enumerate() {
            self.secrets[index] = secret.save(self.id.unwrap())?;
        }

        Ok(self)
    }

    pub fn save(self, user_id: i32) -> Result<Self, Error> {
        match self.id {
            Some(_) => self.update(user_id),
            None => self.save_new(user_id),
        }
    }

    pub fn delete(self) -> Result<(), Error> {
        match self.id {
            Some(id) => {
                let connection = database::connection()?;
                connection.execute("DELETE FROM jobs WHERE id = $1", &[&id])?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn execute(self) -> Result<(), Error> {
        // TODO: use alpine docker container
        let mut args: String = "".to_owned();

        for secret in self.secrets {
            args = args + &secret.get_as_string();
        }

        let args = format!("{}; {}", args, self.command);
        Command::new("sh").arg("-c").arg(args).output()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let job = Job::new(
            None,
            "0 * * * *",
            "echo $hello",
            0,
            1,
            vec![Secret::new(None, "hello", "world")],
        );

        assert_eq!(job.schedule, "0 * * * *");
        assert_eq!(job.command, "echo $hello");
        assert_eq!(job.last_run, 0);
        assert_eq!(job.next_run, 1);
        assert_eq!(job.secrets, vec![Secret::new(None, "hello", "world")]);
    }
}
