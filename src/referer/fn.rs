use crate::*;

/// Filters requests matching the specified Referer header.
/// Supports both single and multiple referer value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with Referer filter.
pub(crate) fn referer_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    if let Ok(multi_referer) = syn::parse::<MultiRefererData>(attr.clone()) {
        inject(position, item, |context| {
            let statements = multi_referer.referer_values.iter().map(|referer_value| {
                quote! {
                    let referer: ::hyperlane::OptionRequestHeadersValueItem = #context.try_get_request_header_back(REFERER).await;
                    if let Some(referer_header) = referer {
                        if referer_header != #referer_value {
                            return;
                        }
                    } else {
                        return;
                    }
                }
            });
            quote! {
                #(#statements)*
            }
        })
    } else {
        let referer_data: RefererData = parse_macro_input!(attr as RefererData);
        let referer_value: Expr = referer_data.referer_value;
        inject(position, item, |context| {
            quote! {
                let referer: ::hyperlane::OptionRequestHeadersValueItem = #context.try_get_request_header_back(REFERER).await;
                if let Some(referer_header) = referer {
                    if referer_header != #referer_value {
                        return;
                    }
                } else {
                    return;
                }
            }
        })
    }
}

inventory::submit! {
    InjectableMacro {
        name: "referer",
        handler: Handler::WithAttrPosition(referer_macro),
    }
}

/// Reject requests not matching the specified Referer header.
/// Supports both single and multiple referer value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inverse Referer filter.
pub(crate) fn reject_referer_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    if let Ok(multi_referer) = syn::parse::<MultiRefererData>(attr.clone()) {
        inject(position, item, |context| {
            let statements = multi_referer.referer_values.iter().map(|referer_value| {
                quote! {
                    let referer: ::hyperlane::OptionRequestHeadersValueItem = #context.try_get_request_header_back(REFERER).await;
                    if let Some(referer_header) = referer {
                        if referer_header == #referer_value {
                            return;
                        }
                    }
                }
            });
            quote! {
                #(#statements)*
            }
        })
    } else {
        let referer_data: RefererData = parse_macro_input!(attr as RefererData);
        let referer_value: Expr = referer_data.referer_value;
        inject(position, item, |context| {
            quote! {
                let referer: ::hyperlane::OptionRequestHeadersValueItem = #context.try_get_request_header_back(REFERER).await;
                if let Some(referer_header) = referer {
                    if referer_header == #referer_value {
                        return;
                    }
                }
            }
        })
    }
}

inventory::submit! {
    InjectableMacro {
        name: "reject_referer",
        handler: Handler::WithAttrPosition(reject_referer_macro),
    }
}
