mod aborted;
mod closed;
mod common;
mod filter;
mod flush;
mod hook;
mod http;
mod protocol;
mod request;
mod response;
mod send;

pub(crate) use aborted::*;
pub(crate) use closed::*;
pub(crate) use common::*;
pub(crate) use filter::*;
pub(crate) use flush::*;
pub(crate) use hook::*;
pub(crate) use http::*;
pub(crate) use protocol::*;
pub(crate) use request::*;
pub(crate) use response::*;
pub(crate) use send::*;

pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
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
pub fn flush(_attr: TokenStream, item: TokenStream) -> TokenStream {
    flush_macro(item)
}

#[proc_macro_attribute]
pub fn aborted(_attr: TokenStream, item: TokenStream) -> TokenStream {
    aborted_macro(item)
}

#[proc_macro_attribute]
pub fn closed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    closed_macro(item)
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

#[proc_macro_attribute]
pub fn pre_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    pre_hook_macro(attr, item)
}

#[proc_macro_attribute]
pub fn post_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    post_hook_macro(attr, item)
}

#[proc_macro_attribute]
pub fn body(attr: TokenStream, item: TokenStream) -> TokenStream {
    body_macro(attr, item)
}

#[proc_macro_attribute]
pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute_macro(attr, item)
}

#[proc_macro_attribute]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes_macro(attr, item)
}

#[proc_macro_attribute]
pub fn route_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_param_macro(attr, item)
}

#[proc_macro_attribute]
pub fn route_params(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_params_macro(attr, item)
}

#[proc_macro_attribute]
pub fn query(attr: TokenStream, item: TokenStream) -> TokenStream {
    query_macro(attr, item)
}

#[proc_macro_attribute]
pub fn querys(attr: TokenStream, item: TokenStream) -> TokenStream {
    querys_macro(attr, item)
}

#[proc_macro_attribute]
pub fn header(attr: TokenStream, item: TokenStream) -> TokenStream {
    header_macro(attr, item)
}

#[proc_macro_attribute]
pub fn headers(attr: TokenStream, item: TokenStream) -> TokenStream {
    headers_macro(attr, item)
}
