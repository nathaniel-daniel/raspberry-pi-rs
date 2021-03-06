#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::useless_transmute)]
// WARNING: Fix this ASAP.
//
// This appears to be caused by bindgen-created tests.
// Check to see if the deref actually exists, then work with upstream to fix it.
#![allow(deref_nullptr)]

#[cfg(all(target_arch = "arm", target_os = "linux"))]
mod arm_bindings;
#[cfg(all(target_arch = "arm", target_os = "linux"))]
pub use self::arm_bindings::*;

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
mod arm64_bindings;
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub use self::arm64_bindings::*;
