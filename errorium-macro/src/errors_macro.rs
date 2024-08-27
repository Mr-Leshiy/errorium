//! `errorium::errors` attribute macro implmenentation.

#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, parse_macro_input, Attribute, Block, Expr, ItemFn, Stmt, Token};

use crate::error::Result;

/// Special `return result ...` expression
struct ExprReturnResult {
    attrs: Vec<Attribute>,
    return_token: Token![return],
    result_token: Token![return],
    expr: Option<Box<Expr>>,
}

pub(crate) fn generate(
    _attr: proc_macro::TokenStream, input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    generate_impl(input_fn).unwrap_or_else(Into::into).into()
}

fn generate_impl(input_fn: ItemFn) -> Result<TokenStream> {
    let mut output_fn = input_fn;

    extend_function_block(&mut output_fn.block)?;

    let res = quote! {
        #output_fn
    };
    println!("res: \"{res}\"");
    Ok(res)
}

fn extend_function_block(block: &mut Block) -> Result<()> {
    block.stmts.clear();
    block.stmts.insert(0, generate_errors_array_init_stmt()?);

    Ok(())
}

fn generate_errors_array_init_stmt() -> Result<Stmt> {
    let errors_array_init_stmt = quote! {let mut errors: Vec<errorium::Error> = Vec::new(); };
    Ok(parse::<Stmt>(errors_array_init_stmt.into())?)
}
