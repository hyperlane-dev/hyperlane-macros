use crate::*;

pub(crate) fn host_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let host_data: HostData = parse_macro_input!(attr as HostData);
    let host_value: Expr = host_data.host_value;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let request_host: RequestHost = #context.get_request_host().await;
            if request_host != #host_value.to_string() {
                return;
            }
        }
    })
}

pub(crate) fn host_filter_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let host_data: HostData = parse_macro_input!(attr as HostData);
    let host_value: Expr = host_data.host_value;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let request_host: RequestHost = #context.get_request_host().await;
            if request_host == #host_value.to_string() {
                return;
            }
        }
    })
}
