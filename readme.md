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

- `#[before_hook(function_name)]` - Execute function before the marked code
- `#[after_hook(function_name)]` - Execute function after the marked code

## Example Usage

```rust
use hyperlane::*;

async fn before_hook_function(ctx: Context) {
    println!("前置钩子执行");
}

async fn after_hook_function(ctx: Context) {
    println!("后置钩子执行");
}

#[hyperlane_macros::methods(get, post)]
#[hyperlane_macros::http]
#[hyperlane_macros::status_code(200)]
#[hyperlane_macros::reason_phrase("OK")]
#[hyperlane_macros::send]
async fn get_post(ctx: Context) {
    let _ = ctx.set_response_body("get_post").await;
}

#[hyperlane_macros::before_hook(before_hook_function)]
#[hyperlane_macros::after_hook(after_hook_function)]
#[hyperlane_macros::get]
#[hyperlane_macros::send]
async fn hook_example(ctx: Context) {
    let _ = ctx.set_response_body("hook example").await;
}

#[hyperlane_macros::get]
#[hyperlane_macros::ws]
#[hyperlane_macros::send_body]
async fn websocket(ctx: Context) {
    let _ = ctx.set_response_body("websocket").await;
}

#[hyperlane_macros::http2]
#[hyperlane_macros::send]
async fn http2(ctx: Context) {
    let _ = ctx.set_response_body("http2").await;
}

#[hyperlane_macros::filter_unknown]
#[hyperlane_macros::send]
async fn unknown_all(ctx: Context) {
    let _ = ctx.set_response_body("unknown all").await;
}

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
    server.route("/get_post", get_post).await;
    server.route("/ws", websocket).await;
    server.route("/http2", http2).await;
    server.route("/unknown-all", unknown_all).await;
    server.run().await.unwrap();
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
