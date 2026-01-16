//! project_arches library
//!
//! Rust project: project_arches

pub fn hello() -> &'static str {
    "Hello from project_arches!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from project_arches!");
    }
}
