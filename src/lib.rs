use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};
use uuid::Uuid;

pub mod api_types;
pub mod permissions;
pub mod types;

pub const NAME_ALLOWED_CHARS: &str =
    " .,_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub type ID = Uuid;
pub fn new_id() -> ID {
    uuid::Uuid::new_v4()
}

pub fn is_valid_username(name: &str) -> bool {
    name.len() > 0 && name.len() < 32 && name.chars().all(|c| NAME_ALLOWED_CHARS.contains(c))
}

pub fn get_system_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn uuid_from_num_string(string: &str) -> Result<Uuid, ()> {
    if let Ok(num) = string.parse::<u128>() {
        Ok(Uuid::from_u128(num))
    } else {
        Err(())
    }
}

pub fn get_id(id: &str, service: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(id);
    hasher.update(service);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::is_valid_username;

    #[test]
    fn valid_username_check() {
        assert!(is_valid_username("a"));
        assert!(is_valid_username("Test_test tHAt-tester."));
        assert!(is_valid_username("1234567890"));
        assert!(is_valid_username("l33t 5p34k"));
        assert!(!is_valid_username(""));
        assert!(!is_valid_username("Test! @thing"));
        assert!(!is_valid_username("123456789111315171921232527293133"));
    }
}
