use crate::*;

/// Tries to send the response with both headers and body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with try send operation.
pub(crate) fn try_send_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            let _ = #context.try_send().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "try_send",
        handler: Handler::NoAttrPosition(try_send_macro),
    }
}

inventory::submit! {
    InjectableMacro {
        name: "send",
        handler: Handler::NoAttrPosition(send_macro),
    }
}

/// Tries to send only the response body.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body try send operation.
pub(crate) fn try_send_body_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            let _ = #context.try_send_body().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "try_send_body",
        handler: Handler::NoAttrPosition(try_send_body_macro),
    }
}

inventory::submit! {
    InjectableMacro {
        name: "send_body",
        handler: Handler::NoAttrPosition(send_body_macro),
    }
}

/// Tries to send only the response body with specified data.
///
/// # Arguments
///
/// - `attr` - The attribute token stream containing the data to send.
/// - `item` - The input token stream to process.
/// - `position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body try send operation.
pub(crate) fn try_send_body_with_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let send_data: SendData = parse_macro_input!(attr as SendData);
    let data: Expr = send_data.data;
    inject(position, item, |context| {
        quote! {
            let _ = #context.try_send_body_with_data(#data).await;
        }
    })
}

/// Sends the response with both headers and body, panics on failure.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with send operation that panics on failure.
pub(crate) fn send_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            #context.send().await.unwrap();
        }
    })
}

/// Sends only the response body, panics on failure.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body send operation that panics on failure.
pub(crate) fn send_body_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            #context.send_body().await.unwrap();
        }
    })
}

/// Sends only the response body with specified data, panics on failure.
///
/// # Arguments
///
/// - `attr` - The attribute token stream containing the data to send.
/// - `item` - The input token stream to process.
/// - `position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body send operation that panics on failure.
pub(crate) fn send_body_with_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let send_data: SendData = parse_macro_input!(attr as SendData);
    let data: Expr = send_data.data;
    inject(position, item, |context| {
        quote! {
            #context.send_body_with_data(#data).await.unwrap();
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "try_send_body_with_data",
        handler: Handler::WithAttrPosition(try_send_body_with_data_macro),
    }
}

inventory::submit! {
    InjectableMacro {
        name: "send_body_with_data",
        handler: Handler::WithAttrPosition(send_body_with_data_macro),
    }
}
