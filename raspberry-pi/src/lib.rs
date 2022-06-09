#![cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_os = "linux"))]

/// Wrapper code for the c shared library files
mod wrapper;
pub use self::wrapper::RaspberryPi;
use std::os::raw::c_int;

/// The error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to load a library
    #[error("failed to load `{name}`")]
    LibraryLoad {
        /// The library name
        name: &'static str,
        #[source]
        error: libloading::Error,
    },

    /// bcm_host is not initialized
    #[error("bcm_host is not initialized")]
    BcmHostNotInitialized,

    /// A board type was unknown
    #[error("the board type `{0}` was unknown")]
    UnknownBoardType(c_int),

    /// `graphics_get_display_size` failed with an error code
    #[error("`graphics_get_display_size` failed with error code `{0}`")]
    GraphicsGetDisplaySize(i32),

    /// A processor id was unknown
    #[error("the processor id `{0}` is unknown")]
    UnknownProcessorId(c_int),

    /// Failed to conver to CString
    #[error(transparent)]
    InteriorNul(#[from] std::ffi::NulError),

    /// A vc_gencmd error
    #[error("a vc gen cmd function failed with error code`{0}`")]
    VcGenCmd(c_int),

    /// A VCos Error
    #[error("a vcos function failed with error code `{0}`")]
    VCos(raspberry_pi_sys::libbcm_host::VCOS_STATUS_T),
}

// The following definitons are copied from `rust-raspberry-pi`.
// Consider the creation of a `raspberry-pi`.

/// The board type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BoardType {
    ModelA = 0x00,
    ModelB = 0x01,
    ModelAPlus = 0x02,
    ModelBPlus = 0x03,
    Pi2ModelB = 0x04,
    Alpha = 0x05,
    Cm = 0x06,
    Cm2 = 0x07,
    Pi3ModelB = 0x08,
    Pi0 = 0x09,
    Cm3 = 0x0A,
    Custom = 0x0B,
    Pi0W = 0x0C,
    Pi3ModelBPlus = 0x0D,
    Pi3ModelAPlus = 0x0E,
    Fpga = 0x0F,
    Cm3Plus = 0x10,
    Pi4ModelB = 0x11,
    Pi400 = 0x13,
    Cm4 = 0x14,
}

impl TryFrom<u8> for BoardType {
    type Error = u8;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0x00 => Ok(Self::ModelA),
            0x01 => Ok(Self::ModelB),
            0x02 => Ok(Self::ModelAPlus),
            0x03 => Ok(Self::ModelBPlus),
            0x04 => Ok(Self::Pi2ModelB),
            0x05 => Ok(Self::Alpha),
            0x06 => Ok(Self::Cm),
            0x07 => Ok(Self::Cm2),
            0x08 => Ok(Self::Pi3ModelB),
            0x09 => Ok(Self::Pi0),
            0x0A => Ok(Self::Cm3),
            0x0B => Ok(Self::Custom),
            0x0C => Ok(Self::Pi0W),
            0x0D => Ok(Self::Pi3ModelBPlus),
            0x0E => Ok(Self::Pi3ModelAPlus),
            0x0F => Ok(Self::Fpga),
            0x10 => Ok(Self::Cm3Plus),
            0x11 => Ok(Self::Pi4ModelB),
            0x13 => Ok(Self::Pi400),
            0x14 => Ok(Self::Cm4),
            _ => Err(n),
        }
    }
}

/// The id of the processor
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ProcessorId {
    Bcm2835 = 0x00,
    Bcm2836 = 0x01,
    Bcm2837 = 0x02,

    /// This is also Bcm2838, which is a deprecated name for this id.
    Bcm2711 = 0x03,
}

impl TryFrom<u8> for ProcessorId {
    type Error = u8;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0x00 => Ok(Self::Bcm2835),
            0x01 => Ok(Self::Bcm2836),
            0x02 => Ok(Self::Bcm2837),
            0x03 => Ok(Self::Bcm2711),
            _ => Err(n),
        }
    }
}
