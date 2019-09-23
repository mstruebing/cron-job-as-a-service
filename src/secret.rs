#[derive(Debug)]
pub struct Secret {
    // TODO: Add id
    key: &'static str,
    value: &'static str,
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl Secret {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        Secret { key, value }
    }

    pub fn show(&self) {
        println!("{}: {}", self.key, self.value)
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
