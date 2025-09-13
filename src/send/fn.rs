use crate::*;

/// Sends the response with both headers and body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
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
/// - `Position` - The position to inject the code.
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
/// - `Position` - The position to inject the code.
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
/// - `Position` - The position to inject the code.
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

/// Sends the response with both headers and body with specified data.
///
/// # Arguments
///
/// - `attr` - The attribute token stream containing the data to send.
/// - `item` - The input token stream to process.
/// - `position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with send operation.
pub(crate) fn send_with_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let send_data: SendData = parse_macro_input!(attr as SendData);
    let data: Expr = send_data.data;
    inject(position, item, |context| {
        quote! {
            let _ = #context.send_with_data(#data).await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send_with_data",
        handler: Handler::WithAttrPosition(send_with_data_macro),
    }
}

/// Sends the response once with both headers and body with specified data (no keep-alive).
///
/// # Arguments
///
/// - `attr` - The attribute token stream containing the data to send.
/// - `item` - The input token stream to process.
/// - `position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with single send operation.
pub(crate) fn send_once_with_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let send_data: SendData = parse_macro_input!(attr as SendData);
    let data: Expr = send_data.data;
    inject(position, item, |context| {
        quote! {
            let _ = #context.send_once_with_data(#data).await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send_once_with_data",
        handler: Handler::WithAttrPosition(send_once_with_data_macro),
    }
}

/// Sends only the response body with specified data.
///
/// # Arguments
///
/// - `attr` - The attribute token stream containing the data to send.
/// - `item` - The input token stream to process.
/// - `position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body send operation.
pub(crate) fn send_body_with_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let send_data: SendData = parse_macro_input!(attr as SendData);
    let data: Expr = send_data.data;
    inject(position, item, |context| {
        quote! {
            let _ = #context.send_body_with_data(#data).await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "send_body_with_data",
        handler: Handler::WithAttrPosition(send_body_with_data_macro),
    }
}
