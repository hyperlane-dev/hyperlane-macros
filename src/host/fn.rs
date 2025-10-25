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
    if let Ok(multi_host) = syn::parse::<MultiHostData>(attr.clone()) {
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
    } else {
        let host_data: HostData = parse_macro_input!(attr as HostData);
        let host_value: Expr = host_data.host_value;
        inject(position, item, |context| {
            quote! {
                let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
                if request_host.as_str() != #host_value {
                    return;
                }
            }
        })
    }
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
    if let Ok(multi_host) = syn::parse::<MultiHostData>(attr.clone()) {
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
    } else {
        let host_data: HostData = parse_macro_input!(attr as HostData);
        let host_value: Expr = host_data.host_value;
        inject(position, item, |context| {
            quote! {
                let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
                if request_host.as_str() == #host_value {
                    return;
                }
            }
        })
    }
}

inventory::submit! {
    InjectableMacro {
        name: "reject_host",
        handler: Handler::WithAttrPosition(reject_host_macro),
    }
}
