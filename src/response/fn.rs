use crate::*;

/// Parses a TokenStream into an Expr.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to parse
///
/// # Returns
///
/// - `syn::Result<Expr>` - Parsed expression or error
fn parse_expr(input: TokenStream) -> syn::Result<Expr> {
    syn::parse::<Expr>(input)
}

/// Sets response status code from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The status code expression
/// - `TokenStream` - The input token stream to process
///
/// # Returns
///
/// - `TokenStream` - Expanded token stream with status code setting
pub(crate) fn response_status_code_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_macro_with_attr_and_before_insertion(attr, item, parse_expr, |context, value| {
        quote! {
            #context.set_response_status_code(hyperlane::ResponseStatusCode::from(#value as usize)).await;
        }
    })
}

/// Sets response reason phrase from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The reason phrase expression
/// - `TokenStream` - The input token stream to process
///
/// # Returns
///
/// - `TokenStream` - Expanded token stream with reason phrase setting
pub(crate) fn response_reason_phrase_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_macro_with_attr_and_before_insertion(attr, item, parse_expr, |context, value| {
        quote! {
            #context.set_response_reason_phrase(#value).await;
        }
    })
}

/// Sets or replaces response header from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The header key, value and operation
/// - `TokenStream` - The input token stream to process
///
/// # Returns
///
/// - `TokenStream` - Expanded token stream with header operation
pub(crate) fn response_header_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let header_data: ResponseHeaderData = parse_macro_input!(attr as ResponseHeaderData);
    let key: Expr = header_data.key;
    let value: Expr = header_data.value;
    let operation: HeaderOperation = header_data.operation;
    expand_macro_with_before_insertion(item, |context| match operation {
        HeaderOperation::Set => {
            quote! {
                #context.set_response_header(#key, #value).await;
            }
        }
        HeaderOperation::Replace => {
            quote! {
                #context.replace_response_header(#key, #value).await;
            }
        }
    })
}

/// Sets response body from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The body expression
/// - `TokenStream` - The input token stream to process
///
/// # Returns
///
/// - `TokenStream` - Expanded token stream with body setting
pub(crate) fn response_body_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_data: ResponseBodyData = parse_macro_input!(attr as ResponseBodyData);
    let body: Expr = body_data.body;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            #context.set_response_body(#body).await;
        }
    })
}

/// Sets response version from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The version expression
/// - `TokenStream` - The input token stream to process
///
/// # Returns
///
/// - `TokenStream` - Expanded token stream with version setting
pub(crate) fn response_version_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_macro_with_attr_and_before_insertion(attr, item, parse_expr, |context, value| {
        quote! {
            #context.set_response_version(#value).await;
        }
    })
}
