//! `errorium` macro definitions crate, with all corresponding procedural macros
//! implementation.

use proc_macro::TokenStream;

mod error;
mod tags_macro;
mod utils;

/// Generates for each error tag a new type which could be built from
/// any `Error` object, with the `handle` function.
#[proc_macro]
pub fn tags(input: TokenStream) -> TokenStream {
    tags_macro::generate(input)
}
