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
