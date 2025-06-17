mod method;

pub(crate) use method::*;

pub(crate) use proc_macro::TokenStream;
pub(crate) use quote::quote;
pub(crate) use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    get_handler(item)
}

#[proc_macro_attribute]
pub fn post(_attr: TokenStream, item: TokenStream) -> TokenStream {
    post_handler(item)
}

#[proc_macro_attribute]
pub fn put(_attr: TokenStream, item: TokenStream) -> TokenStream {
    put_handler(item)
}

#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, item: TokenStream) -> TokenStream {
    delete_handler(item)
}

#[proc_macro_attribute]
pub fn patch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    patch_handler(item)
}

#[proc_macro_attribute]
pub fn head(_attr: TokenStream, item: TokenStream) -> TokenStream {
    head_handler(item)
}

#[proc_macro_attribute]
pub fn options(_attr: TokenStream, item: TokenStream) -> TokenStream {
    options_handler(item)
}

#[proc_macro_attribute]
pub fn connect(_attr: TokenStream, item: TokenStream) -> TokenStream {
    connect_handler(item)
}

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    trace_handler(item)
}

#[proc_macro_attribute]
pub fn methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    methods_macro(attr, item)
}

#[proc_macro_attribute]
pub fn ws(_attr: TokenStream, item: TokenStream) -> TokenStream {
    ws_macro(item)
}

#[proc_macro_attribute]
pub fn http(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http_macro(item)
}

#[proc_macro_attribute]
pub fn status_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    code_macro(attr, item)
}

#[proc_macro_attribute]
pub fn reason_phrase(attr: TokenStream, item: TokenStream) -> TokenStream {
    reason_phrase_macro(attr, item)
}

#[proc_macro_attribute]
pub fn send(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_macro(item)
}

#[proc_macro_attribute]
pub fn send_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_body_macro(item)
}

#[proc_macro_attribute]
pub fn send_once(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_once_macro(item)
}

#[proc_macro_attribute]
pub fn send_once_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_once_body_macro(item)
}

#[proc_macro_attribute]
pub fn h2c(_attr: TokenStream, item: TokenStream) -> TokenStream {
    h2c_macro(item)
}

#[proc_macro_attribute]
pub fn http0_9(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http0_9_macro(item)
}

#[proc_macro_attribute]
pub fn http1_0(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_0_macro(item)
}

#[proc_macro_attribute]
pub fn http1_1(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_1_macro(item)
}

#[proc_macro_attribute]
pub fn http1_1_or_higher(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_1_or_higher_macro(item)
}

#[proc_macro_attribute]
pub fn http2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http2_macro(item)
}

#[proc_macro_attribute]
pub fn http3(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http3_macro(item)
}

#[proc_macro_attribute]
pub fn tls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tls_macro(item)
}

#[proc_macro_attribute]
pub fn filter_unknown_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_method_macro(item)
}

#[proc_macro_attribute]
pub fn filter_unknown_upgrade(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_upgrade_macro(item)
}

#[proc_macro_attribute]
pub fn filter_unknown_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_version_macro(item)
}

#[proc_macro_attribute]
pub fn filter_unknown(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_macro(item)
}
