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
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
            if request_host != #host_value.to_string() {
                return;
            }
        }
    })
}

/// Filters requests not matching the specified host.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inverse host filter.
pub(crate) fn host_filter_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let host_data: HostData = parse_macro_input!(attr as HostData);
    let host_value: Expr = host_data.host_value;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let request_host: ::hyperlane::RequestHost = #context.get_request_host().await;
            if request_host == #host_value.to_string() {
                return;
            }
        }
    })
}
