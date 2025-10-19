use crate::*;

/// Expands macro with code inserted before method body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `impl FnOnce(&Ident) -> TokenStream2` - Function to generate code inserted before.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inserted code.
fn inject_at_start(
    input: TokenStream,
    before_fn: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_self_from_method(sig) {
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

/// Expands macro with code inserted after method body.
///
/// # Arguments
///
/// - `TokenStream` - The input `TokenStream` to process.
/// - `impl FnOnce(&Ident) -> TokenStream2` - A closure that takes a context identifier and returns a `TokenStream` to be inserted at the end of the method.
fn inject_at_end(input: TokenStream, after_fn: impl FnOnce(&Ident) -> TokenStream2) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_self_from_method(sig) {
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

/// Injects code into a method at a specified position.
///
/// # Arguments
///
/// - `Position` - The position at which to inject the code (`Prologue` or `Epilogue`).
/// - `TokenStream` - The input `TokenStream` of the method to modify.
/// - `impl FnOnce(&Ident) -> TokenStream2` - A closure that generates the code to be injected, based on the method's context identifier.
///
/// # Returns
///
/// - `TokenStream` - Returns the modified `TokenStream` with the injected code.
pub(crate) fn inject(
    position: Position,
    input: TokenStream,
    hook: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    match position {
        Position::Prologue => inject_at_start(input, hook),
        Position::Epilogue => inject_at_end(input, hook),
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
/// - `syn::Result<&Ident>` - Returns a `syn::Result` containing the context identifier if successful, or an error otherwise.
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

/// Parses self from method signature and returns the context identifier (second parameter).
///
/// # Arguments
///
/// - `&Signature` - The method signature to parse.
///
/// # Returns
///
/// - `syn::Result<&Ident>` - Returns the context identifier from the second parameter.
pub(crate) fn parse_self_from_method(sig: &Signature) -> syn::Result<&Ident> {
    match sig.inputs.first() {
        Some(FnArg::Receiver(_)) => {
            // Get the second parameter (context)
            match sig.inputs.iter().nth(1) {
                Some(FnArg::Typed(pat_type)) => match &*pat_type.pat {
                    Pat::Ident(pat_ident) => Ok(&pat_ident.ident),
                    Pat::Wild(wild) => Err(syn::Error::new_spanned(
                        wild,
                        "The context argument cannot be anonymous `_`, please use a named identifier",
                    )),
                    _ => Err(syn::Error::new_spanned(
                        &pat_type.pat,
                        "expected identifier as second argument (context)",
                    )),
                },
                _ => Err(syn::Error::new_spanned(
                    &sig.inputs,
                    "expected context as second argument",
                )),
            }
        }
        _ => Err(syn::Error::new_spanned(
            &sig.inputs,
            "expected self as first argument for method",
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
/// # Arguments
///
/// - `&Option<Expr>` - An optional reference to the expression to convert.
///
/// # Returns
///
/// - `TokenStream` - A `TokenStream2` representing `Some(isize)` for supported literals, or `None` otherwise.
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

/// Checks if an expression is an integer literal.
///
/// # Arguments
///
/// - `expr` - The expression to check.
///
/// # Returns
///
/// - `bool` - Returns `true` if the expression is an integer literal, `false` otherwise.
pub(crate) fn is_integer_literal(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lit(ExprLit {
            lit: Lit::Int(_),
            ..
        })
    )
}

/// Handles macros that can be applied to either structs or functions.
/// Generates a factory function name based on prefix, struct name, and optional order.
///
/// This function creates a valid Rust identifier for factory functions by:
/// - Using only the prefix and struct name when order is None
/// - Extracting the numeric value from order expression and appending it when order is Some
///
/// # Arguments
///
/// - `prefix` - The prefix for the factory function name (e.g., "__panic_hook_factory").
/// - `struct_name` - The identifier of the struct.
/// - `order_expr` - Optional expression representing the order value.
///
/// # Returns
///
/// - `Ident` - A valid Rust identifier for the factory function.
pub(crate) fn generate_factory_fn_name(
    prefix: &str,
    struct_name: &Ident,
    order_expr: &Option<Expr>,
) -> Ident {
    match order_expr {
        None => Ident::new(&format!("{}_{}", prefix, struct_name), struct_name.span()),
        Some(expr) => {
            let order_suffix = match expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(lit_int),
                    ..
                }) => lit_int.base10_digits().to_string(),
                Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) => lit_str.value(),
                _ => "custom".to_string(),
            };
            Ident::new(
                &format!("{}_{}_{}", prefix, struct_name, order_suffix),
                struct_name.span(),
            )
        }
    }
}
