//! # Hyperlane Macros
//!
//! A comprehensive collection of procedural macros for building HTTP servers with enhanced functionality.
//! This crate provides attribute macros that simplify HTTP request handling, protocol validation,
//! response management, and request data extraction.
//!
//! ## Features
//!
//! - **HTTP Method Handlers**: Macros for all standard HTTP methods (GET, POST, PUT, DELETE, etc.)
//! - **Protocol Validation**: Support for HTTP versions, WebSocket, TLS, and HTTP/2 cleartext
//! - **Response Management**: Status code setting, reason phrase customization, and response sending
//! - **Request Data Extraction**: Body parsing, request header extraction, request query parameters, and route parameters
//! - **Filter Mechanisms**: Unknown method, upgrade, and version filtering
//! - **Hook System**: Pre and post execution hooks for request processing
//! - **Stream Management**: Connection state handling for aborted and closed streams
//!
//! ## Dependencies
//!
//! To use this crate, add the following dependencies to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hyperlane-macros = "*"
//! hyperlane = "*"
//! serde = { version = "*", features = ["derive"] }
//! ```
//!
//! ## Usage Guidelines
//!
//! All macros are designed to be used as attribute macros on async functions that accept a `Context` parameter.
//! Multiple macros can be combined on a single function to create complex request handling logic.
//! When using hook macros (`pre_hook`, `post_hook`), avoid combining them with other macros on the same function
//! to prevent unexpected behavior during macro expansion.

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

/// Restricts function execution to HTTP GET requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the GET HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[get]
/// async fn handle_get(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    get_handler(item)
}

/// Restricts function execution to HTTP POST requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the POST HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[post]
/// async fn handle_post(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn post(_attr: TokenStream, item: TokenStream) -> TokenStream {
    post_handler(item)
}

/// Restricts function execution to HTTP PUT requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the PUT HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[put]
/// async fn handle_put(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn put(_attr: TokenStream, item: TokenStream) -> TokenStream {
    put_handler(item)
}

/// Restricts function execution to HTTP DELETE requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the DELETE HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[delete]
/// async fn handle_delete(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, item: TokenStream) -> TokenStream {
    delete_handler(item)
}

/// Restricts function execution to HTTP PATCH requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the PATCH HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[patch]
/// async fn handle_patch(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn patch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    patch_handler(item)
}

/// Restricts function execution to HTTP HEAD requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the HEAD HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[head]
/// async fn handle_head(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn head(_attr: TokenStream, item: TokenStream) -> TokenStream {
    head_handler(item)
}

/// Restricts function execution to HTTP OPTIONS requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the OPTIONS HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[options]
/// async fn handle_options(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn options(_attr: TokenStream, item: TokenStream) -> TokenStream {
    options_handler(item)
}

/// Restricts function execution to HTTP CONNECT requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the CONNECT HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[connect]
/// async fn handle_connect(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn connect(_attr: TokenStream, item: TokenStream) -> TokenStream {
    connect_handler(item)
}

/// Restricts function execution to HTTP TRACE requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the TRACE HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[trace]
/// async fn handle_trace(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    trace_handler(item)
}

/// Allows function to handle multiple HTTP methods.
///
/// This attribute macro configures the decorated function to execute for any of the specified
/// HTTP methods. Methods should be provided as a comma-separated list.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[methods(get, post)]
/// async fn handle_get_post(ctx: Context) {
///     // Function body
/// }
///
/// #[methods(put, patch, delete)]
/// async fn handle_modifications(ctx: Context) {
///     // Function body
/// }
/// ```
///
/// The macro accepts a comma-separated list of HTTP method names (lowercase) and should be
/// applied to async functions that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    methods_macro(attr, item)
}

/// Restricts function execution to WebSocket upgrade requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// is a valid WebSocket upgrade request with proper request headers and protocol negotiation.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[ws]
/// async fn handle_websocket(ctx: Context) {
///     // WebSocket handling logic
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn ws(_attr: TokenStream, item: TokenStream) -> TokenStream {
    ws_macro(item)
}

/// Restricts function execution to standard HTTP requests only.
///
/// This attribute macro ensures the decorated function only executes for standard HTTP requests,
/// excluding WebSocket upgrades and other protocol upgrade requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http]
/// async fn handle_http(ctx: Context) {
///     // HTTP request handling logic
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http_macro(item)
}

/// Sets the HTTP status code for the response.
///
/// This attribute macro configures the HTTP status code that will be sent with the response.
/// The status code should be provided as a numeric value.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[status_code(200)]
/// async fn success_handler(ctx: Context) {
///     // Response will have status code 200
/// }
///
/// #[status_code(404)]
/// async fn not_found_handler(ctx: Context) {
///     // Response will have status code 404
/// }
///
/// #[status_code(500)]
/// async fn error_handler(ctx: Context) {
///     // Response will have status code 500
/// }
/// ```
///
/// The macro accepts a numeric HTTP status code (e.g., 200, 404, 500) and should be
/// applied to async functions that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn status_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    code_macro(attr, item)
}

/// Sets the HTTP reason phrase for the response.
///
/// This attribute macro configures the HTTP reason phrase that accompanies the status code.
/// The reason phrase should be provided as a string literal.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[reason_phrase("OK")]
/// async fn success_handler(ctx: Context) {
///     // Response will have reason phrase "OK"
/// }
///
/// #[reason_phrase("Not Found")]
/// async fn not_found_handler(ctx: Context) {
///     // Response will have reason phrase "Not Found"
/// }
///
/// #[reason_phrase("Internal Server Error")]
/// async fn error_handler(ctx: Context) {
///     // Response will have reason phrase "Internal Server Error"
/// }
/// ```
///
/// The macro accepts a string literal for the reason phrase and should be
/// applied to async functions that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn reason_phrase(attr: TokenStream, item: TokenStream) -> TokenStream {
    reason_phrase_macro(attr, item)
}

/// Automatically sends the complete response after function execution.
///
/// This attribute macro ensures that the response (request headers and body) is automatically sent
/// to the client after the function completes execution.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[send]
/// async fn auto_send_handler(ctx: Context) {
///     let _ = ctx.set_response_body("Hello World").await;
///     // Response is automatically sent after function returns
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn send(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_macro(item)
}

/// Automatically sends only the response body after function execution.
///
/// This attribute macro ensures that only the response body is automatically sent
/// to the client after the function completes, handling request headers separately.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[send_body]
/// async fn auto_send_body_handler(ctx: Context) {
///     let _ = ctx.set_response_body("Response body content").await;
///     // Only response body is automatically sent after function returns
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn send_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_body_macro(item)
}

/// Sends the complete response exactly once after function execution.
///
/// This attribute macro ensures that the response is sent exactly once to the client,
/// preventing multiple response transmissions for single-use scenarios.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[send_once]
/// async fn send_once_handler(ctx: Context) {
///     let _ = ctx.set_response_body("One-time response").await;
///     // Response is sent exactly once after function returns
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn send_once(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_once_macro(item)
}

/// Sends only the response body exactly once after function execution.
///
/// This attribute macro ensures that the response body is sent exactly once to the client,
/// preventing multiple body transmissions for single-use scenarios.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[send_once_body]
/// async fn send_once_body_handler(ctx: Context) {
///     let _ = ctx.set_response_body("One-time body content").await;
///     // Response body is sent exactly once after function returns
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn send_once_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    send_once_body_macro(item)
}

/// Flushes the response stream after function execution.
///
/// This attribute macro ensures that the response stream is flushed to guarantee immediate
/// data transmission, forcing any buffered response data to be sent to the client.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[flush]
/// async fn flush_handler(ctx: Context) {
///     let _ = ctx.set_response_body("Immediate response").await;
///     // Response stream is flushed after function returns
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn flush(_attr: TokenStream, item: TokenStream) -> TokenStream {
    flush_macro(item)
}

/// Handles aborted request scenarios.
///
/// This attribute macro configures the function to handle cases where the client has
/// aborted the request, providing appropriate handling for interrupted or cancelled requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[aborted]
/// async fn handle_aborted(ctx: Context) {
///     // Handle aborted request logic
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn aborted(_attr: TokenStream, item: TokenStream) -> TokenStream {
    aborted_macro(item)
}

/// Handles closed connection scenarios.
///
/// This attribute macro configures the function to handle cases where the connection
/// has been closed, providing appropriate handling for terminated or disconnected connections.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[closed]
/// async fn handle_closed(ctx: Context) {
///     // Handle closed connection logic
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn closed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    closed_macro(item)
}

/// Restricts function execution to HTTP/2 Cleartext (h2c) requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/2 cleartext
/// requests that use the h2c upgrade mechanism.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[h2c]
/// async fn handle_h2c(ctx: Context) {
///     // Handle HTTP/2 cleartext requests
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn h2c(_attr: TokenStream, item: TokenStream) -> TokenStream {
    h2c_macro(item)
}

/// Restricts function execution to HTTP/0.9 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/0.9
/// protocol requests, the earliest version of the HTTP protocol.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http0_9]
/// async fn handle_http09(ctx: Context) {
///     // Handle HTTP/0.9 requests
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http0_9(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http0_9_macro(item)
}

/// Restricts function execution to HTTP/1.0 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.0
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http1_0]
/// async fn handle_http10(ctx: Context) {
///     // Handle HTTP/1.0 requests
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http1_0(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_0_macro(item)
}

/// Restricts function execution to HTTP/1.1 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.1
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http1_1]
/// async fn handle_http11(ctx: Context) {
///     // Handle HTTP/1.1 requests
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http1_1(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_1_macro(item)
}

/// Restricts function execution to HTTP/1.1 or higher protocol versions.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.1
/// or newer protocol versions, including HTTP/2, HTTP/3, and future versions.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http1_1_or_higher]
/// async fn handle_modern_http(ctx: Context) {
///     // Handle HTTP/1.1, HTTP/2, HTTP/3, etc.
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http1_1_or_higher(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http1_1_or_higher_macro(item)
}

/// Restricts function execution to HTTP/2 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/2
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http2]
/// async fn handle_http2(ctx: Context) {
///     // Handle HTTP/2 requests
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http2_macro(item)
}

/// Restricts function execution to HTTP/3 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/3
/// protocol requests, the latest version of the HTTP protocol.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[http3]
/// async fn handle_http3(ctx: Context) {
///     // Handle HTTP/3 requests
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn http3(_attr: TokenStream, item: TokenStream) -> TokenStream {
    http3_macro(item)
}

/// Restricts function execution to TLS-encrypted requests only.
///
/// This attribute macro ensures the decorated function only executes for requests
/// that use TLS/SSL encryption on the connection.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[tls]
/// async fn handle_secure(ctx: Context) {
///     // Handle TLS-encrypted requests only
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn tls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tls_macro(item)
}

/// Handles requests with unknown or non-standard HTTP methods.
///
/// This attribute macro configures the function to handle requests that use
/// unrecognized or unsupported HTTP methods, providing a fallback for non-standard methods.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[filter_unknown_method]
/// async fn handle_unknown_method(ctx: Context) {
///     // Handle requests with unknown HTTP methods
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn filter_unknown_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_method_macro(item)
}

/// Handles requests with unknown or non-standard upgrade protocols.
///
/// This attribute macro configures the function to handle requests that specify
/// unrecognized upgrade protocols, providing a fallback for non-standard upgrade request headers.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[filter_unknown_upgrade]
/// async fn handle_unknown_upgrade(ctx: Context) {
///     // Handle requests with unknown upgrade protocols
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn filter_unknown_upgrade(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_upgrade_macro(item)
}

/// Handles requests with unknown or non-standard HTTP versions.
///
/// This attribute macro configures the function to handle requests that use
/// unrecognized HTTP protocol versions, providing a fallback for non-standard versions.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[filter_unknown_version]
/// async fn handle_unknown_version(ctx: Context) {
///     // Handle requests with unknown HTTP versions
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn filter_unknown_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_version_macro(item)
}

/// Handles requests with any unknown characteristics.
///
/// This attribute macro combines filtering for unknown methods, upgrade protocols, and HTTP versions,
/// providing comprehensive handling for requests with any unrecognized characteristics.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[filter_unknown]
/// async fn handle_all_unknown(ctx: Context) {
///     // Handle requests with any unknown characteristics
/// }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `Context` parameter.
#[proc_macro_attribute]
pub fn filter_unknown(_attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_unknown_macro(item)
}

/// Executes a specified function before the main handler function.
///
/// This attribute macro configures a pre-execution hook that runs before the main function logic.
/// The specified hook function will be called first, followed by the main function execution.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[get]
/// async fn pre_handler(ctx: Context) {
///     // Pre-execution logic
/// }
///
/// #[pre_hook(pre_handler)]
/// async fn main_handler(ctx: Context) {
///     // Main function logic (runs after pre_handler)
/// }
/// ```
///
/// The macro accepts a function name as parameter. Both the hook function and main function
/// must accept a `Context` parameter. Avoid combining this macro with other macros on the
/// same function to prevent macro expansion conflicts.
#[proc_macro_attribute]
pub fn pre_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    pre_hook_macro(attr, item)
}

/// Executes a specified function after the main handler function.
///
/// This attribute macro configures a post-execution hook that runs after the main function logic.
/// The main function will execute first, followed by the specified hook function.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[send]
/// async fn post_handler(ctx: Context) {
///     // Post-execution logic
/// }
///
/// #[post_hook(post_handler)]
/// async fn main_handler(ctx: Context) {
///     // Main function logic (runs before post_handler)
/// }
/// ```
///
/// The macro accepts a function name as parameter. Both the hook function and main function
/// must accept a `Context` parameter. Avoid combining this macro with other macros on the
/// same function to prevent macro expansion conflicts.
#[proc_macro_attribute]
pub fn post_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    post_hook_macro(attr, item)
}

/// Parses the request body into a specified variable and type.
///
/// This attribute macro extracts and deserializes the request body content into a variable
/// with the specified type. The body content is typically parsed as JSON.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Clone)]
/// struct UserData {
///     name: String,
///     age: u32,
/// }
///
/// #[body(user_data: UserData)]
/// async fn handle_user(ctx: Context) {
///     if let Ok(data) = user_data {
///         // Use the parsed user data
///     }
/// }
/// ```
///
/// The macro accepts a variable name and type in the format `variable_name: Type`.
/// The variable will be available in the function scope as a `Result<Type, Error>`.
#[proc_macro_attribute]
pub fn body(attr: TokenStream, item: TokenStream) -> TokenStream {
    body_macro(attr, item)
}

/// Extracts a specific attribute value into a variable.
///
/// This attribute macro retrieves a specific attribute by key and makes it available
/// as a typed variable from the request context.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
/// use serde::Deserialize;
///
/// const USER_KEY: &str = "user_data";
///
/// #[derive(Deserialize, Clone)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// #[attribute(USER_KEY => user: User)]
/// async fn handle_with_attribute(ctx: Context) {
///     if let Some(user_data) = user {
///         // Use the extracted attribute
///     }
/// }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `key => variable_name: Type`.
/// The variable will be available as an `Option<Type>` in the function scope.
#[proc_macro_attribute]
pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute_macro(attr, item)
}

/// Extracts all attributes into a HashMap variable.
///
/// This attribute macro retrieves all available attributes from the request context
/// and makes them available as a HashMap for comprehensive attribute access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[attributes(all_attrs)]
/// async fn handle_with_all_attributes(ctx: Context) {
///     for (key, value) in all_attrs {
///         // Process each attribute
///     }
/// }
/// ```
///
/// The macro accepts a variable name that will contain a HashMap of all attributes.
/// The variable will be available as a HashMap in the function scope.
#[proc_macro_attribute]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes_macro(attr, item)
}

/// Extracts a specific route parameter into a variable.
///
/// This attribute macro retrieves a specific route parameter by key and makes it
/// available as a variable. Route parameters are extracted from the URL path segments.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// // For route like "/users/{id}"
/// #[route_param("id" => user_id)]
/// async fn get_user(ctx: Context) {
///     if let Some(id) = user_id {
///         // Use the route parameter
///     }
/// }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `Option<String>` in the function scope.
#[proc_macro_attribute]
pub fn route_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_param_macro(attr, item)
}

/// Extracts all route parameters into a collection variable.
///
/// This attribute macro retrieves all available route parameters from the URL path
/// and makes them available as a collection for comprehensive route parameter access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// // For route like "/users/{id}/posts/{post_id}"
/// #[route_params(params)]
/// async fn handle_nested_route(ctx: Context) {
///     for (key, value) in params {
///         // Process each route parameter
///     }
/// }
/// ```
///
/// The macro accepts a variable name that will contain all route parameters.
/// The variable will be available as a collection in the function scope.
#[proc_macro_attribute]
pub fn route_params(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_params_macro(attr, item)
}

/// Extracts a specific request query parameter into a variable.
///
/// This attribute macro retrieves a specific request query parameter by key and makes it
/// available as a variable. Query parameters are extracted from the URL request query string.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// // For URL like "/search?q=rust&limit=10"
/// #[request_query("q" => search_term)]
/// async fn search(ctx: Context) {
///     if let Some(term) = search_term {
///         // Use the request query parameter
///     }
/// }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `Option<String>` in the function scope.
#[proc_macro_attribute]
pub fn request_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_query_macro(attr, item)
}

/// Extracts all request query parameters into a collection variable.
///
/// This attribute macro retrieves all available request query parameters from the URL request query string
/// and makes them available as a collection for comprehensive request query parameter access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// // For URL like "/search?q=rust&limit=10&sort=date"
/// #[request_querys(all_params)]
/// async fn search_with_params(ctx: Context) {
///     for (key, value) in all_params {
///         // Process each request query parameter
///     }
/// }
/// ```
///
/// The macro accepts a variable name that will contain all request query parameters.
/// The variable will be available as a collection in the function scope.
#[proc_macro_attribute]
pub fn request_querys(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_querys_macro(attr, item)
}

/// Extracts a specific HTTP request header into a variable.
///
/// This attribute macro retrieves a specific HTTP request header by name and makes it
/// available as a variable. Header values are extracted from the request request headers collection.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[request_header(HOST => host_request_header)]
/// async fn handle_with_host(ctx: Context) {
///     if let Some(host) = host_request_header {
///         // Use the host request_header value
///     }
/// }
///
/// #[request_header("Content-Type" => content_type)]
/// async fn handle_with_content_type(ctx: Context) {
///     if let Some(ct) = content_type {
///         // Use the content type request_header
///     }
/// }
/// ```
///
/// The macro accepts a request header name-to-variable mapping in the format `HEADER_NAME => variable_name`
/// or `"Header-Name" => variable_name`. The variable will be available as an `Option<String>`.
#[proc_macro_attribute]
pub fn request_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_header_macro(attr, item)
}

/// Extracts all HTTP request headers into a collection variable.
///
/// This attribute macro retrieves all available HTTP request headers from the request
/// and makes them available as a collection for comprehensive request header access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_macros::*;
/// use hyperlane::*;
///
/// #[request_headers(all_request_headers)]
/// async fn handle_with_all_request_headers(ctx: Context) {
///     for (name, value) in all_request_headers {
///         // Process each request_header
///     }
/// }
/// ```
///
/// The macro accepts a variable name that will contain all HTTP request headers.
/// The variable will be available as a collection in the function scope.
#[proc_macro_attribute]
pub fn request_headers(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_headers_macro(attr, item)
}
