use crate::*;

macro_rules! impl_http_method_macro {
    ($name:ident, $method:expr) => {
        pub(crate) fn $name(item: TokenStream) -> TokenStream {
            expand_check_macro(
                item,
                create_method_check($method, proc_macro2::Span::call_site()),
            )
        }
    };
}

impl_http_method_macro!(get_handler, "get");
impl_http_method_macro!(post_handler, "post");
impl_http_method_macro!(put_handler, "put");
impl_http_method_macro!(delete_handler, "delete");
impl_http_method_macro!(patch_handler, "patch");
impl_http_method_macro!(head_handler, "head");
impl_http_method_macro!(options_handler, "options");
impl_http_method_macro!(connect_handler, "connect");
impl_http_method_macro!(trace_handler, "trace");

macro_rules! impl_protocol_check_macro {
    ($name:ident, $check:ident) => {
        pub(crate) fn $name(item: TokenStream) -> TokenStream {
            expand_check_macro(item, |context| {
                let check_fn = Ident::new(stringify!($check), proc_macro2::Span::call_site());
                quote! {
                    if !#context.get_request().await.#check_fn() {
                        return;
                    }
                }
            })
        }
    };
}

impl_protocol_check_macro!(h2c_macro, is_h2c);
impl_protocol_check_macro!(http0_9_macro, is_http0_9);
impl_protocol_check_macro!(http1_0_macro, is_http1_0);
impl_protocol_check_macro!(http1_1_macro, is_http1_1);
impl_protocol_check_macro!(http1_1_or_higher_macro, is_http1_1_or_higher);
impl_protocol_check_macro!(http2_macro, is_http2);
impl_protocol_check_macro!(http3_macro, is_http3);
impl_protocol_check_macro!(tls_macro, is_tls);
impl_protocol_check_macro!(filter_unknown_method_macro, is_unknown_method);
impl_protocol_check_macro!(filter_unknown_upgrade_macro, is_unknown_upgrade);
impl_protocol_check_macro!(filter_unknown_version_macro, is_unknown_version);
