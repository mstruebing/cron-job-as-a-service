use postgres::Error;

use crate::database;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn save_new(job_id: i32, secrets: Vec<Secret>) -> Result<Vec<Secret>, Error> {
        if secrets.is_empty() {
            return Ok(secrets);
        }

        let mut secrets = secrets.clone();
        let mut query: String = "INSERT INTO secrets (job_id, key, value) VALUES ".to_owned();

        for (index, secret) in secrets.iter().enumerate() {
            let secret_values: &str =
                &format!("({}, '{}', '{}')", job_id, secret.key, secret.value);

            if index == 0 {
                query.push_str(secret_values);
            } else {
                query.push_str(", ");
                query.push_str(secret_values);
            }

            if index == secrets.len() - 1 {
                query.push_str(" RETURNING id;");
            }
        }

        let connection = database::connection();
        let rows = &connection.query(&query, &[])?;

        for (index, row) in rows.iter().enumerate() {
            let id: i32 = row.get(0);
            secrets[index].id = Some(id);
        }

        Ok(secrets)
    }

    pub fn update(job_id: i32, secrets: Vec<Secret>) -> Result<Vec<Secret>, Error> {
        if secrets.is_empty() {
            return Ok(secrets);
        }

        for secret in secrets.clone() {
            let connection = database::connection();

            let query = "UPDATE secrets SET job_id = $1, key = $2, value = $3 WHERE id = $4;";
            connection.execute(
                query,
                &[&job_id, &secret.key, &secret.value, &secret.id.unwrap()],
            )?;
        }

        Ok(secrets)
    }

    pub fn delete(self) -> Result<(), Error> {
        match self.id {
            Some(id) => {
                let connection = database::connection();
                connection.execute("DELETE FROM secrets WHERE id = $1", &[&id])?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn save(job_id: i32, secrets: Vec<Secret>) -> Result<Vec<Secret>, Error> {
        if secrets.is_empty() {
            return Ok(secrets);
        }

        let mut new_secrets: Vec<Secret> = Vec::with_capacity(secrets.len());
        let mut existing_secrets: Vec<Secret> = Vec::with_capacity(secrets.len());

        for job in secrets {
            match job.id {
                Some(_) => existing_secrets.push(job),
                None => new_secrets.push(job),
            }
        }

        let new_secrets = Secret::save_new(job_id, new_secrets)?;
        let existing_secrets = Secret::update(job_id, existing_secrets)?;

        let mut concat = Vec::with_capacity(new_secrets.len() + existing_secrets.len());
        concat.extend(new_secrets);
        concat.extend(existing_secrets);

        Ok(concat)
    }

    pub fn get_as_string(secrets: Vec<Secret>) -> String {
        let mut secret_string: String = String::from("");
        for secret in secrets {
            secret_string = format!("{}={} ", secret.key, secret.value);
        }

        secret_string
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
}
