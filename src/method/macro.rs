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
