<center>

## hyperlane-macro

[![](https://img.shields.io/crates/v/hyperlane-macro.svg)](https://crates.io/crates/hyperlane-macro)
[![](https://img.shields.io/crates/d/hyperlane-macro.svg)](https://img.shields.io/crates/d/hyperlane-macro.svg)
[![](https://docs.rs/hyperlane-macro/badge.svg)](https://docs.rs/hyperlane-macro)
[![](https://github.com/eastspire/hyperlane-macro/workflows/Rust/badge.svg)](https://github.com/eastspire/hyperlane-macro/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-macro.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-macro/)

[Api Docs](https://docs.rs/hyperlane-macro/latest/hyperlane_macro/)

> hyperlane macro.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-macro
```

## Use

```rust
use hyperlane::*;
use hyperlane_macro::*;

#[methods(post, get)]
async fn a(ctx: Context) {
    let _ = ctx.set_response_body("a").await.send().await;
}

#[get]
async fn b(ctx: Context) {
    let _ = ctx.set_response_body("b").await.send().await;
}

fn error_handler(error: String) {
    eprintln!("{}", error);
    let _ = std::io::Write::flush(&mut std::io::stderr());
}

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.error_handler(error_handler).await;
    server.route("/a", a).await;
    server.route("/b", b).await;
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
