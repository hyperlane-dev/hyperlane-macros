use hyperlane::*;
use hyperlane_macro::*;

#[methods(get, post)]
async fn get_post(ctx: Context) {
    let _ = ctx.set_response_body("get_post").await.send().await;
}

#[get]
async fn get(ctx: Context) {
    let _ = ctx.set_response_body("get").await.send().await;
}

#[post]
async fn post(ctx: Context) {
    let _ = ctx.set_response_body("post").await.send().await;
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
    let test = || async move {
        server.run().await.unwrap();
    };
    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), test()).await;
}
