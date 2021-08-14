use crate::error::Error;

/// Error thrown while fetching data from pulseaudio,
/// has two variants: PulseCtlError for when PulseAudio returns an error code
/// and GetInfoError when a request for data fails for whatever reason
#[derive(Debug, Clone)]
pub enum ControllerError {
    PulseCtl(String),
    GetInfo(&'static str),
}

impl std::fmt::Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PulseCtl(e) => f.write_str(&format!("Pulse ctl error: {}", e)),
            Self::GetInfo(e) => f.write_str(&format!("Get info error: {}", e)),
        }
    }
}

impl std::error::Error for ControllerError {}

impl From<Error> for ControllerError {
    fn from(error: Error) -> Self {
        Self::PulseCtl(error.to_string())
    }
}
