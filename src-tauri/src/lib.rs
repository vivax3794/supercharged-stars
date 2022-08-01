pub mod api;
pub mod filesystem;

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
