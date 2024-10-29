extern crate custom_macros;

use custom_macros::BuiderPattern;

#[derive(BuiderPattern)]
pub struct User {
    #[allow(dead_code)]
    username: String,
    #[allow(dead_code)]
    first_name: String,
    #[allow(dead_code)]
    last_name: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_builder() {
        // Create a User using the builder pattern
        let user = User::builder()
            .username("alice@example.com".into())
            .first_name("etienne".into())
            .last_name("tonie".into())
            .build()
            .unwrap();

        let u = format!("User: {}, {}, {}", user.username, user.first_name, user.last_name);
        assert_eq!(u, "User: alice@example.com, etienne, tonie");
    }
}