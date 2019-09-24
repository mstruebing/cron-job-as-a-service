#[derive(Debug)]
pub struct Secret {
    key: &'static str,
    value: &'static str,
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl Clone for Secret {
    fn clone(&self) -> Self {
        Secret {
            key: self.key,
            value: self.value,
        }
    }
}

impl Secret {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        Secret { key, value }
    }

    pub fn show(&self) {
        println!("{}: {}", self.key, self.value)
    }

    pub fn drop_table() -> &'static str {
        "DROP TABLE IF EXISTS secrets;"
    }
    pub fn create_table() -> &'static str {
        "CREATE TABLE secrets (
            id SERIAL PRIMARY KEY NOT NULL,
            job_id INTEGER REFERENCES jobs(id),
            key TEXT NOT NULL,
            value TEXT NOT NULL
            );"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let secret = Secret::new("hello", "world");
        assert_eq!(secret.key, "hello");
        assert_eq!(secret.value, "world");
    }
}
