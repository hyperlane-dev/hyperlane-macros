use crate::*;

/// Checks if request is WebSocket protocol.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with protocol check.
pub(crate) fn ws_macro(item: TokenStream) -> TokenStream {
    expand_check_macro(item, |context| {
        quote! {
            if !#context.get_request().await.is_ws() {
                return;
            }
        }
    })
}

/// Checks if request is HTTP protocol.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with protocol check.
pub(crate) fn http_macro(item: TokenStream) -> TokenStream {
    expand_check_macro(item, |context| {
        quote! {
            if !#context.get_request().await.is_http() {
                return;
            }
        }
    })
}

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
