//! `errorium` macro definitions crate, with all corresponding procedural macros
//! implementation.

mod error;
mod tags_macro;
mod utils;

/// Generates  for each error tag a new type which could be built from
/// any `Error` object, with the `handle` function.
#[proc_macro]
pub fn tags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tags_macro::generate(input)
}
