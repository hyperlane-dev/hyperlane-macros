use crate::*;

/// Filters requests matching the specified host.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with host filter.
pub(crate) fn host_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let host_data: HostData = parse_macro_input!(attr as HostData);
    let host_value: Expr = host_data.host_value;
    inject_at_start(item, |context| {
        quote! {
            let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
            if request_host != #host_value.to_string() {
                return;
            }
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "host",
        handler: Handler::WithAttr(host_macro),
    }
}

/// Reject requests not matching the specified host.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inverse host filter.
pub(crate) fn reject_host_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let host_data: HostData = parse_macro_input!(attr as HostData);
    let host_value: Expr = host_data.host_value;
    inject_at_start(item, |context| {
        quote! {
            let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
            if request_host == #host_value.to_string() {
                return;
            }
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "reject_host",
        handler: Handler::WithAttr(reject_host_macro),
    }
}
