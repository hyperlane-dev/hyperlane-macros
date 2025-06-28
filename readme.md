<center>

## hyperlane-macros

[![](https://img.shields.io/crates/v/hyperlane-macros.svg)](https://crates.io/crates/hyperlane-macros)
[![](https://img.shields.io/crates/d/hyperlane-macros.svg)](https://img.shields.io/crates/d/hyperlane-macros.svg)
[![](https://docs.rs/hyperlane-macros/badge.svg)](https://docs.rs/hyperlane-macros)
[![](https://github.com/eastspire/hyperlane-macros/workflows/Rust/badge.svg)](https://github.com/eastspire/hyperlane-macros/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-macros.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-macros/)

[Api Docs](https://docs.rs/hyperlane-macros/latest/hyperlane_macros/)

> hyperlane macro.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-macros
```

## Available Macros

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

- `#[ws]` - WebSocket check
- `#[http]` - HTTP check
- `#[h2c]` - HTTP/2 Cleartext check
- `#[http0_9]` - HTTP/0.9 check
- `#[http1_0]` - HTTP/1.0 check
- `#[http1_1]` - HTTP/1.1 check
- `#[http1_1_or_higher]` - HTTP/1.1 or higher version check
- `#[http2]` - HTTP/2 check
- `#[http3]` - HTTP/3 check
- `#[tls]` - TLS check

### Response Setting Macros

- `#[status_code(code)]` - Set response status code
- `#[reason_phrase("phrase")]` - Set response reason phrase

### Send Operation Macros

- `#[send]` - Send response
- `#[send_body]` - Send response body
- `#[send_once]` - Send response once
- `#[send_once_body]` - Send response body once

### Flush Macros

- `#[flush]` - Flush response

### Aborted Macros

- `#[aborted]` - Aborted request

### Closed Operation Macros

- `#[closed]` - Closed stream

### Filter Macros

- `#[filter_unknown_method]` - Filter unknown HTTP methods
- `#[filter_unknown_upgrade]` - Filter unknown upgrade requests
- `#[filter_unknown_version]` - Filter unknown HTTP versions
- `#[filter_unknown]` - Combined filter for unknown method, upgrade, and version

### Request Body Macros

- `#[body(variable_name: type)]` - Parse request body as JSON into specified variable and type

### Attribute Macros

- `#[attribute(key => variable_name: type)]` - Put the attribute of the specified key into the specified variable and type

### Attributes Macros

- `#[attributes(variable_name)]` - Get all attributes as a HashMap

### Route param Macros

- `#[route_param(key => variable_name)]` - Put the route param of the specified key into the specified variable

### Route params Macros

- `#[route_params(variable_name)]` - Get all route parameters

### Query Macros

- `#[query(key => variable_name)]` - Get specific query parameter

### Querys Macros

- `#[querys(variable_name)]` - Get all query parameters

### Header Macros

- `#[header(key => variable_name)]` - Get specific HTTP header

### Headers Macros

- `#[headers(variable_name)]` - Get all HTTP headers

### Hook Macros

- `#[pre_hook(function_name)]` - Execute function before the marked code
- `#[post_hook(function_name)]` - Execute function after the marked code

### Best Practice Warning

- When using `pre_hook` or `post_hook` macros, it is not recommended to combine them with other macros (such as `#[get]`, `#[post]`, `#[http]`, etc.) on the same function. These macros should be placed in the hook functions themselves. If you are not clear about how macros are expanded, combining them may lead to problematic code behavior.

## Example Usage

```rust
use hyperlane::*;
use hyperlane_macros::*;
use serde::{Deserialize, Serialize};

const TEST_ATTRIBUTE_KEY: &str = "test_attribute_key";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestData {
    name: String,
    age: u32,
}

#[get]
#[http]
async fn ctx_pre_hook(ctx: Context) {}

#[flush]
#[send]
#[status_code(200)]
async fn ctx_post_hook(ctx: Context) {}

#[send]
#[pre_hook(ctx_pre_hook)]
#[post_hook(ctx_post_hook)]
async fn ctx_hook(ctx: Context) {
    let _ = ctx.set_response_body("Testing hook macro").await;
}

#[closed]
#[send]
#[reason_phrase("OK")]
#[status_code(200)]
#[methods(get, post)]
#[http]
async fn get_post(ctx: Context) {
    let _ = ctx.set_response_body("get_post").await;
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

#[send_once_body]
#[ws]
async fn websocket(ctx: Context) {
    let _ = ctx.set_response_body("websocket").await;
}

#[send]
#[http]
async fn http_only(ctx: Context) {
    let _ = ctx.set_response_body("http").await;
}

#[send]
#[h2c]
async fn h2c(ctx: Context) {
    let _ = ctx.set_response_body("h2c").await;
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
#[http1_1_or_higher]
async fn http1_1_or_higher(ctx: Context) {
    let _ = ctx.set_response_body("http1.1+").await;
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

#[send]
#[body(request_data_result: TestData)]
async fn body(ctx: Context) {
    if let Ok(data) = request_data_result {
        let response: String = format!("name={}, age={}", data.name, data.age);
        let _ = ctx.set_response_body(response).await;
    }
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
#[attributes(request_attributes)]
async fn attributes(ctx: Context) {
    let response: String = format!("{:?}", request_attributes);
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
#[route_params(request_route_params)]
async fn route_params(ctx: Context) {
    let response: String = format!("{:?}", request_route_params);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[query("test" => request_query_option)]
async fn query(ctx: Context) {
    if let Some(data) = request_query_option {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[querys(request_querys)]
async fn querys(ctx: Context) {
    let response: String = format!("{:?}", request_querys);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[header(HOST => request_header_option)]
async fn header(ctx: Context) {
    if let Some(data) = request_header_option {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[headers(request_headers)]
async fn headers(ctx: Context) {
    let response: String = format!("{:?}", request_headers);
    let _ = ctx.set_response_body(response).await;
}

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.route("/get_post", get_post).await;
    server.route("/get", get).await;
    server.route("/post", post).await;
    server.route("/connect", connect).await;
    server.route("/delete", delete).await;
    server.route("/head", head).await;
    server.route("/options", options).await;
    server.route("/patch", patch).await;
    server.route("/put", put).await;
    server.route("/trace", trace).await;
    server.route("/ws", websocket).await;
    server.route("/http", http_only).await;
    server.route("/h2c", h2c).await;
    server.route("/http0.9", http0_9).await;
    server.route("/http1.0", http1_0).await;
    server.route("/http1.1", http1_1).await;
    server.route("/http1.1+", http1_1_or_higher).await;
    server.route("/http2", http2).await;
    server.route("/http3", http3).await;
    server.route("/tls", tls).await;
    server.route("/unknown-method", unknown_method).await;
    server.route("/unknown-upgrade", unknown_upgrade).await;
    server.route("/unknown-version", unknown_version).await;
    server.route("/unknown-all", unknown_all).await;
    server.route("/hook", ctx_hook).await;
    server.route("/body", body).await;
    server.route("/attribute", attribute).await;
    server.route("/attributes", attributes).await;
    server.route("/route-param", route_param).await;
    server.route("/route-params", route_params).await;
    server.route("/query", query).await;
    server.route("/querys", querys).await;
    server.route("/header", header).await;
    server.route("/headers", headers).await;
    let test = || async move {
        server.run().await.unwrap();
    };
    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), test()).await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
