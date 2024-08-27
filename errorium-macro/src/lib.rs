//! `errorium` macro definitions crate, with all corresponding procedural macros
//! implementation.

use proc_macro::TokenStream;

mod error;
mod errors_macro;
mod tags_macro;
mod utils;

/// Generates for each error tag a new type which could be built from
/// any `Error` object, with the `handle` function.
#[proc_macro]
pub fn tags(input: TokenStream) -> TokenStream {
    tags_macro::generate(input)
}

/// Something
#[proc_macro_attribute]
pub fn errors(attr: TokenStream, input: TokenStream) -> TokenStream {
    errors_macro::generate(attr, input)
}
