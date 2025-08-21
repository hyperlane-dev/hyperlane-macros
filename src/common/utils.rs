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
pub(crate) fn expand_macro_with_before_insertion(
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
pub(crate) fn expand_macro_with_after_insertion(
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

/// Expands macro with attribute parsing and code insertion before function body.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream to parse.
/// - `TokenStream` - The input token stream to process.
/// - `impl FnOnce(TokenStream) -> syn::Result<T>` - Function to parse attributes.
/// - `impl FnOnce(&Ident, T) -> TokenStream2` - Function to generate code inserted before.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inserted code.
pub(crate) fn expand_macro_with_attr_and_before_insertion<T>(
    attr: TokenStream,
    item: TokenStream,
    parse_attr: impl FnOnce(TokenStream) -> syn::Result<T>,
    before_fn: impl FnOnce(&Ident, T) -> TokenStream2,
) -> TokenStream {
    match parse_attr(attr) {
        Ok(value) => expand_macro_with_before_insertion(item, |context| before_fn(context, value)),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Expands macro with check code inserted before function body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `impl FnOnce(&Ident) -> TokenStream2` - Function to generate check code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with check code.
pub(crate) fn expand_check_macro(
    input: TokenStream,
    check_fn: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    expand_macro_with_before_insertion(input, check_fn)
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

pub(crate) fn expand_macro_with_before_return_insertion(
    input: TokenStream,
    before_return_fn: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    let mut input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let before_return_code: TokenStream2 = before_return_fn(context);
            let stmts: &mut Vec<Stmt> = &mut input_fn.block.stmts;
            if let Some(last_stmt) = stmts.pop() {
                let gen_code: TokenStream2 = quote! {
                    #(#attrs)*
                    #vis #sig {
                        #(#stmts)*
                        #before_return_code
                        #last_stmt
                    }
                };
                gen_code.into()
            } else {
                let gen_code: TokenStream2 = quote! {
                    #(#attrs)*
                    #vis #sig {
                        #before_return_code
                    }
                };
                gen_code.into()
            }
        }
        Err(err) => err.to_compile_error().into(),
    }
}
