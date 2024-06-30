//!

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
    master_error_struct_ident: Ident,
    #[allow(unused)]
    comma_token: Comma,
    #[allow(unused)]
    bracket_token: Bracket,
    error_type_idents: Vec<Ident>,
}

impl Parse for ErroriumArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let master_error_struct_ident = input.parse()?;
        let comma_token = input.parse::<Comma>()?;

        let content;
        let bracket_token = bracketed!(content in input);
        let error_type_idents = Punctuated::<Ident, Comma>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Self {
            visibility,
            master_error_struct_ident,
            comma_token,
            bracket_token,
            error_type_idents,
        })
    }
}

///
#[proc_macro]
pub fn errorium(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as ErroriumArgs);
    generate(args).unwrap_or_else(Into::into).into()
}

fn generate(args: ErroriumArgs) -> Result<TokenStream> {
    let ErroriumArgs {
        visibility,
        master_error_struct_ident,
        error_type_idents,
        ..
    } = args;

    if error_type_idents.is_empty() {
        return Err(Error::Other(
            "It should be at least one error type ident".to_string(),
        ));
    }

    let error_types_def = error_type_idents
        .iter()
        .map(|ident| generate_error_type(&visibility, ident));

    let master_error_def =
        generate_master_error(&visibility, &master_error_struct_ident, &error_type_idents);

    let res = quote! {
        #(#error_types_def)*

        #master_error_def
    };
    println!("{}", res);
    Ok(res)
}

fn generate_error_type(visibility: &Visibility, ident: &Ident) -> TokenStream {
    quote! {
        #visibility struct #ident(errorium::anyhow::Error);

        impl #ident {
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

fn generate_master_error(
    visibility: &Visibility, master_ident: &Ident, error_type_idents: &[Ident],
) -> TokenStream {
    let enum_def = generate_master_error_enum(visibility, master_ident, error_type_idents);
    let from_defs = generate_master_error_from(master_ident, error_type_idents);
    let consume_def = generate_master_error_consume(visibility, master_ident, error_type_idents);

    quote! {
        #enum_def

        #(#from_defs)*

        #consume_def
    }
}

fn generate_master_error_enum(
    visibility: &Visibility, master_ident: &Ident, error_type_idents: &[Ident],
) -> TokenStream {
    let variants = error_type_idents.iter().map(|i| {
        quote! { #i(#i), }
    });
    quote! {
        #visibility enum #master_ident
        {
            #(#variants)*
        }

    }
}

fn generate_master_error_from(
    master_ident: &Ident, error_type_idents: &[Ident],
) -> Vec<TokenStream> {
    error_type_idents
        .iter()
        .map(|i| {
            quote! {
                impl From<#i> for #master_ident {
                    fn from(err: #i) -> Self {
                        Self::#i(err)
                    }
                }
            }
        })
        .collect()
}

fn generate_master_error_consume(
    visibility: &Visibility, master_ident: &Ident, error_type_idents: &[Ident],
) -> TokenStream {
    let args_def = error_type_idents.iter().map(|i| {
        let arg_name = format!("{}_handler", to_snake_case(i.to_string()));
        let arg_ident = Ident::new(&arg_name, i.span());
        quote! {
            #arg_ident: impl FnOnce(errorium::anyhow::Error),
        }
    });

    let match_arms_def = error_type_idents.iter().map(|i| {
        let arg_name = format!("{}_handler", to_snake_case(i.to_string()));
        let arg_ident = Ident::new(&arg_name, i.span());
        quote! {
            Self::#i(err) => #arg_ident(err.0),
        }
    });

    quote! {
        impl #master_ident {
            #visibility fn consume(self, #(#args_def)*) {
                match self {
                    #(#match_arms_def)*
                }
            }
        }
    }
}
