use crate::*;

/// Filters requests matching the specified Referer header.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with Referer filter.
pub(crate) fn referer_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let referer_data: RefererData = parse_macro_input!(attr as RefererData);
    let referer_value: Expr = referer_data.referer_value;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let referer: OptionRequestHeadersValueItem = #context.get_request_header_back(REFERER).await;
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

/// Filters requests not matching the specified Referer header.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inverse Referer filter.
pub(crate) fn referer_filter_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let referer_data: RefererData = parse_macro_input!(attr as RefererData);
    let referer_value: Expr = referer_data.referer_value;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let referer: OptionRequestHeadersValueItem = #context.get_request_header_back(REFERER).await;
            if let Some(referer_header) = referer {
                if referer_header == #referer_value {
                    return;
                }
            }
        }
    })
}
