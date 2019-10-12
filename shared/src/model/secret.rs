#[derive(Clone, Debug, Default, PartialEq)]
pub struct Secret {
    pub id: Option<i32>,
    pub key: &'static str,
    pub value: &'static str,
}

impl Secret {
    pub fn new() -> Self {
        Secret::default()
    }

    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = key;
        self
    }

    pub fn value(mut self, value: &'static str) -> Self {
        self.value = value;
        self
    }

    pub fn get_as_string(self) -> String {
        format!("{}={}", self.key, self.value)
    }

    pub fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS secrets;"
    }

    pub fn create_table_query() -> &'static str {
        "CREATE TABLE secrets (
            id SERIAL PRIMARY KEY NOT NULL,
            job_id INTEGER REFERENCES jobs(id) ON DELETE CASCADE,
            key TEXT NOT NULL,
            value TEXT NOT NULL
            );"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let secret = Secret::new();

        assert_eq!(secret.id, None);
        assert_eq!(secret.key, "");
        assert_eq!(secret.value, "");

        let secret = secret.id(Some(2));
        assert_eq!(secret.id, Some(2));
        assert_eq!(secret.key, "");
        assert_eq!(secret.value, "");

        let secret = secret.key("hello");
        assert_eq!(secret.id, Some(2));
        assert_eq!(secret.key, "hello");
        assert_eq!(secret.value, "");

        let secret = secret.value("world");
        assert_eq!(secret.id, Some(2));
        assert_eq!(secret.key, "hello");
        assert_eq!(secret.value, "world");
    }

    #[test]
    fn get_as_string() {
        let secret = Secret::new().key("hello").value("world");
        assert_eq!(secret.get_as_string(), "hello=world");
    }
}
