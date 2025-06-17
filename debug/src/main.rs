use hyperlane::*;

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
    let test = || async move {
        server.run().await.unwrap();
    };
    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), test()).await;
}
