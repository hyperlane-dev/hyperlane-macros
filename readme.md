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

### Filter Macros

- `#[filter_unknown_method]` - Filter unknown HTTP methods
- `#[filter_unknown_upgrade]` - Filter unknown upgrade requests
- `#[filter_unknown_version]` - Filter unknown HTTP versions
- `#[filter_unknown]` - Combined filter for unknown method, upgrade, and version

### Hook Macros

- `#[pre_hook(function_name)]` - Execute function before the marked code
- `#[post_hook(function_name)]` - Execute function after the marked code

**⚠️ Best Practice Warning**: When using `pre_hook` or `post_hook` macros, it is **not recommended** to combine them with other macros (such as `#[get]`, `#[post]`, `#[http]`, etc.) on the same function. These macros should be placed in the hook functions themselves. If you are not clear about how macros are expanded, combining them may lead to problematic code behavior.

## Example Usage

```rust
use hyperlane::*;
use hyperlane::*;

#[hyperlane_macros::get]
#[hyperlane_macros::http]
async fn test_pre_hook(ctx: Context) {
    println!(
        "Executing pre hook function, parameter type: {:?}",
        std::any::type_name_of_val(&ctx)
    );
}

#[hyperlane_macros::status_code(200)]
#[hyperlane_macros::send]
async fn test_post_hook(ctx: Context) {
    println!(
        "Executing post hook function, parameter type: {:?}",
        std::any::type_name_of_val(&ctx)
    );
}

#[hyperlane_macros::pre_hook(test_pre_hook)]
#[hyperlane_macros::post_hook(test_post_hook)]
async fn hook_example(ctx: Context) {
    let _ = ctx.set_response_body("hook example").await;
}

#[hyperlane_macros::methods(get, post)]
#[hyperlane_macros::http]
#[hyperlane_macros::status_code(200)]
#[hyperlane_macros::reason_phrase("OK")]
#[hyperlane_macros::send]
async fn get_post(ctx: Context) {
    let _ = ctx.set_response_body("get_post").await;
}

#[hyperlane_macros::get]
#[hyperlane_macros::ws]
#[hyperlane_macros::send_body]
async fn get(ctx: Context) {
    let _ = ctx.set_response_body("get").await;
}

#[hyperlane_macros::post]
#[hyperlane_macros::send_once]
async fn post(ctx: Context) {
    let _ = ctx.set_response_body("post").await;
}

#[hyperlane_macros::connect]
async fn connect(ctx: Context) {
    let _ = ctx.set_response_body("connect").await.send().await;
}

#[hyperlane_macros::delete]
async fn delete(ctx: Context) {
    let _ = ctx.set_response_body("delete").await.send().await;
}

#[hyperlane_macros::head]
async fn head(ctx: Context) {
    let _ = ctx.set_response_body("head").await.send().await;
}

#[hyperlane_macros::options]
async fn options(ctx: Context) {
    let _ = ctx.set_response_body("options").await.send().await;
}

#[hyperlane_macros::patch]
async fn patch(ctx: Context) {
    let _ = ctx.set_response_body("patch").await.send().await;
}

#[hyperlane_macros::put]
async fn put(ctx: Context) {
    let _ = ctx.set_response_body("put").await.send().await;
}

#[hyperlane_macros::trace]
async fn trace(ctx: Context) {
    let _ = ctx.set_response_body("trace").await.send().await;
}

#[hyperlane_macros::ws]
#[hyperlane_macros::send_once_body]
async fn websocket(ctx: Context) {
    let _ = ctx.set_response_body("websocket").await;
}

#[hyperlane_macros::http]
#[hyperlane_macros::send]
async fn http_only(ctx: Context) {
    let _ = ctx.set_response_body("http").await;
}

#[hyperlane_macros::h2c]
#[hyperlane_macros::send]
async fn h2c(ctx: Context) {
    let _ = ctx.set_response_body("h2c").await;
}

#[hyperlane_macros::http0_9]
#[hyperlane_macros::send]
async fn http0_9(ctx: Context) {
    let _ = ctx.set_response_body("http0.9").await;
}

#[hyperlane_macros::http1_0]
#[hyperlane_macros::send]
async fn http1_0(ctx: Context) {
    let _ = ctx.set_response_body("http1.0").await;
}

#[hyperlane_macros::http1_1]
#[hyperlane_macros::send]
async fn http1_1(ctx: Context) {
    let _ = ctx.set_response_body("http1.1").await;
}

#[hyperlane_macros::http1_1_or_higher]
#[hyperlane_macros::send]
async fn http1_1_or_higher(ctx: Context) {
    let _ = ctx.set_response_body("http1.1+").await;
}

#[hyperlane_macros::http2]
#[hyperlane_macros::send]
async fn http2(ctx: Context) {
    let _ = ctx.set_response_body("http2").await;
}

#[hyperlane_macros::http3]
#[hyperlane_macros::send]
async fn http3(ctx: Context) {
    let _ = ctx.set_response_body("http3").await;
}

#[hyperlane_macros::tls]
#[hyperlane_macros::send]
async fn tls(ctx: Context) {
    let _ = ctx.set_response_body("tls").await;
}

#[hyperlane_macros::filter_unknown_method]
#[hyperlane_macros::send]
async fn unknown_method(ctx: Context) {
    let _ = ctx.set_response_body("unknown method").await;
}

#[hyperlane_macros::filter_unknown_upgrade]
#[hyperlane_macros::send]
async fn unknown_upgrade(ctx: Context) {
    let _ = ctx.set_response_body("unknown upgrade").await;
}

#[hyperlane_macros::filter_unknown_version]
#[hyperlane_macros::send]
async fn unknown_version(ctx: Context) {
    let _ = ctx.set_response_body("unknown version").await;
}

#[hyperlane_macros::filter_unknown]
#[hyperlane_macros::send]
async fn unknown_all(ctx: Context) {
    let _ = ctx.set_response_body("unknown all").await;
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
    server.route("/hook-example", hook_example).await;
    server.run().await.unwrap();
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
