//! JSON Protocol Structs and representations for communication with
//! authenticators and clients.

pub mod attest;
pub mod auth;
pub mod cose;
pub mod extensions;
pub mod options;

pub use attest::*;
pub use auth::*;
pub use cose::*;
pub use extensions::*;
pub use options::*;
