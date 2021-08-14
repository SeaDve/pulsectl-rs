use pulse::error::PAErr;

use crate::controllers::error::ControllerError;

/// Error thrown when PulseAudio throws an error code.
#[derive(Debug, Clone)]
pub enum Error {
    /// An error that may occur while establishing a connection.
    Connect(String),
    /// The requested operation is cancelled or quits unexpectedly.
    Operation(String),
    /// PulseAudio returns an error code in any circumstance.
    PulseAudio(String),
    /// A problem occurs while fetching data from pulseaudio.
    Controller(ControllerError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(e) => f.write_str(&format!("ConnectError: {}", e)),
            Self::Operation(e) => f.write_str(&format!("OperationError: {}", e)),
            Self::PulseAudio(e) => f.write_str(&format!("PulseAudioError: {}", e)),
            Self::Controller(e) => f.write_str(&format!("ControllerError: {}", e)),
        }
    }
}

impl From<PAErr> for Error {
    fn from(error: PAErr) -> Self {
        Self::PulseAudio(
            error
                .to_string()
                .unwrap_or_else(|| "Unknown error".to_string()),
        )
    }
}
