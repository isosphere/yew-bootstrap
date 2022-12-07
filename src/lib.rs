//! # The yew-bootstrap Crate
//! 
//! `yew-bootstrap` is a collection of frontend components made to simplify the usage of Bootstraaap 5 in Yew applications.
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! `yew-bootstrap = "^0.5"`
//! 
//! Check `main.rs` which includes an example for every component implemented.
//! 
//! ## Version Convention
//! `0.xy` 
//! Versions that have the same `x` value target the same version of Yew.
//! 
//! There is currently no indication of which version of Bootstrap is targeted, however, we'll try to target the previous release pipeline. For example
//! as of this writing the latest boostrap is 5.2.x, so we'll try to target the latest 5.1.x version.

/// Components supported by this crate are listed here
pub mod component;
/// Helper functions and types are listed here, for example [util::Color] and the [util::include_cdn] function.
pub mod util;
