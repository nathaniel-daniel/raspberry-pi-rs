#![cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_os = "linux"))]

/// Ports of the `bcm_host_*` family of functions.
pub mod bcm_host;

/// The library error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Parse int error
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// Something was not found
    #[error("not found")]
    NotFound,
}
