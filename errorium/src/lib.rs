//! # Errorium
//!
//! This library provides a convinient, type safe error handling functionality to error
//! handle Rust applications.
//!
//! It works on the ground of the [anyhow](https://docs.rs/anyhow/latest/anyhow/)
//! with additional error type safety by providing error "taging".

mod errors;

pub use errorium_macro::*;
pub use errors::Errors;

/// `Box<dyn 'static + std::error::Error + Send + Sync>` type alias,
/// which is a base error type definition for `errorium`.
pub type Error = Box<dyn 'static + std::error::Error + Send + Sync>;

/// `Result<T, Box<dyn std::error::Error>>` type alias
///
/// A necessary type alias to maintain and keep correct tagging propagation through the
/// call stack.
/// As `errorium` based on the `Box<dyn std::error::Error>` generic type, this
/// type alias will reduce the boilerplate code in your application.
pub type Result<T> = std::result::Result<T, Error>;
