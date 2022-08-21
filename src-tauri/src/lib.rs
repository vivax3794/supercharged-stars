pub mod api;
pub mod filesystem;
pub mod image;

use serde::{Deserialize, Serialize};

/// Exten result with helpfull tools
pub trait ResultExtended<T> {
    fn prefix_error(self, prefix: &str) -> Result<T, String>;
}

/// Implement ResultExtended trait for Result type!
impl<T, E: core::fmt::Debug> ResultExtended<T> for Result<T, E> {
    fn prefix_error(self, prefix: &str) -> Result<T, String> {
        self.map_err(|err| format!("{}: {:#?}", prefix, err))
    }
}

/// A Specific star, these usually comes in a huge vector
#[derive(Debug, Deserialize, Serialize)]
pub struct Star {
    pub x: f32,
    pub y: f32,

    #[serde(rename = "currentStar")]
    pub color: u8,
}

#[cfg(test)]
mod result_tests {
    use super::*;

    #[test]
    fn prefix_error_returns_ok() {
        let res: Result<i32, String> = Ok(1);
        assert_eq!(res.prefix_error("something"), Ok(1))
    }

    #[test]
    fn prefix_error_returns_err() {
        let res: Result<i32, &str> = Err("went wrong");
        let new = res.prefix_error("oh no");

        assert!(new.is_err());
        let inner = new.unwrap_err();
        assert!(inner.contains("went wrong"));
        assert!(inner.contains("oh no"));
    }
}
