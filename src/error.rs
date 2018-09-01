use ffi::RLBotCoreStatus;
use std::error::Error;
use std::fmt;

/// An error code from the RLBot interface.
#[derive(Debug)]
pub struct RLBotError {
    pub status: RLBotCoreStatus,
}

impl Error for RLBotError {}

impl fmt::Display for RLBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RLBotError({:?})", self.status)
    }
}
