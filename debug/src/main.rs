use hyperlane::*;

#[hyperlane_macros::methods(get, post)]
#[hyperlane_macros::http]
#[hyperlane_macros::code(200)]
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

#[tokio::main]
async fn main() {
    let server: Server = Server::new();
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
    let test = || async move {
        server.run().await.unwrap();
    };
    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), test()).await;
}
