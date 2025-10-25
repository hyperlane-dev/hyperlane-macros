use crate::*;

/// Filters requests matching the specified host.
/// Supports both single and multiple host value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with host filter.
pub(crate) fn host_macro(attr: TokenStream, item: TokenStream, position: Position) -> TokenStream {
    let multi_host: MultiHostData = parse_macro_input!(attr as MultiHostData);
    inject(position, item, |context| {
        let statements = multi_host.host_values.iter().map(|host_value| {
            quote! {
                let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
                if request_host.as_str() != #host_value {
                    return;
                }
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "host",
        handler: Handler::WithAttrPosition(host_macro),
    }
}

/// Reject requests not matching the specified host.
/// Supports both single and multiple host value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inverse host filter.
pub(crate) fn reject_host_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_host: MultiHostData = parse_macro_input!(attr as MultiHostData);
    inject(position, item, |context| {
        let statements = multi_host.host_values.iter().map(|host_value| {
            quote! {
                let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
                if request_host.as_str() == #host_value {
                    return;
                }
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "reject_host",
        handler: Handler::WithAttrPosition(reject_host_macro),
    }
}
