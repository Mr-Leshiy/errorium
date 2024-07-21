//! `errorium` macro definitions crate, with all corresponding procedural macros
//! implementation.

mod error;
mod utils;

use error::{Error, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Ident, Visibility,
};
use utils::to_snake_case;

struct ErrorTags(Vec<ErrorTagArgs>);

struct ErrorTagArgs {
    visibility: Visibility,
    ident: Ident,
}

impl Parse for ErrorTags {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tags = Punctuated::<ErrorTagArgs, Comma>::parse_terminated(&input)?
            .into_iter()
            .collect();

        Ok(Self(tags))
    }
}

impl Parse for ErrorTagArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let ident = input.parse()?;

        Ok(Self { visibility, ident })
    }
}

/// Generates a new "main" error type, which is a enumeration of all possible and
/// provided error tags, generates for each error tag a new type which could be built from
/// any `Error` object, the same way as `anyhow::Error` does.
#[proc_macro]
pub fn tags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as ErrorTags);
    generate(args).unwrap_or_else(Into::into).into()
}

fn generate(tags: ErrorTags) -> Result<TokenStream> {
    let ErrorTags(tags) = tags;

    if tags.is_empty() {
        return Err(Error::Other(
            "It should be at least one error tag".to_string(),
        ));
    }

    let error_tags_def = tags
        .iter()
        .map(|tag| generate_error_tag(&tag.visibility, &tag.ident));

    let res = quote! {
        #(#error_tags_def)*
    };
    Ok(res)
}

fn generate_error_tag(visibility: &Visibility, ident: &Ident) -> TokenStream {
    let tag_type_def = quote! {
        #[derive(Debug)]
        #visibility struct #ident(Box<dyn std::error::Error + Send + Sync + 'static>);
    };

    let tag_type_impl_def = quote! {
        impl #ident {
            #visibility fn handle<F>(err: Box<dyn std::error::Error>, handler: F)
            where F: FnOnce(&dyn std::error::Error) {
                if let Some(tag) = err.downcast_ref::<#ident>() {
                    handler(tag.0.as_ref());
                }
            }

            fn tag<T: Into<Box<dyn std::error::Error + Send + Sync + 'static>>>(val: T) -> Self {
                Self(val.into())
            }
        }
    };

    let tag_type_std_traits_impl_def = quote! {

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl std::error::Error for #ident {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.0.source()
            }
        }
    };

    quote! {
        #tag_type_def

        #tag_type_impl_def

        #tag_type_std_traits_impl_def
    }
}

#[allow(dead_code)]
fn generate_main_error(
    main_visibility: &Visibility, main_ident: &Ident, error_tags: &[ErrorTagArgs],
) -> TokenStream {
    let from_defs = generate_main_error_from(main_ident, error_tags);
    let consume_def = generate_main_error_consume(main_visibility, main_ident, error_tags);

    quote! {
        #(#from_defs)*

        #consume_def
    }
}

fn generate_main_error_from(main_ident: &Ident, error_tags: &[ErrorTagArgs]) -> Vec<TokenStream> {
    error_tags
        .iter()
        .map(|tag| {
            let i = &tag.ident;
            quote! {
                impl From<#i> for #main_ident {
                    fn from(err: #i) -> Self {
                        Self::#i(err)
                    }
                }
            }
        })
        .collect()
}

fn generate_main_error_consume(
    main_visibility: &Visibility, main_ident: &Ident, error_tags: &[ErrorTagArgs],
) -> TokenStream {
    let args_def = error_tags.iter().map(|tag| {
        let i = &tag.ident;
        let arg_name = format!("{}_handler", to_snake_case(i.to_string().as_str()));
        let arg_ident = Ident::new(&arg_name, i.span());
        quote! {
            #arg_ident: impl FnOnce(errorium::anyhow::Error),
        }
    });

    let match_arms_def = error_tags.iter().map(|tag| {
        let i = &tag.ident;
        let arg_name = format!("{}_handler", to_snake_case(i.to_string().as_str()));
        let arg_ident = Ident::new(&arg_name, i.span());
        quote! {
            Self::#i(err) => #arg_ident(err.0),
        }
    });

    quote! {
        impl #main_ident {
            #[allow(dead_code)]
            #main_visibility fn consume(self, #(#args_def)*) {
                match self {
                    #(#match_arms_def)*
                }
            }
        }
    }
}
