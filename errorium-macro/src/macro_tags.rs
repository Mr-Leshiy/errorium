//! `errorium::tags!` macro implmenentation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Ident, Visibility,
};

use crate::error::{Error, Result};

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

pub(crate) fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tags = parse_macro_input!(input as ErrorTags);
    generate_impl(tags).unwrap_or_else(Into::into).into()
}

fn generate_impl(tags: ErrorTags) -> Result<TokenStream> {
    let ErrorTags(tags) = tags;

    if tags.is_empty() {
        return Err(Error::Other(
            "It should be at least one error tag".to_string(),
        ));
    }

    let error_tags_def = tags.iter().map(|tag| {
        generate_error_tag(
            &tag.visibility,
            &tag.ident,
            tags.iter()
                .filter(|t| t.ident == tag.ident)
                .map(|t| &t.ident),
        )
    });

    let res = quote! {
        #(#error_tags_def)*
    };
    Ok(res)
}

fn generate_error_tag<'a>(
    visibility: &'a Visibility, ident: &'a Ident, other_tags: impl Iterator<Item = &'a Ident>,
) -> TokenStream {
    let tag_type_def = quote! {
        #[derive(Debug)]
        #visibility struct #ident(Box<dyn std::error::Error + Send + Sync + 'static>);
    };

    let tag_type_impl_def = generate_tag_type(visibility, ident, other_tags);

    let tag_type_std_traits_impl_def = generate_tag_std_traits_impl(ident);

    quote! {
        #tag_type_def

        #tag_type_impl_def

        #tag_type_std_traits_impl_def
    }
}

fn generate_tag_type<'a>(
    visibility: &'a Visibility, ident: &'a Ident, other_tags: impl Iterator<Item = &'a Ident>,
) -> TokenStream {
    let handle_tag_conditions = other_tags.map(|ident| {
        quote! {
            else if let Some(tag) = err.downcast_ref::<#ident>() {
                Self::handle(tag.0.as_ref(), handler);
            }
        }
    });

    quote! {
        impl #ident {
            #visibility fn handle<F>(err: &(dyn std::error::Error + 'static), handler: F)
            where F: FnOnce(&dyn std::error::Error) {
                if let Some(tag) = err.downcast_ref::<#ident>() {
                    handler(tag.0.as_ref());
                } #(#handle_tag_conditions)*
            }

            fn tag<T: Into<Box<dyn std::error::Error + Send + Sync + 'static>>>(val: T) -> Box<Self> {
                Self(val.into()).into()
            }
        }
    }
}

fn generate_tag_std_traits_impl(ident: &Ident) -> TokenStream {
    quote! {
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
    }
}
