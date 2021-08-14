use pulse::error::PAErr;

use crate::controllers::error::ControllerError;

/// Error thrown when PulseAudio throws an error code, there are 3 variants
#[derive(Debug, Clone)]
pub enum Error {
    /// When there's an error establishing a connection
    Connect(&'static str),
    /// When the requested operation quis unexpectedly or is cancelled
    Operation(&'static str),
    /// When PulseAudio returns an error code in any circumstance
    PulseAudio(String),
    Controller(ControllerError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(e) => f.write_str(&format!("Connect error: {}", e)),
            Self::Operation(e) => f.write_str(&format!("Operation error: {}", e)),
            Self::PulseAudio(e) => f.write_str(&format!("Pulse audio error: {}", e)),
            Self::Controller(e) => f.write_str(&format!("Controller error: {}", e)),
        }
    }
}

impl From<PAErr> for Error {
    fn from(error: PAErr) -> Self {
        Self::PulseAudio(error.to_string().unwrap_or_else(|| "Unknown".to_string()))
    }
}
