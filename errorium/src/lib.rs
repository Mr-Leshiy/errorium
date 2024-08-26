//! # Errorium
//!
//! This library provides a convinient, type safe error handling functionality to error
//! handle Rust applications.
//!
//! It works on the ground of the [anyhow](https://docs.rs/anyhow/latest/anyhow/)
//! with additional error type safety by providing error "taging".

pub use errorium_macro::*;

/// `Box<dyn std::error::Error>` type alias
pub type Error = Box<dyn std::error::Error>;

/// `Result<T, Box<dyn std::error::Error>>` type alias
///
/// A necessary type alias to maintain and keep correct tagging propagation through the
/// call stack.
/// As `errorium` based on the `Box<dyn std::error::Error>` generic type, this
/// type alias will reduce the boilerplate code in your application.
pub type Result<T> = std::result::Result<T, Error>;
