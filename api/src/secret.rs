use postgres::Error;

use crate::database;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Secret {
    pub id: Option<i32>,
    pub key: &'static str,
    pub value: &'static str,
}

impl Secret {
    pub fn new(id: Option<i32>, key: &'static str, value: &'static str) -> Self {
        Secret { id, key, value }
    }

    pub fn drop_table() -> &'static str {
        "DROP TABLE IF EXISTS secrets;"
    }
    pub fn create_table() -> &'static str {
        "CREATE TABLE secrets (
            id SERIAL PRIMARY KEY NOT NULL,
            job_id INTEGER REFERENCES jobs(id) ON DELETE CASCADE,
            key TEXT NOT NULL,
            value TEXT NOT NULL
            );"
    }

    pub fn save_new(mut self, job_id: i32) -> Result<Self, Error> {
        let connection = database::connection()?;
        let query = "INSERT INTO secrets (job_id, key, value) VALUES ($1, $2, $3) RETURNING id";
        let rows = connection.query(query, &[&job_id, &self.key, &self.value])?;

        for row in rows.iter() {
            let id: i32 = row.get(0);
            self.id = Some(id);
        }

        Ok(self)
    }

    pub fn update(self, job_id: i32) -> Result<Self, Error> {
        let connection = database::connection()?;

        let query = "UPDATE secrets SET job_id = $1, key = $2, value = $3 WHERE id = $4;";
        connection.execute(query, &[&job_id, &self.key, &self.value, &self.id.unwrap()])?;

        Ok(self)
    }

    pub fn save(self, job_id: i32) -> Result<Self, Error> {
        match self.id {
            Some(_) => self.update(job_id),
            None => self.save_new(job_id),
        }
    }

    pub fn delete(self) -> Result<(), Error> {
        match self.id {
            Some(id) => {
                let connection = database::connection()?;
                connection.execute("DELETE FROM secrets WHERE id = $1", &[&id])?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn get_as_string(self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let secret = Secret::new(None, "hello", "world");
        assert_eq!(secret.key, "hello");
        assert_eq!(secret.value, "world");
    }

    #[test]
    fn test_get_as_string() {
        let secret = Secret::new(None, "hello", "world");
        assert_eq!(secret.get_as_string(), "hello=world");
    }
}
