//! Multiwii Serial Protocol (MSP) traffic decoder and structures
//!
//! Incomplete. Includes some structures from Cleanflight and Betaflight.

#![cfg_attr(not(feature = "std"), no_std)]

pub use packed_struct;
pub use serde;

mod prelude;

mod commands;
mod packet;
pub mod structs;

pub use commands::*;
pub use packet::*;
