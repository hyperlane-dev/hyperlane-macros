<center>

## hyperlane-macros

[![](https://img.shields.io/crates/v/hyperlane-macros.svg)](https://crates.io/crates/hyperlane-macros)
[![](https://img.shields.io/crates/d/hyperlane-macros.svg)](https://img.shields.io/crates/d/hyperlane-macros.svg)
[![](https://docs.rs/hyperlane-macros/badge.svg)](https://docs.rs/hyperlane-macros)
[![](https://github.com/crates-dev/hyperlane-macros/workflows/Rust/badge.svg)](https://github.com/crates-dev/hyperlane-macros/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-macros.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-macros/)

[Api Docs](https://docs.rs/hyperlane-macros/latest/hyperlane_macros/)

> A comprehensive collection of procedural macros for building HTTP servers with enhanced functionality. This crate provides attribute macros that simplify HTTP request handling, protocol validation, response management, and request data extraction.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-macros
```

## Available Macros

### Hyperlane Macro

- `#[hyperlane(server: Server)]` - Creates a new `Server` instance with the specified variable name and type, and automatically registers other hooks and routes defined within the crate.
- `#[hyperlane(config: ServerConfig)]` - Creates a new `ServerConfig` instance with the specified variable name and type.

### HTTP Method Macros

- `#[methods(method1, method2, ...)]` - Accepts multiple HTTP methods
- `#[get]` - GET method handler
- `#[post]` - POST method handler
- `#[put]` - PUT method handler
- `#[delete]` - DELETE method handler
- `#[patch]` - PATCH method handler
- `#[head]` - HEAD method handler
- `#[options]` - OPTIONS method handler
- `#[connect]` - CONNECT method handler
- `#[trace]` - TRACE method handler

### Protocol Check Macros

- `#[ws]` - WebSocket check, ensures function only executes for WebSocket upgrade requests
- `#[http]` - HTTP check, ensures function only executes for standard HTTP requests
- `#[h2c]` - HTTP/2 Cleartext check, ensures function only executes for HTTP/2 cleartext requests
- `#[http0_9]` - HTTP/0.9 check, ensures function only executes for HTTP/0.9 protocol requests
- `#[http1_0]` - HTTP/1.0 check, ensures function only executes for HTTP/1.0 protocol requests
- `#[http1_1]` - HTTP/1.1 check, ensures function only executes for HTTP/1.1 protocol requests
- `#[http1_1_or_higher]` - HTTP/1.1 or higher version check, ensures function only executes for HTTP/1.1 or newer protocol versions
- `#[http2]` - HTTP/2 check, ensures function only executes for HTTP/2 protocol requests
- `#[http3]` - HTTP/3 check, ensures function only executes for HTTP/3 protocol requests
- `#[tls]` - TLS check, ensures function only executes for TLS-secured connections

### Response Setting Macros

- `#[response_status_code(code)]` - Set response status code (supports literals and global constants)
- `#[response_reason_phrase("phrase")]` - Set response reason phrase (supports literals and global constants)
- `#[response_header("key", "value")]` - Add response header (supports literals and global constants)
- `#[response_header("key" => "value")]` - Set response header (supports literals and global constants)
- `#[response_body("data")]` - Set response body (supports literals and global constants)
- `#[response_version(version)]` - Set response HTTP version (supports literals and global constants)

### Send Operation Macros

- `#[send]` - Send complete response (headers and body) after function execution
- `#[send_body]` - Send only response body after function execution
- `#[send_once]` - Send complete response exactly once after function execution
- `#[send_once_body]` - Send response body exactly once after function execution

### Flush Macros

- `#[flush]` - Flush response stream after function execution to ensure immediate data transmission

### Aborted Macros

- `#[aborted]` - Handle aborted requests, providing cleanup logic for prematurely terminated connections

### Closed Operation Macros

- `#[closed]` - Handle closed streams, providing cleanup logic for completed connections

### Filter Macros

- `#[filter_unknown_method]` - Filter unknown HTTP methods, handling requests with non-standard methods
- `#[filter_unknown_upgrade]` - Filter unknown upgrade requests, handling requests with non-standard upgrade protocols
- `#[filter_unknown_version]` - Filter unknown HTTP versions, handling requests with non-standard HTTP protocol versions
- `#[filter_unknown]` - Combined filter for unknown method, upgrade, and version

### Request Body Macros

- `#[request_body(variable_name)]` - Extract raw request body into specified variable with RequestBody type
- `#[request_body_json(variable_name: type)]` - Parse request body as JSON into specified variable and type

### Attribute Macros

- `#[attribute(key => variable_name: type)]` - Extract a specific attribute by key into a typed variable

### Attributes Macros

- `#[attributes(variable_name)]` - Get all attributes as a HashMap for comprehensive attribute access

### Route Param Macros

- `#[route_param(key => variable_name)]` - Extract a specific route parameter by key into a variable

### Route Params Macros

- `#[route_params(variable_name)]` - Get all route parameters as a collection

### Request Query Macros

- `#[request_query(key => variable_name)]` - Extract a specific query parameter by key from the URL query string

### Request Querys Macros

- `#[request_querys(variable_name)]` - Get all query parameters as a collection

### Request Header Macros

- `#[request_header(key => variable_name)]` - Extract a specific HTTP header by name from the request

### Request Headers Macros

- `#[request_headers(variable_name)]` - Get all HTTP headers as a collection

### Request Cookie Macros

- `#[request_cookie(key => variable_name)]` - Extract a specific cookie value by key from the request cookie header

### Request Cookies Macros

- `#[request_cookies(variable_name)]` - Get all cookies as a raw string from the cookie header

### Request Version Macros

- `#[request_version(variable_name)]` - Extract the HTTP request version into a variable

### Request Path Macros

- `#[request_path(variable_name)]` - Extract the HTTP request path into a variable

### Host Macros

- `#[host("hostname")]` - Restrict function execution to requests with a specific host header value
- `#[host_filter("hostname")]` - Filter requests that match a specific host header value

### Referer Macros

- `#[referer("url")]` - Restrict function execution to requests with a specific referer header value
- `#[referer_filter("url")]` - Filter requests that match a specific referer header value

### Hook Macros

- `#[pre_hook(function_name)]` - Execute specified function before the main handler function
- `#[post_hook(function_name)]` - Execute specified function after the main handler function
- `#[connected_hook]` - Execute function when a new client connection is established
- `#[panic_hook]` - Execute function when a panic occurs within the server
- `#[pre_upgrade_hook]` - Execute function before any protocol upgrade occurs

### Disable Hook Macros

- `#[disable_http_hook]` - Disable HTTP handling for a specific route
- `#[disable_ws_hook]` - Disable WebSocket handling for a specific route

### Middleware Macros

- `#[request_middleware]` - Register a function as a request middleware
- `#[response_middleware]` - Register a function as a response middleware

### Response Header Macros

- `#[response_header("key", "value")]` - Add a specific HTTP response header with the given key and value (add to existing headers)
- `#[response_header("key" => "value")]` - Set a specific HTTP response header with the given key and value (overwrite existing)

### Response Body Macros

- `#[response_body(value)]` - Set the HTTP response body with the given value

### Route Macros

- `#[route("path")]` - Register a route handler for the given path using the default server (Prerequisite: requires the #[hyperlane(server: Server)] macro)

### Helper Tips

- **Request related macros** (data extraction) use **`get`** operations - they retrieve/query data from the request
- **Response related macros** (data setting) use **`set`** operations - they assign/configure response data
- **Hook macros** For hook-related macros that support an `order` parameter, if `order` is not specified, the hook will have higher priority than hooks with a specified `order` (applies only to macros like `#[request_middleware]`, `#[response_middleware]`, `#[panic_hook]`, `#[connected_hook]`, `#[pre_upgrade_hook]`)

### Best Practice Warning

- Request related macros are mostly query functions, while response related macros are mostly assignment functions.
- When using `pre_hook` or `post_hook` macros, it is not recommended to combine them with other macros (such as `#[get]`, `#[post]`, `#[http]`, etc.) on the same function. These macros should be placed in the hook functions themselves. If you are not clear about how macros are expanded, combining them may lead to problematic code behavior.

## Example Usage

```rust
use hyperlane::*;
use hyperlane_macros::*;
use serde::{Deserialize, Serialize};

const TEST_ATTRIBUTE_KEY: &str = "test_attribute_key";
const CUSTOM_STATUS_CODE: i32 = 200;
const CUSTOM_REASON: &str = "Accepted";
const CUSTOM_HEADER_NAME: &str = "X-Custom-Header";
const CUSTOM_HEADER_VALUE: &str = "custom-value";
const RESPONSE_DATA: &str = "{\"status\": \"success\"}";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestData {
    name: String,
    age: u32,
}

#[response_body("1")]
#[request_middleware]
#[response_version(HttpVersion::HTTP1_1)]
async fn request_middleware(ctx: Context) {}

#[response_body("2")]
#[request_middleware(1)]
#[response_version(HttpVersion::HTTP1_1)]
async fn request_middleware_1(ctx: Context) {}

#[response_middleware]
async fn response_middleware(ctx: Context) {
    if ctx.get_request().await.get_upgrade_type().is_ws() {
        return;
    }
}

#[send]
#[response_middleware(1)]
async fn response_middleware_1(ctx: Context) {}

#[panic_hook]
#[send]
async fn panic_hook(ctx: Context) {}

#[route("/disable_http_hook")]
#[response_body("disable_http_hook")]
#[disable_http_hook("/disable_http_hook")]
async fn disable_http_hook(ctx: Context) {}

#[route("/disable_ws_hook")]
#[response_body("disable_ws_hook")]
#[disable_ws_hook("/disable_ws_hook")]
async fn disable_ws_hook(ctx: Context) {}

#[get]
#[http]
async fn ctx_pre_hook(ctx: Context) {}

#[flush]
#[response_status_code(200)]
async fn ctx_post_hook(ctx: Context) {}

#[route("/response")]
#[response_body(RESPONSE_DATA)]
#[response_reason_phrase(CUSTOM_REASON)]
#[response_status_code(CUSTOM_STATUS_CODE)]
#[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
async fn response(ctx: Context) {}

#[connect]
#[route("/connect")]
#[response_body("connect")]
async fn connect(ctx: Context) {}

#[delete]
#[route("/delete")]
#[response_body("delete")]
async fn delete(ctx: Context) {}

#[head]
#[route("/head")]
#[response_body("head")]
async fn head(ctx: Context) {}

#[options]
#[route("/options")]
#[response_body("options")]
async fn options(ctx: Context) {}

#[patch]
#[route("/patch")]
#[response_body("patch")]
async fn patch(ctx: Context) {}

#[put]
#[route("/put")]
#[response_body("put")]
async fn put(ctx: Context) {}

#[trace]
#[route("/trace")]
#[response_body("trace")]
async fn trace(ctx: Context) {}

#[h2c]
#[route("/h2c")]
#[response_body("h2c")]
async fn h2c(ctx: Context) {}

#[http]
#[route("/http")]
#[response_body("http")]
async fn http_only(ctx: Context) {}

#[http0_9]
#[route("/http0_9")]
#[response_body("http0.9")]
async fn http0_9(ctx: Context) {}

#[http1_0]
#[route("/http1_0")]
#[response_body("http1.0")]
async fn http1_0(ctx: Context) {}

#[http1_1]
#[route("/http1_1")]
#[response_body("http1.1")]
async fn http1_1(ctx: Context) {}

#[http2]
#[route("/http2")]
#[response_body("http2")]
async fn http2(ctx: Context) {}

#[http3]
#[route("/http3")]
#[response_body("http3")]
async fn http3(ctx: Context) {}

#[tls]
#[route("/tls")]
#[response_body("tls")]
async fn tls(ctx: Context) {}

#[http1_1_or_higher]
#[route("/http1_1_or_higher")]
#[response_body("http1.1+")]
async fn http1_1_or_higher(ctx: Context) {}

#[filter_unknown_method]
#[route("/unknown_method")]
#[response_body("unknown method")]
async fn unknown_method(ctx: Context) {}

#[filter_unknown_upgrade]
#[route("/unknown_upgrade")]
#[response_body("unknown upgrade")]
async fn unknown_upgrade(ctx: Context) {}

#[filter_unknown_version]
#[route("/unknown_version")]
#[response_body("unknown version")]
async fn unknown_version(ctx: Context) {}

#[filter_unknown]
#[route("/unknown_all")]
#[response_body("unknown all")]
async fn unknown_all(ctx: Context) {}

#[ws]
#[get]
#[send_body]
#[route("/get")]
#[response_body("get")]
async fn get(ctx: Context) {}

#[post]
#[send_once]
#[route("/post")]
#[response_body("post")]
async fn post(ctx: Context) {}

#[ws]
#[send_once_body]
#[route("/websocket")]
#[response_body("websocket")]
async fn websocket(ctx: Context) {}

#[route("/ctx_hook")]
#[pre_hook(ctx_pre_hook)]
#[post_hook(ctx_post_hook)]
#[response_body("Testing hook macro")]
async fn ctx_hook(ctx: Context) {}

#[http]
#[closed]
#[route("/get_post")]
#[methods(get, post)]
#[response_reason_phrase("OK")]
#[response_status_code(200)]
#[response_body("get_post")]
async fn get_post(ctx: Context) {}

#[route("/attributes")]
#[response_body(format!("request attributes: {request_attributes:?}"))]
#[attributes(request_attributes)]
async fn attributes(ctx: Context) {}

#[route("/route_params/:test")]
#[response_body(format!("request route params: {request_route_params:?}"))]
#[route_params(request_route_params)]
async fn route_params(ctx: Context) {}

#[route("/request_querys")]
#[response_body(format!("request querys: {request_querys:?}"))]
#[request_querys(request_querys)]
async fn request_querys(ctx: Context) {}

#[route("/request_headers")]
#[response_body(format!("request headers: {request_headers:?}"))]
#[request_headers(request_headers)]
async fn request_headers(ctx: Context) {}

#[route("/route_param/:test")]
#[response_body(format!("route param: {request_route_param:?}"))]
#[route_param("test" => request_route_param)]
async fn route_param(ctx: Context) {}

#[route("/request_query")]
#[response_body(format!("request query: {request_query_option:?}"))]
#[request_query("test" => request_query_option)]
async fn request_query(ctx: Context) {}

#[route("/request_header")]
#[response_body(format!("request header: {request_header_option:?}"))]
#[request_header(HOST => request_header_option)]
async fn request_header(ctx: Context) {}

#[response_body(format!("raw body: {raw_body:?}"))]
#[request_body(raw_body)]
#[route("/request_body")]
async fn request_body(ctx: Context) {}

#[route("/host")]
#[host("localhost")]
#[response_body("host string literal: localhost")]
async fn host(ctx: Context) {}

#[route("/host_filter")]
#[host_filter("filter.localhost")]
#[response_body("host filter string literal")]
async fn host_filter(ctx: Context) {}

#[route("/attribute")]
#[response_body(format!("request attribute: {request_attribute_option:?}"))]
#[attribute(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
async fn attribute(ctx: Context) {}

#[route("/request_body_json")]
#[response_body(format!("request data: {request_data_result:?}"))]
#[request_body_json(request_data_result: TestData)]
async fn request_body_json(ctx: Context) {}

#[route("/referer")]
#[referer("http://localhost")]
#[response_body("referer string literal: http://localhost")]
async fn referer(ctx: Context) {}

#[route("/referer_filter")]
#[referer_filter("http://localhost")]
#[response_body("referer filter string literal")]
async fn referer_filter(ctx: Context) {}

#[route("/cookies")]
#[response_body(format!("All cookies: {cookie_value:?}"))]
#[request_cookies(cookie_value)]
async fn cookies(ctx: Context) {}

#[route("/cookie")]
#[response_body(format!("Session cookie: {session_cookie_opt:?}"))]
#[request_cookie("test" => session_cookie_opt)]
async fn cookie(ctx: Context) {}

#[route("/request_version")]
#[response_body(format!("HTTP Version: {http_version}"))]
#[request_version(http_version)]
async fn request_version_test(ctx: Context) {}

#[route("/request_path")]
#[response_body(format!("Request Path: {request_path}"))]
#[request_path(request_path)]
async fn request_path_test(ctx: Context) {}

#[route("/response_header")]
#[response_body("Testing header set and replace operations")]
#[response_header("X-Add-Header", "add-value")]
#[response_header("X-Set-Header" => "set-value")]
async fn response_header_test(ctx: Context) {}

#[route("/literals")]
#[response_status_code(201)]
#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
#[response_body("{\"message\": \"Resource created\"}")]
#[response_reason_phrase(HttpStatus::Created.to_string())]
async fn literals(ctx: Context) {}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.disable_nodelay().await;
    server.config(config).await;
    let server_hook: ServerHook = server.run().await.unwrap_or_default();
    server_hook.wait().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
