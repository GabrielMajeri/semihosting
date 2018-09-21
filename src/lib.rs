//! Support for ARM [semihosting].
//!
//! Semihosting is a setup where the ARM bare-metal target is able to call into
//! the host computer's environment.
//!
//! [semihosting]: https://developer.arm.com/docs/100863/latest

#![warn(missing_docs)]
#![feature(asm)]
#![no_std]

mod call;
pub use self::call::*;

mod error;
pub use self::error::*;

pub mod arm;
