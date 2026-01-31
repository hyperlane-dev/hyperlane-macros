<center>

## hyperlane-macros

[![](https://img.shields.io/crates/v/hyperlane-macros.svg)](https://crates.io/crates/hyperlane-macros)
[![](https://img.shields.io/crates/d/hyperlane-macros.svg)](https://img.shields.io/crates/d/hyperlane-macros.svg)
[![](https://docs.rs/hyperlane-macros/badge.svg)](https://docs.rs/hyperlane-macros)
[![](https://github.com/hyperlane-dev/hyperlane-macros/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane-macros/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-macros.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-macros/)

[Api Docs](https://docs.rs/hyperlane-macros/latest/)

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
- `#[get_method]` - GET method handler
- `#[post_method]` - POST method handler
- `#[put_method]` - PUT method handler
- `#[delete_method]` - DELETE method handler
- `#[patch_method]` - PATCH method handler
- `#[head_method]` - HEAD method handler
- `#[options_method]` - OPTIONS method handler
- `#[connect_method]` - CONNECT method handler
- `#[trace_method]` - TRACE method handler
- `#[unknown_method]` - Unknown method handler

### HTTP Version Macros

- `#[http_version]` - HTTP check, ensures function only executes for standard HTTP requests
- `#[http0_9_version]` - HTTP/0.9 check, ensures function only executes for HTTP/0.9 protocol requests
- `#[http1_0_version]` - HTTP/1.0 check, ensures function only executes for HTTP/1.0 protocol requests
- `#[http1_1_version]` - HTTP/1.1 check, ensures function only executes for HTTP/1.1 protocol requests
- `#[http1_1_or_higher_version]` - HTTP/1.1 or higher version check, ensures function only executes for HTTP/1.1 or newer protocol versions
- `#[http2_version]` - HTTP/2 check, ensures function only executes for HTTP/2 protocol requests
- `#[http3_version]` - HTTP/3 check, ensures function only executes for HTTP/3 protocol requests
- `#[unknown_version]` - Unknown version check, ensures function only executes for requests with unknown HTTP versions

### Protocol Check Macros

- `#[ws_upgrade_type]` - WebSocket check, ensures function only executes for WebSocket upgrade requests
- `#[h2c_upgrade_type]` - HTTP/2 Cleartext check, ensures function only executes for HTTP/2 cleartext requests
- `#[tls_upgrade_type]` - TLS check, ensures function only executes for TLS-secured connections
- `#[unknown_upgrade_type]` - Unknown upgrade type check, ensures function only executes for requests with unknown upgrade types

### Response Setting Macros

- `#[response_status_code(code)]` - Set response status code (supports literals and global constants)
- `#[response_reason_phrase("phrase")]` - Set response reason phrase (supports literals and global constants)
- `#[response_header("key", "value")]` - Add response header (supports literals and global constants)
- `#[response_header("key" => "value")]` - Set response header (supports literals and global constants)
- `#[response_body("data")]` - Set response body (supports literals and global constants)
- `#[response_version(version)]` - Set response HTTP version (supports literals and global constants)
- `#[clear_response_headers]` - Clear all response headers

### Send Operation Macros

- `#[try_send]` - Try to send complete response (headers and body) after function execution (returns Result)
- `#[send]` - Send complete response (headers and body) after function execution (**panics on failure**)
- `#[try_send_body]` - Try to send only response body after function execution (returns Result)
- `#[send_body]` - Send only response body after function execution (**panics on failure**)
- `#[try_send_body_with_data("data")]` - Try to send only response body with specified data after function execution (returns Result)

- `#[send_body_with_data("data")]` - Send only response body with specified data after function execution (**panics on failure**)

### Flush Macros

- `#[try_flush]` - Try to flush response stream after function execution to ensure immediate data transmission (returns Result)
- `#[flush]` - Flush response stream after function execution to ensure immediate data transmission (**panics on failure**)

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

- `#[attribute_option(key => variable_name: type)]` - Extract a specific attribute by key into a typed variable
- `#[attribute_option("key1" => var1: Type1, "key2" => var2: Type2, ...)]` - Supports multiple attribute extraction
- `#[attribute(key => variable_name: type)]` - Extract a specific attribute by key into a typed variable
- `#[attribute("key1" => var1: Type1, "key2" => var2: Type2, ...)]` - Supports multiple attribute extraction

### Attributes Macros

- `#[attributes(variable_name)]` - Get all attributes as a HashMap for comprehensive attribute access
- `#[attributes(var1, var2, ...)]` - Supports multiple attribute collections

### Panic Data Macros

- `#[task_panic_data_option(variable_name)]` - Extract panic data into a variable wrapped in Option type
- `#[task_panic_data_option(var1, var2, ...)]` - Supports multiple panic data variables
- `#[task_panic_data(variable_name)]` - Extract panic data into a variable with panic on missing value
- `#[task_panic_data(var1, var2, ...)]` - Supports multiple panic data variables

### Request Error Data Macros

- `#[request_error_data_option(variable_name)]` - Extract request error data into a variable wrapped in Option type
- `#[request_error_data_option(var1, var2, ...)]` - Supports multiple request error data variables
- `#[request_error_data(variable_name)]` - Extract request error data into a variable with panic on missing value
- `#[request_error_data(var1, var2, ...)]` - Supports multiple request error data variables

### Route Param Macros

- `#[route_param_option(key => variable_name)]` - Extract a specific route parameter by key into a variable
- `#[route_param_option("key1" => var1, "key2" => var2, ...)]` - Supports multiple route parameter extraction
- `#[route_param(key => variable_name)]` - Extract a specific route parameter by key into a variable
- `#[route_param("key1" => var1, "key2" => var2, ...)]` - Supports multiple route parameter extraction

### Route Params Macros

- `#[route_params(variable_name)]` - Get all route parameters as a collection
- `#[route_params(var1, var2, ...)]` - Supports multiple route parameter collections

### Request Query Macros

- `#[request_query_option(key => variable_name)]` - Extract a specific query parameter by key from the URL query string
- `#[request_query_option("key1" => var1, "key2" => var2, ...)]` - Supports multiple query parameter extraction
- `#[request_query(key => variable_name)]` - Extract a specific query parameter by key from the URL query string
- `#[request_query("key1" => var1, "key2" => var2, ...)]` - Supports multiple query parameter extraction

### Request Querys Macros

- `#[request_querys(variable_name)]` - Get all query parameters as a collection
- `#[request_querys(var1, var2, ...)]` - Supports multiple query parameter collections

### Request Header Macros

- `#[request_header_option(key => variable_name)]` - Extract a specific HTTP header by name from the request
- `#[request_header_option(KEY1 => var1, KEY2 => var2, ...)]` - Supports multiple header extraction
- `#[request_header(key => variable_name)]` - Extract a specific HTTP header by name from the request
- `#[request_header(KEY1 => var1, KEY2 => var2, ...)]` - Supports multiple header extraction

### Request Headers Macros

- `#[request_headers(variable_name)]` - Get all HTTP headers as a collection
- `#[request_headers(var1, var2, ...)]` - Supports multiple header collections

### Request Cookie Macros

- `#[request_cookie_option(key => variable_name)]` - Extract a specific cookie value by key from the request cookie header
- `#[request_cookie_option("key1" => var1, "key2" => var2, ...)]` - Supports multiple cookie extraction
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
- `#[prologue_hooks(method::expression, another::method)]` - Supports method expressions for advanced hook configurations
- `#[epilogue_hooks(method::expression, another::method)]` - Supports method expressions for advanced hook configurations
- `#[task_panic]` - Execute function when a panic occurs within the server
- `#[request_error]` - Execute function when a request error occurs within the server
- `#[prologue_macros(macro1, macro2, ...)]` - Injects a list of macros before the decorated function.
- `#[epilogue_macros(macro1, macro2, ...)]` - Injects a list of macros after the decorated function.

### Middleware Macros

- `#[request_middleware]` - Register a function as a request middleware
- `#[request_middleware(order)]` - Register a function as a request middleware with specified order
- `#[response_middleware]` - Register a function as a response middleware
- `#[response_middleware(order)]` - Register a function as a response middleware with specified order
- `#[task_panic]` - Register a function as a panic hook
- `#[task_panic(order)]` - Register a function as a panic hook with specified order
- `#[request_error]` - Register a function as a request error hook
- `#[request_error(order)]` - Register a function as a request error hook with specified order

### Stream Processing Macros

- `#[http_from_stream]` - Wraps function body with HTTP stream processing, using default request config. The function body only executes if data is successfully read from the HTTP stream.
- `#[http_from_stream(request_config)]` - Wraps function body with HTTP stream processing using specified request config.
- `#[http_from_stream(variable_name)]` - Wraps function body with HTTP stream processing, storing data in specified variable name.
- `#[http_from_stream(request_config, variable_name)]` - Wraps function body with HTTP stream processing using specified request config and variable name.
- `#[http_from_stream(variable_name, request_config)]` - Wraps function body with HTTP stream processing using specified variable name and request config (reversed order).
- `#[ws_from_stream]` - Wraps function body with WebSocket stream processing, using default request config. The function body only executes if data is successfully read from the WebSocket stream.
- `#[ws_from_stream(request_config)]` - Wraps function body with WebSocket stream processing using specified request config.
- `#[ws_from_stream(variable_name)]` - Wraps function body with WebSocket stream processing, storing data in specified variable name.
- `#[ws_from_stream(request_config, variable_name)]` - Wraps function body with WebSocket stream processing using specified request config and variable name.
- `#[ws_from_stream(variable_name, request_config)]` - Wraps function body with WebSocket stream processing using specified variable name and request config (reversed order).

### Response Header Macros

### Response Body Macros

### Route Macros

- `#[route("path")]` - Register a route handler for the given path using the default server (Prerequisite: requires the #[hyperlane(server: Server)] macro)

### Helper Tips

- **Request related macros** (data extraction) use **`get`** operations - they retrieve/query data from the request
- **Response related macros** (data setting) use **`set`** operations - they assign/configure response data
- **Hook macros** For hook-related macros that support an `order` parameter, if `order` is not specified, the hook will have higher priority than hooks with a specified `order` (applies only to macros like `#[request_middleware]`, `#[response_middleware]`, `#[task_panic]`, `#[request_error]`)
- **Multi-parameter support** Most data extraction macros support multiple parameters in a single call (e.g., `#[request_body(var1, var2)]`, `#[request_query("k1" => v1, "k2" => v2)]`). This reduces macro repetition and improves code readability.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
