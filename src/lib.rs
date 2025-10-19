//! hyperlane-macros
//!
//! A comprehensive collection of procedural macros for building
//! HTTP servers with enhanced functionality. This crate provides
//! attribute macros that simplify HTTP request handling, protocol
//! validation, response management, and request data extraction.

mod aborted;
mod closed;
mod common;
mod filter;
mod flush;
mod from_stream;
mod hook;
mod host;
mod http;
mod hyperlane;
mod inject;
mod protocol;
mod referer;
mod reject;
mod request;
mod request_middleware;
mod response;
mod response_middleware;
mod route;
mod send;
mod stream;

pub(crate) use aborted::*;
pub(crate) use closed::*;
pub(crate) use common::*;
pub(crate) use filter::*;
pub(crate) use flush::*;
pub(crate) use from_stream::*;
pub(crate) use hook::*;
pub(crate) use host::*;
pub(crate) use http::*;
pub(crate) use hyperlane::*;
pub(crate) use inject::*;
pub(crate) use protocol::*;
pub(crate) use referer::*;
pub(crate) use reject::*;
pub(crate) use request::*;
pub(crate) use request_middleware::*;
pub(crate) use response::*;
pub(crate) use response_middleware::*;
pub(crate) use route::*;
pub(crate) use send::*;
pub(crate) use stream::*;

pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
pub(crate) use quote::quote;
pub(crate) use syn::{
    Ident, Token,
    parse::{Parse, ParseStream, Parser, Result},
    punctuated::Punctuated,
    token::Comma,
    *,
};

inventory::collect!(InjectableMacro);

/// Restricts function execution to HTTP GET requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the GET HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/get")]
/// struct Get;
///
/// impl ServerHook for Get {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(get, response_body("get"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    get_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP POST requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the POST HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/post")]
/// struct Post;
///
/// impl ServerHook for Post {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(post, response_body("post"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn post(_attr: TokenStream, item: TokenStream) -> TokenStream {
    epilogue_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP PUT requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the PUT HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/put")]
/// struct Put;
///
/// impl ServerHook for Put {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(put, response_body("put"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn put(_attr: TokenStream, item: TokenStream) -> TokenStream {
    put_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP DELETE requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the DELETE HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/delete")]
/// struct Delete;
///
/// impl ServerHook for Delete {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(delete, response_body("delete"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, item: TokenStream) -> TokenStream {
    delete_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP PATCH requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the PATCH HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/patch")]
/// struct Patch;
///
/// impl ServerHook for Patch {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(patch, response_body("patch"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn patch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    patch_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP HEAD requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the HEAD HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/head")]
/// struct Head;
///
/// impl ServerHook for Head {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(head, response_body("head"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn head(_attr: TokenStream, item: TokenStream) -> TokenStream {
    head_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP OPTIONS requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the OPTIONS HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/options")]
/// struct Options;
///
/// impl ServerHook for Options {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(options, response_body("options"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn options(_attr: TokenStream, item: TokenStream) -> TokenStream {
    options_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP CONNECT requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the CONNECT HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/connect")]
/// struct Connect;
///
/// impl ServerHook for Connect {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(connect, response_body("connect"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn connect(_attr: TokenStream, item: TokenStream) -> TokenStream {
    connect_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP TRACE requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the TRACE HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/trace")]
/// struct Trace;
///
/// impl ServerHook for Trace {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(trace, response_body("trace"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    trace_handler(item, Position::Prologue)
}

/// Allows function to handle multiple HTTP methods.
///
/// This attribute macro configures the decorated function to execute for any of the specified
/// HTTP methods. Methods should be provided as a comma-separated list.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/get_post")]
/// struct GetPost;
///
/// impl ServerHook for GetPost {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         http,
///         methods(get, post),
///         response_body("get_post")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a comma-separated list of HTTP method names (lowercase) and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    methods_macro(attr, item, Position::Prologue)
}

/// Restricts function execution to WebSocket upgrade requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// is a valid WebSocket upgrade request with proper request headers and protocol negotiation.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/ws")]
/// struct Websocket;
///
/// impl ServerHook for Websocket {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[ws]
///     #[ws_from_stream]
///     async fn handle(self, ctx: &Context) {
///         let body: RequestBody = ctx.get_request_body().await;
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         ctx.send_body_list_with_data(&body_list).await.unwrap();
///     }
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn ws(_attr: TokenStream, item: TokenStream) -> TokenStream {
    ws_macro(item, Position::Prologue)
}

/// Restricts function execution to standard HTTP requests only.
///
/// This attribute macro ensures the decorated function only executes for standard HTTP requests,
/// excluding WebSocket upgrades and other protocol upgrade requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http")]
/// struct HttpOnly;
///
/// impl ServerHook for HttpOnly {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http, response_body("http"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http_macro(item, Position::Prologue)
}

/// Sets the HTTP status code for the response.
///
/// This attribute macro configures the HTTP status code that will be sent with the response.
/// The status code can be provided as a numeric literal or a global constant.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// const CUSTOM_STATUS_CODE: i32 = 200;
///
/// #[route("/response")]
/// struct Response;
///
/// impl ServerHook for Response {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_status_code(CUSTOM_STATUS_CODE)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a numeric HTTP status code or a global constant
/// and should be applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn response_status_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_status_code_macro(attr, item, Position::Prologue)
}

/// Sets the HTTP reason phrase for the response.
///
/// This attribute macro configures the HTTP reason phrase that accompanies the status code.
/// The reason phrase can be provided as a string literal or a global constant.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// const CUSTOM_REASON: &str = "Accepted";
///
/// #[route("/response")]
/// struct Response;
///
/// impl ServerHook for Response {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_reason_phrase(CUSTOM_REASON)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a string literal or global constant for the reason phrase and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn response_reason_phrase(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_reason_phrase_macro(attr, item, Position::Prologue)
}

/// Sets or replaces a specific HTTP response header.
///
/// This attribute macro configures a specific HTTP response header that will be sent with the response.
/// Both the header name and value can be provided as string literals or global constants.
/// Use `"key", "value"` to set a header (add to existing headers) or `"key" => "value"` to replace a header (overwrite existing).
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// const CUSTOM_HEADER_NAME: &str = "X-Custom-Header";
/// const CUSTOM_HEADER_VALUE: &str = "custom-value";
///
/// #[route("/response")]
/// struct Response;
///
/// impl ServerHook for Response {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
///     async fn handle(self, ctx: &Context) {}
/// }
///
/// #[route("/response_header")]
/// struct ResponseHeaderTest;
///
/// impl ServerHook for ResponseHeaderTest {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body("Testing header set and replace operations")]
///     #[response_header("X-Add-Header", "add-value")]
///     #[response_header("X-Set-Header" => "set-value")]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts header name and header value, both can be string literals or global constants.
/// Use `"key", "value"` for setting headers and `"key" => "value"` for replacing headers.
/// Should be applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn response_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_header_macro(attr, item, Position::Prologue)
}

/// Sets the HTTP response body.
///
/// This attribute macro configures the HTTP response body that will be sent with the response.
/// The body content can be provided as a string literal or a global constant.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// const RESPONSE_DATA: &str = "{\"status\": \"success\"}";
///
/// #[route("/response")]
/// struct Response;
///
/// impl ServerHook for Response {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&RESPONSE_DATA)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a string literal or global constant for the response body and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn response_body(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_body_macro(attr, item, Position::Prologue)
}

/// Clears all response headers.
///
/// This attribute macro clears all response headers from the response.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/unknown_method")]
/// struct UnknownMethod;
///
/// impl ServerHook for UnknownMethod {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         clear_response_headers,
///         filter(ctx.get_request().await.is_unknown_method()),
///         response_body("unknown_method")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro should be applied to async functions that accept a `&Context` parameter.   
#[proc_macro_attribute]
pub fn clear_response_headers(_attr: TokenStream, item: TokenStream) -> TokenStream {
    clear_response_headers_macro(item, Position::Prologue)
}

/// Sets the HTTP response version.
///
/// This attribute macro configures the HTTP response version that will be sent with the response.
/// The version can be provided as a variable or code block.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[request_middleware]
/// struct RequestMiddleware;
///
/// impl ServerHook for RequestMiddleware {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(
///         response_status_code(200),
///         response_version(HttpVersion::HTTP1_1),
///         response_header(SERVER => HYPERLANE)
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable or code block for the response version and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn response_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_version_macro(attr, item, Position::Prologue)
}

/// Automatically sends the complete response after function execution.
///
/// This attribute macro ensures that the response (request headers and body) is automatically sent
/// to the client after the function completes execution.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware(2)]
/// struct ResponseMiddleware2;
///
/// impl ServerHook for ResponseMiddleware2 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send, flush)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_macro(item, Position::Epilogue)
}

/// Automatically sends only the response body after function execution.
///
/// This attribute macro ensures that only the response body is automatically sent
/// to the client after the function completes, handling request headers separately.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware("3")]
/// struct ResponseMiddleware3;
///
/// impl ServerHook for ResponseMiddleware3 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send_body, flush)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_body_macro(item, Position::Epilogue)
}

/// Sends the complete response with data after function execution.
///
/// This attribute macro ensures that the response (request headers and body) is automatically sent
/// to the client after the function completes execution, with the specified data.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/send_with_data")]
/// struct SendWithData;
///
/// impl ServerHook for SendWithData {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send_with_data("Hello, World!"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts data to send and should be applied to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send_with_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    send_with_data_macro(attr, item, Position::Epilogue)
}

/// Sends the complete response exactly once after function execution.
///
/// This attribute macro ensures that the response is sent exactly once to the client,
/// preventing multiple response transmissions for single-use scenarios.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/post")]
/// struct Post;
///
/// impl ServerHook for Post {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(post, response_body("post"), send_once)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send_once(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_once_macro(item, Position::Epilogue)
}

/// Sends only the response body exactly once after function execution.
///
/// This attribute macro ensures that the response body is sent exactly once to the client,
/// preventing multiple body transmissions for single-use scenarios.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/get")]
/// struct Get;
///
/// impl ServerHook for Get {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(get, response_body("get"), send_body_once)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send_body_once(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_body_once_macro(item, Position::Epilogue)
}

/// Sends the complete response exactly once with data after function execution.
///
/// This attribute macro ensures that the response is sent exactly once to the client,
/// preventing multiple response transmissions for single-use scenarios, with the specified data.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/send_once_with_data")]
/// struct SendOnceWithData;
///
/// impl ServerHook for SendOnceWithData {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send_once_with_data("One-time response"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts data to send and should be applied to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send_once_with_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    send_once_with_data_macro(attr, item, Position::Epilogue)
}

/// Flushes the response stream after function execution.
///
/// This attribute macro ensures that the response stream is flushed to guarantee immediate
/// data transmission, forcing any buffered response data to be sent to the client.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware(2)]
/// struct ResponseMiddleware2;
///
/// impl ServerHook for ResponseMiddleware2 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send, flush)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn flush(_attr: TokenStream, item: TokenStream) -> TokenStream {
    flush_macro(item, Position::Prologue)
}

/// Handles aborted request scenarios.
///
/// This attribute macro configures the function to handle cases where the client has
/// aborted the request, providing appropriate handling for interrupted or cancelled requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/aborted")]
/// struct Aborted;
///
/// impl ServerHook for Aborted {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[aborted]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn aborted(_attr: TokenStream, item: TokenStream) -> TokenStream {
    aborted_macro(item, Position::Prologue)
}

/// Handles closed connection scenarios.
///
/// This attribute macro configures the function to handle cases where the connection
/// has been closed, providing appropriate handling for terminated or disconnected connections.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/get_post")]
/// struct GetPost;
///
/// impl ServerHook for GetPost {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[closed]
///     #[prologue_macros(
///         http,
///         methods(get, post),
///         response_body("get_post")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn closed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    closed_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/2 Cleartext (h2c) requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/2 cleartext
/// requests that use the h2c upgrade mechanism.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/h2c")]
/// struct H2c;
///
/// impl ServerHook for H2c {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(h2c, response_body("h2c"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn h2c(_attr: TokenStream, item: TokenStream) -> TokenStream {
    h2c_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/0.9 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/0.9
/// protocol requests, the earliest version of the HTTP protocol.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http0_9")]
/// struct Http09;
///
/// impl ServerHook for Http09 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http0_9, response_body("http0_9"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http0_9(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http0_9_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/1.0 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.0
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http1_0")]
/// struct Http10;
///
/// impl ServerHook for Http10 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http1_0, response_body("http1_0"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http1_0(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_0_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/1.1 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.1
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http1_1")]
/// struct Http11;
///
/// impl ServerHook for Http11 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http1_1, response_body("http1_1"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http1_1(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_1_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/1.1 or higher protocol versions.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.1
/// or newer protocol versions, including HTTP/2, HTTP/3, and future versions.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http1_1_or_higher")]
/// struct Http11OrHigher;
///
/// impl ServerHook for Http11OrHigher {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http1_1_or_higher, response_body("http1_1_or_higher"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http1_1_or_higher(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_1_or_higher_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/2 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/2
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http2")]
/// struct Http2;
///
/// impl ServerHook for Http2 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http2, response_body("http2"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http2_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/3 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/3
/// protocol requests, the latest version of the HTTP protocol.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/http3")]
/// struct Http3;
///
/// impl ServerHook for Http3 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(http3, response_body("http3"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn http3(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http3_macro(item, Position::Prologue)
}

/// Restricts function execution to TLS-encrypted requests only.
///
/// This attribute macro ensures the decorated function only executes for requests
/// that use TLS/SSL encryption on the connection.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/tls")]
/// struct Tls;
///
/// impl ServerHook for Tls {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(tls, response_body("tls"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn tls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tls_macro(item, Position::Prologue)
}

/// Filters requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `true`.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/unknown_method")]
/// struct UnknownMethod;
///
/// impl ServerHook for UnknownMethod {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         filter(ctx.get_request().await.is_unknown_method()),
///         response_body("unknown_method")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn filter(attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_macro(attr, item, Position::Prologue)
}

/// Rejects requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `false`.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware(2)]
/// struct ResponseMiddleware2;
///
/// impl ServerHook for ResponseMiddleware2 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         reject(ctx.get_request().await.is_ws())
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn reject(attr: TokenStream, item: TokenStream) -> TokenStream {
    reject_macro(attr, item, Position::Prologue)
}

/// Restricts function execution to requests with a specific host.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// has a host header that matches the specified value. Requests with different or missing host headers will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/host")]
/// struct Host;
///
/// impl ServerHook for Host {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[host("localhost")]
///     #[prologue_macros(response_body("host string literal: localhost"), send)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a string literal specifying the expected host value and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn host(attr: TokenStream, item: TokenStream) -> TokenStream {
    host_macro(attr, item, Position::Prologue)
}

/// Reject requests that have no host header.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// has a host header present. Requests without a host header will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/reject_host")]
/// struct RejectHost;
///
/// impl ServerHook for RejectHost {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         reject_host("filter.localhost"),
///         response_body("host filter string literal")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn reject_host(attr: TokenStream, item: TokenStream) -> TokenStream {
    reject_host_macro(attr, item, Position::Prologue)
}

/// Restricts function execution to requests with a specific referer.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// has a referer header that matches the specified value. Requests with different or missing referer headers will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/referer")]
/// struct Referer;
///
/// impl ServerHook for Referer {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         referer("http://localhost"),
///         response_body("referer string literal: http://localhost")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a string literal specifying the expected referer value and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn referer(attr: TokenStream, item: TokenStream) -> TokenStream {
    referer_macro(attr, item, Position::Prologue)
}

/// Reject requests that have a specific referer header.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// does not have a referer header that matches the specified value. Requests with the matching referer header will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/reject_referer")]
/// struct RejectReferer;
///
/// impl ServerHook for RejectReferer {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         reject_referer("http://localhost"),
///         response_body("referer filter string literal")
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a string literal specifying the referer value to filter out and should be
/// applied to async functions that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn reject_referer(attr: TokenStream, item: TokenStream) -> TokenStream {
    reject_referer_macro(attr, item, Position::Prologue)
}

/// Executes multiple specified functions before the main handler function.
///
/// This attribute macro configures multiple pre-execution hooks that run before the main function logic.
/// The specified hook functions will be called in the order provided, followed by the main function execution.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// struct PrologueHooks;
///
/// impl ServerHook for PrologueHooks {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[get]
///     #[http]
///     async fn handle(self, _ctx: &Context) {}
/// }
///
/// async fn prologue_hooks_fn(ctx: Context) {
///     let hook = PrologueHooks::new(&ctx).await;
///     hook.handle(&ctx).await;
/// }
///
/// #[route("/hook")]
/// struct Hook;
///
/// impl ServerHook for Hook {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_hooks(prologue_hooks_fn)]
///     #[response_body("Testing hook macro")]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a comma-separated list of function names as parameters. All hook functions
/// and the main function must accept a `Context` parameter. Avoid combining this macro with other
/// macros on the same function to prevent macro expansion conflicts.
#[proc_macro_attribute]
pub fn prologue_hooks(attr: TokenStream, item: TokenStream) -> TokenStream {
    prologue_hooks_macro(attr, item, Position::Prologue)
}

/// Executes multiple specified functions after the main handler function.
///
/// This attribute macro configures multiple post-execution hooks that run after the main function logic.
/// The main function will execute first, followed by the specified hook functions in the order provided.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// struct EpilogueHooks;
///
/// impl ServerHook for EpilogueHooks {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_status_code(200)]
///     async fn handle(self, ctx: &Context) {}
/// }
///
/// async fn epilogue_hooks_fn(ctx: Context) {
///     let hook = EpilogueHooks::new(&ctx).await;
///     hook.handle(&ctx).await;
/// }
///
/// #[route("/hook")]
/// struct Hook;
///
/// impl ServerHook for Hook {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_hooks(epilogue_hooks_fn)]
///     #[response_body("Testing hook macro")]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a comma-separated list of function names as parameters. All hook functions
/// and the main function must accept a `Context` parameter. Avoid combining this macro with other
/// macros on the same function to prevent macro expansion conflicts.
#[proc_macro_attribute]
pub fn epilogue_hooks(attr: TokenStream, item: TokenStream) -> TokenStream {
    epilogue_hooks_macro(attr, item, Position::Epilogue)
}

/// Extracts the raw request body into a specified variable.
///
/// This attribute macro extracts the raw request body content into a variable
/// with the fixed type `RequestBody`. The body content is not parsed or deserialized.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_body")]
/// struct RequestBodyRoute;
///
/// impl ServerHook for RequestBodyRoute {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("raw body: {raw_body:?}"))]
///     #[request_body(raw_body)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts only a variable name. The variable will be available
/// in the function scope as a `RequestBody` type.
#[proc_macro_attribute]
pub fn request_body(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_body_macro(attr, item, Position::Prologue)
}

/// Parses the request body as JSON into a specified variable and type.
///
/// This attribute macro extracts and deserializes the request body content as JSON into a variable
/// with the specified type. The body content is parsed as JSON using serde.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize, Clone)]
/// struct TestData {
///     name: String,
///     age: u32,
/// }
///
/// #[route("/request_body_json")]
/// struct RequestBodyJson;
///
/// impl ServerHook for RequestBodyJson {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request data: {request_data_result:?}"))]
///     #[request_body_json(request_data_result: TestData)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name and type in the format `variable_name: Type`.
/// The variable will be available in the function scope as a `Result<Type, JsonError>`.
#[proc_macro_attribute]
pub fn request_body_json(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_body_json_macro(attr, item, Position::Prologue)
}

/// Extracts a specific attribute value into a variable.
///
/// This attribute macro retrieves a specific attribute by key and makes it available
/// as a typed variable from the request context.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// const TEST_ATTRIBUTE_KEY: &str = "test_attribute_key";
///
/// #[derive(Debug, Serialize, Deserialize, Clone)]
/// struct TestData {
///     name: String,
///     age: u32,
/// }
///
/// #[route("/attribute")]
/// struct Attribute;
///
/// impl ServerHook for Attribute {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request attribute: {request_attribute_option:?}"))]
///     #[attribute(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `key => variable_name: Type`.
/// The variable will be available as an `Option<Type>` in the function scope.
#[proc_macro_attribute]
pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute_macro(attr, item, Position::Prologue)
}

/// Extracts all attributes into a HashMap variable.
///
/// This attribute macro retrieves all available attributes from the request context
/// and makes them available as a HashMap for comprehensive attribute access.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/attributes")]
/// struct Attributes;
///
/// impl ServerHook for Attributes {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request attributes: {request_attributes:?}"))]
///     #[attributes(request_attributes)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain a HashMap of all attributes.
/// The variable will be available as a HashMap in the function scope.
#[proc_macro_attribute]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes_macro(attr, item, Position::Prologue)
}

/// Extracts a specific route parameter into a variable.
///
/// This attribute macro retrieves a specific route parameter by key and makes it
/// available as a variable. Route parameters are extracted from the URL path segments.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/route_param/:test")]
/// struct RouteParam;
///
/// impl ServerHook for RouteParam {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("route param: {request_route_param:?}"))]
///     #[route_param("test" => request_route_param)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `Option<String>` in the function scope.
#[proc_macro_attribute]
pub fn route_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_param_macro(attr, item, Position::Prologue)
}

/// Extracts all route parameters into a collection variable.
///
/// This attribute macro retrieves all available route parameters from the URL path
/// and makes them available as a collection for comprehensive route parameter access.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/route_params/:test")]
/// struct RouteParams;
///
/// impl ServerHook for RouteParams {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request route params: {request_route_params:?}"))]
///     #[route_params(request_route_params)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain all route parameters.
/// The variable will be available as a collection in the function scope.
#[proc_macro_attribute]
pub fn route_params(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_params_macro(attr, item, Position::Prologue)
}

/// Extracts a specific request query parameter into a variable.
///
/// This attribute macro retrieves a specific request query parameter by key and makes it
/// available as a variable. Query parameters are extracted from the URL request query string.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_query")]
/// struct RequestQuery;
///
/// impl ServerHook for RequestQuery {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_query("test" => request_query_option),
///         response_body(&format!("request query: {request_query_option:?}")),
///         send
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `Option<String>` in the function scope.
#[proc_macro_attribute]
pub fn request_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_query_macro(attr, item, Position::Prologue)
}

/// Extracts all request query parameters into a collection variable.
///
/// This attribute macro retrieves all available request query parameters from the URL request query string
/// and makes them available as a collection for comprehensive request query parameter access.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_querys")]
/// struct RequestQuerys;
///
/// impl ServerHook for RequestQuerys {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_querys(request_querys),
///         response_body(&format!("request querys: {request_querys:?}")),
///         send
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain all request query parameters.
/// The variable will be available as a collection in the function scope.
#[proc_macro_attribute]
pub fn request_querys(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_querys_macro(attr, item, Position::Prologue)
}

/// Extracts a specific HTTP request header into a variable.
///
/// This attribute macro retrieves a specific HTTP request header by name and makes it
/// available as a variable. Header values are extracted from the request request headers collection.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_header")]
/// struct RequestHeader;
///
/// impl ServerHook for RequestHeader {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_header(HOST => request_header_option),
///         response_body(&format!("request header: {request_header_option:?}")),
///         send
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a request header name-to-variable mapping in the format `HEADER_NAME => variable_name`
/// or `"Header-Name" => variable_name`. The variable will be available as an `Option<String>`.
#[proc_macro_attribute]
pub fn request_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_header_macro(attr, item, Position::Prologue)
}

/// Extracts all HTTP request headers into a collection variable.
///
/// This attribute macro retrieves all available HTTP request headers from the request
/// and makes them available as a collection for comprehensive request header access.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_headers")]
/// struct RequestHeaders;
///
/// impl ServerHook for RequestHeaders {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_headers(request_headers),
///         response_body(&format!("request headers: {request_headers:?}")),
///         send
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain all HTTP request headers.
/// The variable will be available as a collection in the function scope.
#[proc_macro_attribute]
pub fn request_headers(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_headers_macro(attr, item, Position::Prologue)
}

/// Extracts a specific cookie value or all cookies into a variable.
///
/// This attribute macro supports two syntaxes:
/// 1. `cookie(key => variable_name)` - Extract a specific cookie value by key
/// 2. `cookie(variable_name)` - Extract all cookies as a raw string
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/cookie")]
/// struct Cookie;
///
/// impl ServerHook for Cookie {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Session cookie: {session_cookie_opt:?}"))]
///     #[request_cookie("test" => session_cookie_opt)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// For specific cookie extraction, the variable will be available as `Option<String>`.
/// For all cookies extraction, the variable will be available as `String`.
#[proc_macro_attribute]
pub fn request_cookie(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_cookie_macro(attr, item, Position::Prologue)
}

/// Extracts all cookies as a raw string into a variable.
///
/// This attribute macro retrieves the entire Cookie header from the request and makes it
/// available as a String variable. If no Cookie header is present, an empty string is used.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/cookies")]
/// struct Cookies;
///
/// impl ServerHook for Cookies {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("All cookies: {cookie_value:?}"))]
///     #[request_cookies(cookie_value)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain the Cookie header value.
/// The variable will be available as a String in the function scope.
#[proc_macro_attribute]
pub fn request_cookies(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_cookies_macro(attr, item, Position::Prologue)
}

/// Extracts the HTTP request version into a variable.
///
/// This attribute macro retrieves the HTTP version from the request and makes it
/// available as a variable. The version represents the HTTP protocol version used.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_version")]
/// struct RequestVersionTest;
///
/// impl ServerHook for RequestVersionTest {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("HTTP Version: {http_version}"))]
///     #[request_version(http_version)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain the HTTP request version.
/// The variable will be available as a RequestVersion type in the function scope.
#[proc_macro_attribute]
pub fn request_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_version_macro(attr, item, Position::Prologue)
}

/// Extracts the HTTP request path into a variable.
///
/// This attribute macro retrieves the request path from the HTTP request and makes it
/// available as a variable. The path represents the URL path portion of the request.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_path")]
/// struct RequestPathTest;
///
/// impl ServerHook for RequestPathTest {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Request Path: {request_path}"))]
///     #[request_path(request_path)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts a variable name that will contain the HTTP request path.
/// The variable will be available as a RequestPath type in the function scope.
#[proc_macro_attribute]
pub fn request_path(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_path_macro(attr, item, Position::Prologue)
}

/// Creates a new instance of a specified type with a given variable name.
///
/// This attribute macro generates an instance initialization at the beginning of the function.
///
/// # Usage
///
/// ```rust,no_run
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[hyperlane(server: Server)]
/// #[hyperlane(config: ServerConfig)]
/// #[tokio::main]
/// async fn main() {
///     config.disable_nodelay().await;
///     server.config(config).await;
///     let server_hook: ServerControlHook = server.run().await.unwrap_or_default();
///     server_hook.wait().await;
/// }
/// ```
///
/// The macro accepts a `variable_name: Type` pair.
/// The variable will be available as an instance of the specified type in the function scope.
#[proc_macro_attribute]
pub fn hyperlane(attr: TokenStream, item: TokenStream) -> TokenStream {
    hyperlane_macro(attr, item)
}

/// Registers a function as a route handler.
///
/// This attribute macro registers the decorated function as a route handler for a given path.
/// This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/response")]
/// struct Response;
///
/// impl ServerHook for Response {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[response_body("response")]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// # Parameters
///
/// - `path`: String literal defining the route path
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_macro(attr, item)
}

/// Registers a function as a request middleware.
///
/// This attribute macro registers the decorated function to be executed as a middleware
/// for incoming requests. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[request_middleware]
/// struct RequestMiddleware;
///
/// impl ServerHook for RequestMiddleware {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(
///         response_status_code(200),
///         response_version(HttpVersion::HTTP1_1),
///         response_header(SERVER => HYPERLANE)
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn request_middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_middleware_macro(attr, item)
}

/// Registers a function as a response middleware.
///
/// This attribute macro registers the decorated function to be executed as a middleware
/// for outgoing responses. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware]
/// struct ResponseMiddleware1;
///
/// impl ServerHook for ResponseMiddleware1 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn response_middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_middleware_macro(attr, item)
}

/// Registers a function as a panic hook.
///
/// This attribute macro registers the decorated function to handle panics that occur
/// during request processing. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[panic_hook]
/// #[panic_hook(1)]
/// #[panic_hook("2")]
/// struct PanicHook;
///
/// impl ServerHook for PanicHook {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(response_body("panic_hook"), send)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn panic_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    panic_hook_macro(attr, item)
}

/// Injects a list of macros before the decorated function.
///
/// The macros are applied in head-insertion order, meaning the first macro in the list
/// is the outermost macro.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/post")]
/// struct Post;
///
/// impl ServerHook for Post {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(post, response_body("post"), send_once)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn prologue_macros(attr: TokenStream, item: TokenStream) -> TokenStream {
    prologue_macros_macro(attr, item)
}

/// Injects a list of macros after the decorated function.
///
/// The macros are applied in tail-insertion order, meaning the last macro in the list
/// is the outermost macro.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware(2)]
/// struct ResponseMiddleware2;
///
/// impl ServerHook for ResponseMiddleware2 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send, flush)]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn epilogue_macros(attr: TokenStream, item: TokenStream) -> TokenStream {
    epilogue_macros_macro(attr, item)
}

/// Sends only the response body with data after function execution.
///
/// This attribute macro ensures that only the response body is automatically sent
/// to the client after the function completes, handling request headers separately,
/// with the specified data.
///
/// # Usage
///
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/send_body_with_data")]
/// struct SendBodyWithData;
///
/// impl ServerHook for SendBodyWithData {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send_body_with_data("Response body content"))]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// The macro accepts data to send and should be applied to async functions
/// that accept a `&Context` parameter.
#[proc_macro_attribute]
pub fn send_body_with_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    send_body_with_data_macro(attr, item, Position::Epilogue)
}

/// Wraps function body with WebSocket stream processing.
///
/// This attribute macro generates code that wraps the function body with a check to see if
/// data can be read from a WebSocket stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// This attribute macro generates code that wraps the function body with a check to see if
/// data can be read from a WebSocket stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `TokenStream`: The buffer to read from the WebSocket stream.
/// - `TokenStream`: The function item to be modified
///
/// # Returns
///
/// Returns a TokenStream containing the modified function with WebSocket stream processing logic.
///
/// # Examples
///
/// Using no parameters (default buffer size):
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/ws1")]
/// struct Websocket1;
///
/// impl ServerHook for Websocket1 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[ws]
///     #[ws_from_stream]
///     async fn handle(self, ctx: &Context) {
///         let body: RequestBody = ctx.get_request_body().await;
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         ctx.send_body_list_with_data(&body_list).await.unwrap();
///     }
/// }
/// ```
///
/// Using only buffer size:
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/ws5")]
/// struct Websocket5;
///
/// impl ServerHook for Websocket5 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[ws]
///     #[ws_from_stream(1024)]
///     async fn handle(self, ctx: &Context) {
///         let body: RequestBody = ctx.get_request_body().await;
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         ctx.send_body_list_with_data(&body_list).await.unwrap();
///     }
/// }
/// ```
///
/// Using variable name to store request data:
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/ws2")]
/// struct Websocket2;
///
/// impl ServerHook for Websocket2 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[ws]
///     #[ws_from_stream(request)]
///     async fn handle(self, ctx: &Context) {
///         let body: &RequestBody = &request.get_body();
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
///         ctx.send_body_list_with_data(&body_list).await.unwrap();
///     }
/// }
/// ```
///
/// Using buffer size and variable name:
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/ws3")]
/// struct Websocket3;
///
/// impl ServerHook for Websocket3 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[ws]
///     #[ws_from_stream(1024, request)]
///     async fn handle(self, ctx: &Context) {
///         let body: &RequestBody = request.get_body();
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         ctx.send_body_list_with_data(&body_list).await.unwrap();
///     }
/// }
/// ```
///
/// Using variable name and buffer size (reversed order):
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/ws4")]
/// struct Websocket4;
///
/// impl ServerHook for Websocket4 {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[ws]
///     #[ws_from_stream(request, 1024)]
///     async fn handle(self, ctx: &Context) {
///         let body: &RequestBody = request.get_body();
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         ctx.send_body_list_with_data(&body_list).await.unwrap();
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn ws_from_stream(attr: TokenStream, item: TokenStream) -> TokenStream {
    ws_from_stream_macro(attr, item)
}

/// Wraps function body with HTTP stream processing.
///
/// This attribute macro generates code that wraps the function body with a check to see if
/// data can be read from an HTTP stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// This attribute macro generates code that wraps the function body with a check to see if
/// data can be read from an HTTP stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `TokenStream`: The buffer to read from the HTTP stream.
/// - `TokenStream`: The function item to be modified
///
/// # Returns
///
/// Returns a TokenStream containing the modified function with HTTP stream processing logic.
///
/// # Examples
///
/// Using with epilogue_macros:
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_query")]
/// struct RequestQuery;
///
/// impl ServerHook for RequestQuery {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(
///         request_query("test" => request_query_option),
///         response_body(&format!("request query: {request_query_option:?}")),
///         send,
///         http_from_stream(1024)
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
///
/// Using with variable name:
/// ```rust
/// use hyperlane::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_header")]
/// struct RequestHeader;
///
/// impl ServerHook for RequestHeader {
///     async fn new(_ctx: &Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(
///         request_header(HOST => request_header_option),
///         response_body(&format!("request header: {request_header_option:?}")),
///         send,
///         http_from_stream(_request)
///     )]
///     async fn handle(self, ctx: &Context) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn http_from_stream(attr: TokenStream, item: TokenStream) -> TokenStream {
    http_from_stream_macro(attr, item)
}
