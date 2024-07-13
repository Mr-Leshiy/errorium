//! `errorium` macro definitions crate, with all corresponding procedural macros
//! implementation.

mod error;
mod utils;

use error::{Error, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Ident, Visibility,
};
use utils::to_snake_case;

struct ErroriumArgs {
    visibility: Visibility,
    main_error_struct_ident: Ident,
    #[allow(unused)]
    comma_token: Comma,
    #[allow(unused)]
    bracket_token: Bracket,
    error_tags: Vec<ErrorTagArgs>,
}

struct ErrorTagArgs {
    visibility: Visibility,
    ident: Ident,
}

impl Parse for ErroriumArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let main_error_struct_ident = input.parse()?;
        let comma_token = input.parse::<Comma>()?;

        let content;
        let bracket_token = bracketed!(content in input);
        let error_tags = Punctuated::<ErrorTagArgs, Comma>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Self {
            visibility,
            main_error_struct_ident,
            comma_token,
            bracket_token,
            error_tags,
        })
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
pub fn errorium(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as ErroriumArgs);
    generate(args).unwrap_or_else(Into::into).into()
}

fn generate(args: ErroriumArgs) -> Result<TokenStream> {
    let ErroriumArgs {
        visibility,
        main_error_struct_ident,
        error_tags: error_type_idents,
        ..
    } = args;

    if error_type_idents.is_empty() {
        return Err(Error::Other(
            "It should be at least one error type ident".to_string(),
        ));
    }

    let error_types_def = error_type_idents
        .iter()
        .map(|tag| generate_error_tag_type(&tag.visibility, &tag.ident));

    let main_error_def =
        generate_main_error(&visibility, &main_error_struct_ident, &error_type_idents);

    let res = quote! {
        #(#error_types_def)*

        #main_error_def
    };
    Ok(res)
}

fn generate_error_tag_type(visibility: &Visibility, ident: &Ident) -> TokenStream {
    quote! {
        #visibility struct #ident(errorium::anyhow::Error);

        impl #ident {
            #[allow(dead_code)]
            pub fn new(err: impl Into<errorium::anyhow::Error>) -> Self {
                Self(err.into())
            }
        }

        impl<E> From<E> for #ident
        where E: Into<errorium::anyhow::Error>
        {
            fn from(err: E) -> Self {
                Self(err.into())
            }
        }
    }
}

fn generate_main_error(
    main_visibility: &Visibility, main_ident: &Ident, error_tags: &[ErrorTagArgs],
) -> TokenStream {
    let enum_def = generate_main_error_enum(main_visibility, main_ident, error_tags);
    let from_defs = generate_main_error_from(main_ident, error_tags);
    let consume_def = generate_main_error_consume(main_visibility, main_ident, error_tags);

    quote! {
        #enum_def

        #(#from_defs)*

        #consume_def
    }
}

fn generate_main_error_enum(
    main_visibility: &Visibility, main_ident: &Ident, error_tags: &[ErrorTagArgs],
) -> TokenStream {
    let variants = error_tags.iter().map(|tag| {
        let ident = &tag.ident;
        quote! { #ident(#ident), }
    });
    quote! {
        #[allow(dead_code)]
        #main_visibility enum #main_ident
        {
            #(#variants)*
        }

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
