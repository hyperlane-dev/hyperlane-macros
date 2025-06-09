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
