<center>

## hyperlane-macros

[![](https://img.shields.io/crates/v/hyperlane-macros.svg)](https://crates.io/crates/hyperlane-macros)
[![](https://img.shields.io/crates/d/hyperlane-macros.svg)](https://img.shields.io/crates/d/hyperlane-macros.svg)
[![](https://docs.rs/hyperlane-macros/badge.svg)](https://docs.rs/hyperlane-macros)
[![](https://github.com/hyperlane-dev/hyperlane-macros/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane-macros/actions?query=workflow:Rust)
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
- `#[hyperlane(var1: Type1, var2: Type2, ...)]` - Supports multiple instance initialization in a single call

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
- `#[clear_response_headers]` - Clear all response headers

### Send Operation Macros

- `#[send]` - Send complete response (headers and body) after function execution
- `#[send_body]` - Send only response body after function execution
- `#[send_with_data("data")]` - Send complete response with specified data after function execution
- `#[send_body_with_data("data")]` - Send only response body with specified data after function execution

### Flush Macros

- `#[flush]` - Flush response stream after function execution to ensure immediate data transmission

### Aborted Macros

- `#[aborted]` - Handle aborted requests, providing cleanup logic for prematurely terminated connections

### Closed Operation Macros

- `#[closed]` - Handle closed streams, providing cleanup logic for completed connections

### Conditional Macros

- `#[filter(condition)]` - Continues execution only if the `condition` (a code block returning a boolean) is `true`.
- `#[reject(condition)]` - Continues execution only if the `condition` (a code block returning a boolean) is `false`.

### Request Body Macros

- `#[request_body(variable_name)]` - Extract raw request body into specified variable with RequestBody type
- `#[request_body(var1, var2, ...)]` - Supports multiple request body variables
- `#[request_body_json(variable_name: type)]` - Parse request body as JSON into specified variable and type
- `#[request_body_json(var1: Type1, var2: Type2, ...)]` - Supports multiple JSON body parsing

### Attribute Macros

- `#[attribute(key => variable_name: type)]` - Extract a specific attribute by key into a typed variable
- `#[attribute("key1" => var1: Type1, "key2" => var2: Type2, ...)]` - Supports multiple attribute extraction

### Attributes Macros

- `#[attributes(variable_name)]` - Get all attributes as a HashMap for comprehensive attribute access
- `#[attributes(var1, var2, ...)]` - Supports multiple attribute collections

### Route Param Macros

- `#[route_param(key => variable_name)]` - Extract a specific route parameter by key into a variable
- `#[route_param("key1" => var1, "key2" => var2, ...)]` - Supports multiple route parameter extraction

### Route Params Macros

- `#[route_params(variable_name)]` - Get all route parameters as a collection
- `#[route_params(var1, var2, ...)]` - Supports multiple route parameter collections

### Request Query Macros

- `#[request_query(key => variable_name)]` - Extract a specific query parameter by key from the URL query string
- `#[request_query("key1" => var1, "key2" => var2, ...)]` - Supports multiple query parameter extraction

### Request Querys Macros

- `#[request_querys(variable_name)]` - Get all query parameters as a collection
- `#[request_querys(var1, var2, ...)]` - Supports multiple query parameter collections

### Request Header Macros

- `#[request_header(key => variable_name)]` - Extract a specific HTTP header by name from the request
- `#[request_header(KEY1 => var1, KEY2 => var2, ...)]` - Supports multiple header extraction

### Request Headers Macros

- `#[request_headers(variable_name)]` - Get all HTTP headers as a collection
- `#[request_headers(var1, var2, ...)]` - Supports multiple header collections

### Request Cookie Macros

- `#[request_cookie(key => variable_name)]` - Extract a specific cookie value by key from the request cookie header
- `#[request_cookie("key1" => var1, "key2" => var2, ...)]` - Supports multiple cookie extraction

### Request Cookies Macros

- `#[request_cookies(variable_name)]` - Get all cookies as a raw string from the cookie header
- `#[request_cookies(var1, var2, ...)]` - Supports multiple cookie collections

### Request Version Macros

- `#[request_version(variable_name)]` - Extract the HTTP request version into a variable
- `#[request_version(var1, var2, ...)]` - Supports multiple request version variables

### Request Path Macros

- `#[request_path(variable_name)]` - Extract the HTTP request path into a variable
- `#[request_path(var1, var2, ...)]` - Supports multiple request path variables

### Host Macros

- `#[host("hostname")]` - Restrict function execution to requests with a specific host header value
- `#[host("host1", "host2", ...)]` - Supports multiple host checks
- `#[reject_host("hostname")]` - Reject requests that match a specific host header value
- `#[reject_host("host1", "host2", ...)]` - Supports multiple host rejections

### Referer Macros

- `#[referer("url")]` - Restrict function execution to requests with a specific referer header value
- `#[referer("url1", "url2", ...)]` - Supports multiple referer checks
- `#[reject_referer("url")]` - Reject requests that match a specific referer header value
- `#[reject_referer("url1", "url2", ...)]` - Supports multiple referer rejections

### Hook Macros

- `#[prologue_hooks(function_name)]` - Execute specified function before the main handler function
- `#[epilogue_hooks(function_name)]` - Execute specified function after the main handler function
- `#[panic_hook]` - Execute function when a panic occurs within the server
- `#[prologue_macros(macro1, macro2, ...)]` - Injects a list of macros before the decorated function.
- `#[epilogue_macros(macro1, macro2, ...)]` - Injects a list of macros after the decorated function.

### Middleware Macros

- `#[request_middleware]` - Register a function as a request middleware
- `#[request_middleware(order)]` - Register a function as a request middleware with specified order
- `#[response_middleware]` - Register a function as a response middleware
- `#[response_middleware(order)]` - Register a function as a response middleware with specified order
- `#[panic_hook]` - Register a function as a panic hook
- `#[panic_hook(order)]` - Register a function as a panic hook with specified order

### Stream Processing Macros

- `#[http_from_stream]` - Wraps function body with HTTP stream processing, using default buffer size. The function body only executes if data is successfully read from the HTTP stream.
- `#[http_from_stream(buffer_size)]` - Wraps function body with HTTP stream processing using specified buffer size.
- `#[http_from_stream(variable_name)]` - Wraps function body with HTTP stream processing, storing data in specified variable name.
- `#[http_from_stream(buffer_size, variable_name)]` - Wraps function body with HTTP stream processing using specified buffer size and variable name.
- `#[http_from_stream(variable_name, buffer_size)]` - Wraps function body with HTTP stream processing using specified variable name and buffer size (reversed order).
- `#[ws_from_stream]` - Wraps function body with WebSocket stream processing, using default buffer size. The function body only executes if data is successfully read from the WebSocket stream.
- `#[ws_from_stream(buffer_size)]` - Wraps function body with WebSocket stream processing using specified buffer size.
- `#[ws_from_stream(variable_name)]` - Wraps function body with WebSocket stream processing, storing data in specified variable name.
- `#[ws_from_stream(buffer_size, variable_name)]` - Wraps function body with WebSocket stream processing using specified buffer size and variable name.
- `#[ws_from_stream(variable_name, buffer_size)]` - Wraps function body with WebSocket stream processing using specified variable name and buffer size (reversed order).

### Response Header Macros

### Response Body Macros

### Route Macros

- `#[route("path")]` - Register a route handler for the given path using the default server (Prerequisite: requires the #[hyperlane(server: Server)] macro)

### Helper Tips

- **Request related macros** (data extraction) use **`get`** operations - they retrieve/query data from the request
- **Response related macros** (data setting) use **`set`** operations - they assign/configure response data
- **Hook macros** For hook-related macros that support an `order` parameter, if `order` is not specified, the hook will have higher priority than hooks with a specified `order` (applies only to macros like `#[request_middleware]`, `#[response_middleware]`, `#[panic_hook]`)
- **Multi-parameter support** Most data extraction macros support multiple parameters in a single call (e.g., `#[request_body(var1, var2)]`, `#[request_query("k1" => v1, "k2" => v2)]`). This reduces macro repetition and improves code readability.

### Best Practice Warning

- Request related macros are mostly query functions, while response related macros are mostly assignment functions.
- When using `prologue_hooks` or `epilogue_hooks` macros, it is not recommended to combine them with other macros (such as `#[get]`, `#[post]`, `#[http]`, etc.) on the same function. These macros should be placed in the hook functions themselves. If you are not clear about how macros are expanded, combining them may lead to problematic code behavior.

## Example Usage

```rust
use hyperlane::*;
use hyperlane_macros::*;
use serde::{Deserialize, Serialize};

const STEP: &str = "step";
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

#[panic_hook]
#[panic_hook(1)]
#[panic_hook("2")]
struct PanicHook;

impl ServerHook for PanicHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(response_body("panic_hook"), send)]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware]
struct RequestMiddleware;

impl ServerHook for RequestMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        response_status_code(200),
        response_version(HttpVersion::HTTP1_1),
        response_header(SERVER => HYPERLANE),
        response_header(CONNECTION => KEEP_ALIVE),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY),
        response_header(STEP => "request_middleware"),
    )]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware(1)]
struct UpgradeHook;

impl ServerHook for UpgradeHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        ws,
        response_body(&vec![]),
        response_status_code(101),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => &WebSocketFrame::generate_accept_key(ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await.unwrap())),
        response_header(STEP => "upgrade_hook"),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware(2)]
struct ConnectedHook;

impl ServerHook for ConnectedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    #[response_header(SERVER => HYPERLANE)]
    #[response_version(HttpVersion::HTTP1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(STEP => "connected_hook")]
    async fn handle(self, ctx: &Context) {}
}

#[response_middleware]
struct ResponseMiddleware1;

impl ServerHook for ResponseMiddleware1 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_header(STEP => "response_middleware_1")]
    async fn handle(self, ctx: &Context) {}
}

#[response_middleware(2)]
struct ResponseMiddleware2;

impl ServerHook for ResponseMiddleware2 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request().await.is_ws()),
        response_header(STEP => "response_middleware_2")
    )]
    #[epilogue_macros(send, flush)]
    async fn handle(self, ctx: &Context) {}
}

#[response_middleware("3")]
struct ResponseMiddleware3;

impl ServerHook for ResponseMiddleware3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        ws,
        response_header(STEP => "response_middleware_3")
    )]
    #[epilogue_macros(send_body, flush)]
    async fn handle(self, ctx: &Context) {}
}

struct PrologueHooks;

impl ServerHook for PrologueHooks {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[get]
    #[http]
    async fn handle(self, _ctx: &Context) {}
}

struct EpilogueHooks;

impl ServerHook for EpilogueHooks {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    async fn handle(self, ctx: &Context) {}
}

async fn prologue_hooks_fn(ctx: Context) {
    let hook = PrologueHooks::new(&ctx).await;
    hook.handle(&ctx).await;
}

async fn epilogue_hooks_fn(ctx: Context) {
    let hook = EpilogueHooks::new(&ctx).await;
    hook.handle(&ctx).await;
}

#[route("/response")]
struct Response;

impl ServerHook for Response {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&RESPONSE_DATA)]
    #[response_reason_phrase(CUSTOM_REASON)]
    #[response_status_code(CUSTOM_STATUS_CODE)]
    #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/connect")]
struct Connect;

impl ServerHook for Connect {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(connect, response_body("connect"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/delete")]
struct Delete;

impl ServerHook for Delete {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(delete, response_body("delete"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/head")]
struct Head;

impl ServerHook for Head {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(head, response_body("head"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/options")]
struct Options;

impl ServerHook for Options {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(options, response_body("options"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/patch")]
struct Patch;

impl ServerHook for Patch {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(patch, response_body("patch"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/put")]
struct Put;

impl ServerHook for Put {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(put, response_body("put"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/trace")]
struct Trace;

impl ServerHook for Trace {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(trace, response_body("trace"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/h2c")]
struct H2c;

impl ServerHook for H2c {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(h2c, response_body("h2c"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http")]
struct HttpOnly;

impl ServerHook for HttpOnly {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http, response_body("http"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http0_9")]
struct Http09;

impl ServerHook for Http09 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http0_9, response_body("http0_9"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http1_0")]
struct Http10;

impl ServerHook for Http10 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http1_0, response_body("http1_0"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http1_1")]
struct Http11;

impl ServerHook for Http11 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1, response_body("http1_1"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http2")]
struct Http2;

impl ServerHook for Http2 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http2, response_body("http2"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http3")]
struct Http3;

impl ServerHook for Http3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http3, response_body("http3"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/tls")]
struct Tls;

impl ServerHook for Tls {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(tls, response_body("tls"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http1_1_or_higher")]
struct Http11OrHigher;

impl ServerHook for Http11OrHigher {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1_or_higher, response_body("http1_1_or_higher"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/unknown_method")]
struct UnknownMethod;

impl ServerHook for UnknownMethod {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        clear_response_headers,
        filter(ctx.get_request().await.is_unknown_method()),
        response_body("unknown_method")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/get")]
struct Get;

impl ServerHook for Get {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws, get, response_body("get"), send_body)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/post")]
struct Post;

impl ServerHook for Post {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(post, response_body("post"), send)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/ws1")]
struct Websocket1;

impl ServerHook for Websocket1 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws2")]
struct Websocket2;

impl ServerHook for Websocket2 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(request)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws3")]
struct Websocket3;

impl ServerHook for Websocket3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(1024, request)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws4")]
struct Websocket4;

impl ServerHook for Websocket4 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(request, 1024)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws5")]
struct Websocket5;

impl ServerHook for Websocket5 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(1024)]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/hook")]
struct Hook;

impl ServerHook for Hook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_hooks(prologue_hooks_fn)]
    #[epilogue_hooks(epilogue_hooks_fn)]
    #[response_body("Testing hook macro")]
    async fn handle(self, ctx: &Context) {}
}

#[route("/get_post")]
struct GetPost;

impl ServerHook for GetPost {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[closed]
    #[prologue_macros(
        http,
        methods(get, post),
        response_body("get_post"),
        response_status_code(200),
        response_reason_phrase("OK")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/attributes")]
struct Attributes;

impl ServerHook for Attributes {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request attributes: {request_attributes:?}"))]
    #[attributes(request_attributes)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/route_params/:test")]
struct RouteParams;

impl ServerHook for RouteParams {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request route params: {request_route_params:?}"))]
    #[route_params(request_route_params)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/route_param/:test")]
struct RouteParam;

impl ServerHook for RouteParam {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param:?} {request_route_param1:?} {request_route_param2:?}"))]
    #[route_param("test" => request_route_param)]
    #[route_param("test1" => request_route_param1, "test2" => request_route_param2)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/host")]
struct Host;

impl ServerHook for Host {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[host("localhost")]
    #[epilogue_macros(
        response_body("host string literal: localhost"),
        send,
        http_from_stream
    )]
    #[prologue_macros(response_body("host string literal: localhost"), send)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_query")]
struct RequestQuery;

impl ServerHook for RequestQuery {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send,
        http_from_stream(1024)
    )]
    #[prologue_macros(
        request_query("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_header")]
struct RequestHeader;

impl ServerHook for RequestHeader {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_header(HOST => request_header_option),
        response_body(&format!("request header: {request_header_option:?}")),
        send,
        http_from_stream(_request)
    )]
    #[prologue_macros(
        request_header(HOST => request_header_option),
        response_body(&format!("request header: {request_header_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_querys")]
struct RequestQuerys;

impl ServerHook for RequestQuerys {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send,
        http_from_stream(1024, _request)
    )]
    #[prologue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_headers")]
struct RequestHeaders;

impl ServerHook for RequestHeaders {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send,
        http_from_stream(_request, 1024)
    )]
    #[prologue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_body")]
struct RequestBodyRoute;

impl ServerHook for RequestBodyRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("raw body: {raw_body:?}"))]
    #[request_body(raw_body)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/reject_host")]
struct RejectHost;

impl ServerHook for RejectHost {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_host("filter.localhost"),
        response_body("host filter string literal")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/attribute")]
struct Attribute;

impl ServerHook for Attribute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute_option:?}"))]
    #[attribute(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_body_json")]
struct RequestBodyJson;

impl ServerHook for RequestBodyJson {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json(request_data_result: TestData)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/referer")]
struct Referer;

impl ServerHook for Referer {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        referer("http://localhost"),
        response_body("referer string literal: http://localhost")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/reject_referer")]
struct RejectReferer;

impl ServerHook for RejectReferer {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_referer("http://localhost"),
        response_body("referer filter string literal")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/cookies")]
struct Cookies;

impl ServerHook for Cookies {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("All cookies: {cookie_value:?}"))]
    #[request_cookies(cookie_value)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/cookie")]
struct Cookie;

impl ServerHook for Cookie {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
    #[request_cookie("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_version")]
struct RequestVersionTest;

impl ServerHook for RequestVersionTest {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("HTTP Version: {http_version}"))]
    #[request_version(http_version)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_path")]
struct RequestPathTest;

impl ServerHook for RequestPathTest {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("Request Path: {request_path}"))]
    #[request_path(request_path)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/response_header")]
struct ResponseHeaderTest;

impl ServerHook for ResponseHeaderTest {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body("Testing header set and replace operations")]
    #[response_header("X-Add-Header", "add-value")]
    #[response_header("X-Set-Header" => "set-value")]
    async fn handle(self, ctx: &Context) {}
}

#[route("/literals")]
struct Literals;

impl ServerHook for Literals {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(201)]
    #[response_header(CONTENT_TYPE => APPLICATION_JSON)]
    #[response_body("{\"message\": \"Resource created\"}")]
    #[response_reason_phrase(HttpStatus::Created.to_string())]
    async fn handle(self, ctx: &Context) {}
}

#[route("/inject/response_body")]
struct InjectResponseBody;

impl ServerHook for InjectResponseBody {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.response_body_with_ref_self(ctx).await;
    }
}

impl InjectResponseBody {
    #[response_body("response body with ref self")]
    async fn response_body_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/post_method")]
struct InjectPostMethod;

impl ServerHook for InjectPostMethod {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.post_method_with_ref_self(ctx).await;
    }
}

impl InjectPostMethod {
    #[prologue_macros(post, response_body("post method with ref self"))]
    async fn post_method_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/send_flush")]
struct InjectSendFlush;

impl ServerHook for InjectSendFlush {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.send_and_flush_with_ref_self(ctx).await;
    }
}

impl InjectSendFlush {
    #[epilogue_macros(send, flush)]
    async fn send_and_flush_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/request_body")]
struct InjectRequestBody;

impl ServerHook for InjectRequestBody {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.extract_request_body_with_ref_self(ctx).await;
    }
}

impl InjectRequestBody {
    #[request_body(_raw_body)]
    async fn extract_request_body_with_ref_self(&self, _ctx: &Context) {}
}

#[route("/inject/multiple_methods")]
struct InjectMultipleMethods;

impl ServerHook for InjectMultipleMethods {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.multiple_methods_with_ref_self(ctx).await;
    }
}

impl InjectMultipleMethods {
    #[methods(get, post)]
    async fn multiple_methods_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/http_stream")]
struct InjectHttpStream;

impl ServerHook for InjectHttpStream {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.http_stream_handler_with_ref_self(ctx).await;
    }
}

impl InjectHttpStream {
    #[http_from_stream(1024, _request)]
    async fn http_stream_handler_with_ref_self(&self, _ctx: &Context) {}
}

#[route("/inject/ws_stream")]
struct InjectWsStream;

impl ServerHook for InjectWsStream {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.websocket_stream_handler_with_ref_self(ctx).await;
    }
}

impl InjectWsStream {
    #[ws_from_stream(_request)]
    async fn websocket_stream_handler_with_ref_self(&self, _ctx: &Context) {}
}

#[route("/inject/complex_post")]
struct InjectComplexPost;

impl ServerHook for InjectComplexPost {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.complex_post_handler_with_ref_self(ctx).await;
    }
}

impl InjectComplexPost {
    #[prologue_macros(
        post,
        http,
        request_body(raw_body),
        response_status_code(201),
        response_header(CONTENT_TYPE => APPLICATION_JSON),
        response_body(&format!("Received: {raw_body:?}"))
    )]
    #[epilogue_macros(send, flush)]
    async fn complex_post_handler_with_ref_self(&self, ctx: &Context) {}
}

impl InjectComplexPost {
    #[post]
    async fn test_with_bool_param(_a: bool, ctx: &Context) {}

    #[get]
    async fn test_with_multiple_params(_a: bool, ctx: &Context, _b: i32) {}
}

#[response_body("standalone response body")]
async fn standalone_response_body_handler(ctx: &Context) {}

#[prologue_macros(get, response_body("standalone get handler"))]
async fn standalone_get_handler(ctx: &Context) {}

#[epilogue_macros(send, flush)]
async fn standalone_send_and_flush_handler(ctx: &Context) {}

#[request_body(_raw_body)]
async fn standalone_request_body_extractor(ctx: &Context) {}

#[methods(get, post)]
async fn standalone_multiple_methods_handler(ctx: &Context) {}

#[http_from_stream]
async fn standalone_http_stream_handler(ctx: &Context) {}

#[ws_from_stream]
async fn standalone_websocket_stream_handler(ctx: &Context) {}

#[prologue_macros(
    get,
    http,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("standalone complex handler")
)]
#[epilogue_macros(send, flush)]
async fn standalone_complex_get_handler(ctx: &Context) {}

#[get]
async fn standalone_get_handler_with_param(_a: bool, ctx: &Context) {}

#[request_body(body1, body2, body3)]
async fn test_multi_request_body(ctx: &Context) {
    println!("body1: {:?}, body2: {:?}, body3: {:?}", body1, body2, body3);
}

#[route("/test_multi_request_body_json")]
#[derive(Debug, serde::Deserialize)]
struct User {
    name: String,
}

impl ServerHook for User {
    async fn new(_ctx: &Context) -> Self {
        Self {
            name: String::from("test"),
        }
    }

    #[prologue_macros(
        request_body_json(user1: User, user2: User),
        response_body(format!(
            "user1: {:?}, user2: {:?}",
            user1.unwrap().name,
            user2.unwrap().name
        )),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[attribute("key1" => attr1: String, "key2" => attr2: i32)]
async fn test_multi_attribute(ctx: &Context) {
    println!("attr1: {:?}, attr2: {:?}", attr1, attr2);
}

#[attributes(attrs1, attrs2)]
async fn test_multi_attributes(ctx: &Context) {
    println!("attrs1: {:?}, attrs2: {:?}", attrs1, attrs2);
}

#[route_params(params1, params2)]
async fn test_multi_route_params(ctx: &Context) {
    println!("params1: {:?}, params2: {:?}", params1, params2);
}

#[request_querys(querys1, querys2)]
async fn test_multi_request_querys(ctx: &Context) {
    println!("querys1: {:?}, querys2: {:?}", querys1, querys2);
}

#[request_headers(headers1, headers2)]
async fn test_multi_request_headers(ctx: &Context) {
    println!("headers1: {:?}, headers2: {:?}", headers1, headers2);
}

#[request_cookies(cookies1, cookies2)]
async fn test_multi_request_cookies(ctx: &Context) {
    println!("cookies1: {:?}, cookies2: {:?}", cookies1, cookies2);
}

#[request_version(version1, version2)]
async fn test_multi_request_version(ctx: &Context) {
    println!("version1: {:?}, version2: {:?}", version1, version2);
}

#[request_path(path1, path2)]
async fn test_multi_request_path(ctx: &Context) {
    println!("path1: {:?}, path2: {:?}", path1, path2);
}

#[host("localhost", "127.0.0.1")]
async fn test_multi_host(ctx: &Context) {
    println!("Host check passed");
}

#[reject_host("badhost.com", "spam.com")]
async fn test_multi_reject_host(ctx: &Context) {
    println!("Reject host check passed");
}

#[referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_referer(ctx: &Context) {
    println!("Referer check passed");
}

#[reject_referer("http://badsite.com", "http://spam.com")]
async fn test_multi_reject_referer(ctx: &Context) {
    println!("Reject referer check passed");
}

#[hyperlane(server1: Server, server2: Server)]
async fn test_multi_hyperlane() {
    println!("server1 and server2 initialized");
}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.disable_nodelay().await;
    server.config(config).await;
    let server_control_hook: ServerControlHook = server.run().await.unwrap_or_default();
    server_control_hook.wait().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
