use crate::error::Error;

/// Error thrown while fetching data from pulseaudio.
#[derive(Debug, Clone)]
pub enum ControllerError {
    /// When PulseAudio returns an error code
    PulseCtl(String),
    /// When a request for data fails for whatever reason
    GetInfo(String),
}

impl std::fmt::Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PulseCtl(e) => f.write_str(&format!("PulseCtl error: {}", e)),
            Self::GetInfo(e) => f.write_str(&format!("GetInfo error: {}", e)),
        }
    }
}

impl std::error::Error for ControllerError {}

impl From<Error> for ControllerError {
    fn from(error: Error) -> Self {
        Self::PulseCtl(error.to_string())
    }
}
