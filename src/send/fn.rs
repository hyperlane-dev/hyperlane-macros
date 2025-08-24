use crate::*;

/// Sends the response with both headers and body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with send operation.
pub(crate) fn send_macro(item: TokenStream) -> TokenStream {
    inject_at_end(item, |context| {
        quote! {
            let _ = #context.send().await;
        }
    })
}

/// Sends only the response body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body send operation.
pub(crate) fn send_body_macro(item: TokenStream) -> TokenStream {
    inject_at_end(item, |context| {
        quote! {
            let _ = #context.send_body().await;
        }
    })
}

/// Sends the response once with both headers and body (no keep-alive).
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with single send operation.
pub(crate) fn send_once_macro(item: TokenStream) -> TokenStream {
    inject_at_end(item, |context| {
        quote! {
            let _ = #context.send_once().await;
        }
    })
}

/// Sends only the response body once (no keep-alive).
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with single body send operation.
pub(crate) fn send_once_body_macro(item: TokenStream) -> TokenStream {
    inject_at_end(item, |context| {
        quote! {
            let _ = #context.send_once_body().await;
        }
    })
}
