use crate::*;

/// Expands macro with code inserted before function body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `impl FnOnce(&Ident) -> TokenStream2` - Function to generate code inserted before.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inserted code.
pub(crate) fn inject_at_start(
    input: TokenStream,
    before_fn: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let before_code: TokenStream2 = before_fn(context);
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    #before_code
                    #(#stmts)*
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

/// Expands macro with code inserted after function body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `impl FnOnce(&Ident) -> TokenStream2` - Function to generate code inserted after.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inserted code.
pub(crate) fn inject_at_end(
    input: TokenStream,
    after_fn: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let after_code: TokenStream2 = after_fn(context);
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    #(#stmts)*
                    #after_code
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

/// Parses context identifier from function signature.
///
/// # Arguments
///
/// - `&Signature` - The function signature to parse.
///
/// # Returns
///
/// - `syn::Result<&Ident>` - The parsed context identifier or error.
pub(crate) fn parse_context_from_fn(sig: &Signature) -> syn::Result<&Ident> {
    match sig.inputs.first() {
        Some(FnArg::Typed(pat_type)) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => Ok(&pat_ident.ident),
            Pat::Wild(wild) => Err(syn::Error::new_spanned(
                wild,
                "The argument cannot be anonymous `_`, please use a named identifier",
            )),
            _ => Err(syn::Error::new_spanned(
                &pat_type.pat,
                "expected identifier as first argument",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            &sig.inputs,
            "expected at least one argument",
        )),
    }
}

/// Convert an optional expression into an `Option<isize>` token stream.
///
/// This function supports integer and string literals only:
/// - Integer literals are parsed and converted into `Some(isize)`.
/// - String literals are parsed into `isize` and wrapped in `Some(...)`.
/// - Any other expression types will result in `None`.
/// - If `opt_expr` is `None`, the result is also `None`.
///
/// # Parameters
/// - `opt_expr` - An optional reference to the expression to convert.
///
/// # Returns
/// - A `TokenStream2` representing `Some(isize)` for supported literals, or `None` otherwise.
pub(crate) fn expr_to_isize(opt_expr: &Option<Expr>) -> TokenStream2 {
    match opt_expr {
        Some(expr) => match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) => {
                let value: isize = lit_int.base10_parse::<isize>().unwrap();
                quote! { Some(#value) }
            }
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                let value: isize = lit_str.value().parse().expect("Cannot parse to isize");
                quote! { Some(#value) }
            }
            _ => quote! { None },
        },
        None => quote! { None },
    }
}
