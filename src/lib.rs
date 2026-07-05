pub fn greeting(name: &str) -> String {
    format!("Hello, {name}. This is the Rust template.")
}

pub fn run(name: Option<&str>) -> String {
    greeting(name.unwrap_or("world"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_uses_default_name() {
        assert_eq!(run(None), "Hello, world. This is the Rust template.");
    }

    #[test]
    fn greeting_uses_custom_name() {
        assert_eq!(greeting("Ada"), "Hello, Ada. This is the Rust template.");
    }
}
