use crate::*;

/// Expands macro with code inserted before method body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `FnOnce(&Ident) -> TokenStream2` - Function to generate code inserted before.
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
    match parse_context_from_signature(sig) {
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
/// - `FnOnce(&Ident) -> TokenStream2` - A closure that takes a context identifier and returns a `TokenStream` to be inserted at the end of the method.
fn inject_at_end(input: TokenStream, after_fn: impl FnOnce(&Ident) -> TokenStream2) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_signature(sig) {
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
/// - `FnOnce(&Ident) -> TokenStream2` - A closure that generates the code to be injected, based on the method's context identifier.
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub(crate) fn parse_self_from_method(sig: &Signature) -> syn::Result<&Ident> {
    match sig.inputs.first() {
        Some(FnArg::Receiver(_)) => match sig.inputs.iter().nth(1) {
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
        },
        _ => Err(syn::Error::new_spanned(
            &sig.inputs,
            "expected self as first argument for method",
        )),
    }
}

/// Checks if a type matches `::hyperlane::Context`.
///
/// This function checks if the given type is a reference to `::hyperlane::Context`.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - Returns `true` if the type is `&::hyperlane::Context` or `&mut Context`, `false` otherwise.
fn is_context_type(ty: &Type) -> bool {
    if let Type::Reference(type_ref) = ty
        && let Type::Path(type_path) = &*type_ref.elem
    {
        let path: &Path = &type_path.path;
        if path.segments.len() >= 2 {
            let segments: Vec<_> = path.segments.iter().collect();
            if segments.len() >= 2 {
                let last_two: &[&PathSegment] = &segments[segments.len() - 2..];
                if last_two[0].ident == "hyperlane" && last_two[1].ident == "Context" {
                    return true;
                }
            }
        }
        if path.segments.len() == 1 && path.segments[0].ident == "Context" {
            return true;
        }
    }
    false
}

/// Parses context identifier from function signature by searching all parameters.
///
/// This function iterates through all function parameters and returns the first one
/// that has type `::hyperlane::Context`. It supports:
/// 1. Methods with self: Searches from the second parameter onwards
/// 2. Functions without self: Searches from the first parameter onwards
/// 3. Context parameter can be at any position
///
/// # Arguments
///
/// - `&Signature` - The function signature to parse.
///
/// # Returns
///
/// - `syn::Result<&Ident>` - Returns the context identifier.
pub(crate) fn parse_context_from_signature(sig: &Signature) -> syn::Result<&Ident> {
    for arg in sig.inputs.iter() {
        if let FnArg::Typed(pat_type) = arg
            && is_context_type(&pat_type.ty)
        {
            match &*pat_type.pat {
                Pat::Ident(pat_ident) => return Ok(&pat_ident.ident),
                Pat::Wild(wild) => {
                    return Err(syn::Error::new_spanned(
                        wild,
                        "The context argument cannot be anonymous `_`, please use a named identifier",
                    ));
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pat_type.pat,
                        "expected identifier for context parameter",
                    ));
                }
            }
        }
    }
    Err(syn::Error::new_spanned(
        &sig.inputs,
        "expected at least one parameter of type &::hyperlane::Context",
    ))
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
