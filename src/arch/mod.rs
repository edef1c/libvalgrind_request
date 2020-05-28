pub use self::imp::{do_client_request, Value};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod imp;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod imp;

#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod imp;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod imp;
