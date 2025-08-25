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
pub(crate) fn send_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            let _ = #context.send().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send",
        handler: Handler::NoAttrPosition(send_macro),
    }
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
pub(crate) fn send_body_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            let _ = #context.send_body().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send_body",
        handler: Handler::NoAttrPosition(send_body_macro),
    }
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
pub(crate) fn send_once_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            let _ = #context.send_once().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send_once",
        handler: Handler::NoAttrPosition(send_once_macro),
    }
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
pub(crate) fn send_once_body_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            let _ = #context.send_once_body().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send_once_body",
        handler: Handler::NoAttrPosition(send_once_body_macro),
    }
}
