#![doc = include_str!("../README.md")]

#[macro_use]
extern crate bitflags;

/// Components supported by this crate are listed here
pub mod component;
/// Helper functions and types are listed here, for example [util::Color] and the [util::include_cdn] function.
pub mod util;
/// Access to bootstrap-icons.
pub mod icons;
