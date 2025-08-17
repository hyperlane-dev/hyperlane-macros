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

- `#[hyperlane(server: Server)]` - Creates a new `Server` or `ServerConfig` instance with the specified variable name and type.
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

### Response Header Macros

- `#[response_header("key", "value")]` - Add a specific HTTP response header with the given key and value (add to existing headers)
- `#[response_header("key" => "value")]` - Set a specific HTTP response header with the given key and value (overwrite existing)

### Response Body Macros

- `#[response_body(value)]` - Set the HTTP response body with the given value

### Helper Tips

- **Request related macros** (data extraction) use **`get`** operations - they retrieve/query data from the request
- **Response related macros** (data setting) use **`set`** operations - they assign/configure response data

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

#[response_version(HttpVersion::HTTP1_1)]
async fn request_middleware(ctx: Context) {}

#[get]
#[http]
async fn ctx_pre_hook(ctx: Context) {}

#[flush]
#[send]
#[response_status_code(200)]
async fn ctx_post_hook(ctx: Context) {}

#[send]
#[response_status_code(CUSTOM_STATUS_CODE)]
#[response_reason_phrase(CUSTOM_REASON)]
#[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
#[response_body(RESPONSE_DATA)]
async fn response(ctx: Context) {}

#[connect]
async fn connect(ctx: Context) {
    let _ = ctx.set_response_body("connect").await.send().await;
}

#[delete]
async fn delete(ctx: Context) {
    let _ = ctx.set_response_body("delete").await.send().await;
}

#[head]
async fn head(ctx: Context) {
    let _ = ctx.set_response_body("head").await.send().await;
}

#[options]
async fn options(ctx: Context) {
    let _ = ctx.set_response_body("options").await.send().await;
}

#[patch]
async fn patch(ctx: Context) {
    let _ = ctx.set_response_body("patch").await.send().await;
}

#[put]
async fn put(ctx: Context) {
    let _ = ctx.set_response_body("put").await.send().await;
}

#[trace]
async fn trace(ctx: Context) {
    let _ = ctx.set_response_body("trace").await.send().await;
}

#[send]
#[h2c]
async fn h2c(ctx: Context) {
    let _ = ctx.set_response_body("h2c").await;
}

#[send]
#[http]
async fn http_only(ctx: Context) {
    let _ = ctx.set_response_body("http").await;
}

#[send]
#[http0_9]
async fn http0_9(ctx: Context) {
    let _ = ctx.set_response_body("http0.9").await;
}

#[send]
#[http1_0]
async fn http1_0(ctx: Context) {
    let _ = ctx.set_response_body("http1.0").await;
}

#[send]
#[http1_1]
async fn http1_1(ctx: Context) {
    let _ = ctx.set_response_body("http1.1").await;
}

#[send]
#[http2]
async fn http2(ctx: Context) {
    let _ = ctx.set_response_body("http2").await;
}

#[send]
#[http3]
async fn http3(ctx: Context) {
    let _ = ctx.set_response_body("http3").await;
}

#[send]
#[tls]
async fn tls(ctx: Context) {
    let _ = ctx.set_response_body("tls").await;
}

#[send]
#[http1_1_or_higher]
async fn http1_1_or_higher(ctx: Context) {
    let _ = ctx.set_response_body("http1.1+").await;
}

#[send]
#[filter_unknown_method]
async fn unknown_method(ctx: Context) {
    let _ = ctx.set_response_body("unknown method").await;
}

#[send]
#[filter_unknown_upgrade]
async fn unknown_upgrade(ctx: Context) {
    let _ = ctx.set_response_body("unknown upgrade").await;
}

#[send]
#[filter_unknown_version]
async fn unknown_version(ctx: Context) {
    let _ = ctx.set_response_body("unknown version").await;
}

#[send]
#[filter_unknown]
async fn unknown_all(ctx: Context) {
    let _ = ctx.set_response_body("unknown all").await;
}

#[send_body]
#[ws]
#[get]
async fn get(ctx: Context) {
    let _ = ctx.set_response_body("get").await;
}

#[send_once]
#[post]
async fn post(ctx: Context) {
    let _ = ctx.set_response_body("post").await;
}

#[send_once_body]
#[ws]
async fn websocket(ctx: Context) {
    let _ = ctx.set_response_body("websocket").await;
}

#[send]
#[pre_hook(ctx_pre_hook)]
#[post_hook(ctx_post_hook)]
async fn ctx_hook(ctx: Context) {
    let _ = ctx.set_response_body("Testing hook macro").await;
}

#[closed]
#[send]
#[response_reason_phrase("OK")]
#[response_status_code(200)]
#[methods(get, post)]
#[http]
async fn get_post(ctx: Context) {
    let _ = ctx.set_response_body("get_post").await;
}

#[send]
#[attributes(request_attributes)]
async fn attributes(ctx: Context) {
    let response: String = format!("{:?}", request_attributes);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[route_params(request_route_params)]
async fn route_params(ctx: Context) {
    let response: String = format!("{:?}", request_route_params);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_querys(request_querys)]
async fn request_querys(ctx: Context) {
    let response: String = format!("{:?}", request_querys);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_headers(request_headers)]
async fn request_headers(ctx: Context) {
    let response: String = format!("{:?}", request_headers);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[route_param("test" => request_route_param)]
async fn route_param(ctx: Context) {
    if let Some(data) = request_route_param {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[request_query("test" => request_query_option)]
async fn request_query(ctx: Context) {
    if let Some(data) = request_query_option {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[request_header(HOST => request_header_option)]
async fn request_header(ctx: Context) {
    if let Some(data) = request_header_option {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[request_body(raw_body)]
async fn request_body(ctx: Context) {
    let response: String = format!("Raw body: {:?}", raw_body);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[host("localhost")]
async fn host(ctx: Context) {
    let _ = ctx
        .set_response_body("host string literal: localhost")
        .await;
}

#[send]
#[host_filter("filter.localhost")]
async fn host_filter(ctx: Context) {
    let _ = ctx.set_response_body("host filter string literal").await;
}

#[send]
#[attribute(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
async fn attribute(ctx: Context) {
    if let Some(data) = request_attribute_option {
        let response: String = format!("name={}, age={}", data.name, data.age);
        let _ = ctx.set_response_body(response).await;
    }
}

#[send]
#[request_body_json(request_data_result: TestData)]
async fn request_body_json(ctx: Context) {
    if let Ok(data) = request_data_result {
        let response: String = format!("name={}, age={}", data.name, data.age);
        let _ = ctx.set_response_body(response).await;
    }
}

#[send]
#[referer("http://localhost")]
async fn referer(ctx: Context) {
    let _ = ctx
        .set_response_body("referer string literal: http://localhost")
        .await;
}

#[send]
#[referer_filter("http://localhost")]
async fn referer_filter(ctx: Context) {
    let _ = ctx.set_response_body("referer filter string literal").await;
}

#[send]
#[request_cookies(cookie_value)]
async fn cookies(ctx: Context) {
    let response: String = format!("All cookies: {:?}", cookie_value);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_cookie("test" => session_cookie_opt)]
async fn cookie(ctx: Context) {
    if let Some(session) = session_cookie_opt {
        let response: String = format!("Session cookie: {}", session);
        let _ = ctx.set_response_body(response).await;
    }
}

#[send]
#[request_version(http_version)]
async fn request_version_test(ctx: Context) {
    let response: String = format!("HTTP Version: {:?}", http_version);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_path(request_path)]
async fn request_path_test(ctx: Context) {
    let response: String = format!("Request Path: {:?}", request_path);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[response_header("X-Add-Header", "add-value")]
#[response_header("X-Set-Header" => "set-value")]
async fn response_header_test(ctx: Context) {
    let _ = ctx
        .set_response_body("Testing header set and replace operations")
        .await;
}

#[send]
#[response_status_code(201)]
#[response_reason_phrase(HttpStatus::Created.to_string())]
#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
#[response_body("{\"message\": \"Resource created\"}")]
async fn literals(ctx: Context) {}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.disable_nodelay().await;
    server.config(config).await;
    server.request_middleware(request_middleware).await;
    server.route("/response", response).await;
    server.route("/connect", connect).await;
    server.route("/delete", delete).await;
    server.route("/head", head).await;
    server.route("/options", options).await;
    server.route("/patch", patch).await;
    server.route("/put", put).await;
    server.route("/trace", trace).await;
    server.route("/h2c", h2c).await;
    server.route("/http", http_only).await;
    server.route("/http0_9", http0_9).await;
    server.route("/http1_0", http1_0).await;
    server.route("/http1_1", http1_1).await;
    server.route("/http2", http2).await;
    server.route("/http3", http3).await;
    server.route("/tls", tls).await;
    server.route("/http1_1_or_higher", http1_1_or_higher).await;
    server.route("/unknown_method", unknown_method).await;
    server.route("/unknown_upgrade", unknown_upgrade).await;
    server.route("/unknown_version", unknown_version).await;
    server.route("/unknown_all", unknown_all).await;
    server.route("/get", get).await;
    server.route("/post", post).await;
    server.route("/websocket", websocket).await;
    server.route("/ctx_hook", ctx_hook).await;
    server.route("/get_post", get_post).await;
    server.route("/attributes", attributes).await;
    server.route("/route_params/:test", route_params).await;
    server.route("/request_querys", request_querys).await;
    server.route("/request_headers", request_headers).await;
    server.route("/route_param/:test", route_param).await;
    server.route("/request_query", request_query).await;
    server.route("/request_header", request_header).await;
    server.route("/request_body", request_body).await;
    server.route("/host", host).await;
    server.route("/host_filter", host_filter).await;
    server.route("/attribute", attribute).await;
    server.route("/request_body_json", request_body_json).await;
    server.route("/referer", referer).await;
    server.route("/referer_filter", referer_filter).await;
    server.route("/cookies", cookies).await;
    server.route("/cookie", cookie).await;
    server.route("/request_version", request_version_test).await;
    server.route("/request_path", request_path_test).await;
    server.route("/response_header", response_header_test).await;
    server.route("/literals", literals).await;
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
