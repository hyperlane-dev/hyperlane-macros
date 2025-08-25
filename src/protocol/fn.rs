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
pub(crate) fn ws_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            if !#context.get_request().await.is_ws() {
                return;
            }
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "ws",
        handler: Handler::SimplePosition(ws_macro),
    }
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
pub(crate) fn http_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            if !#context.get_request().await.is_http() {
                return;
            }
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "http",
        handler: Handler::SimplePosition(http_macro),
    }
}

/// Implements a protocol check macro.
///
/// This macro generates a function that checks if the request matches a specific protocol.
/// If the protocol does not match, the request is aborted.
///
/// # Arguments
///
/// - `$name`: The name of the generated macro function.
/// - `$check`: The name of the method to call on the request to perform the protocol check (e.g., `is_h2c`).
macro_rules! impl_protocol_check_macro {
    ($name:ident, $check:ident, $str_name:expr) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(position, item, |context| {
                let check_fn = Ident::new(stringify!($check), proc_macro2::Span::call_site());
                quote! {
                    let request: ::hyperlane::Request = #context.get_request().await;
                    if !request.#check_fn() {
                        return;
                    }
                }
            })
        }
        inventory::submit! {
            InjectableMacro {
                name: $str_name,
                handler: Handler::SimplePosition($name),
            }
        }
    };
}

// Checks if the request is H2C protocol.
impl_protocol_check_macro!(h2c_macro, is_h2c, "h2c");

// Checks if the request is HTTP/0.9 protocol.
impl_protocol_check_macro!(http0_9_macro, is_http0_9, "http0_9");

// Checks if the request is HTTP/1.0 protocol.
impl_protocol_check_macro!(http1_0_macro, is_http1_0, "http1_0");

// Checks if the request is HTTP/1.1 protocol.
impl_protocol_check_macro!(http1_1_macro, is_http1_1, "http1_1");

// Checks if the request is HTTP/1.1 or higher protocol.
impl_protocol_check_macro!(
    http1_1_or_higher_macro,
    is_http1_1_or_higher,
    "http1_1_or_higher"
);

// Checks if the request is HTTP/2 protocol.
impl_protocol_check_macro!(http2_macro, is_http2, "http2");

// Checks if the request is HTTP/3 protocol.
impl_protocol_check_macro!(http3_macro, is_http3, "http3");

// Checks if the request is TLS protocol.
impl_protocol_check_macro!(tls_macro, is_tls, "tls");
