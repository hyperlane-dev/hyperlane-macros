use hyperlane::*;
use hyperlane_macros::*;
use serde::{Deserialize, Serialize};

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

#[get]
#[http]
async fn ctx_pre_hook(ctx: Context) {}

#[flush]
#[send]
#[response_status_code(200)]
async fn ctx_post_hook(ctx: Context) {}

#[send]
#[response_status_code(CUSTOM_STATUS_CODE)]
#[response_reason_phrase(CUSTOM_REASON)]
#[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
#[response_body(RESPONSE_DATA)]
async fn response(ctx: Context) {}

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

#[send]
#[h2c]
async fn h2c(ctx: Context) {
    let _ = ctx.set_response_body("h2c").await;
}

#[send]
#[http]
async fn http_only(ctx: Context) {
    let _ = ctx.set_response_body("http").await;
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
#[http1_1_or_higher]
async fn http1_1_or_higher(ctx: Context) {
    let _ = ctx.set_response_body("http1.1+").await;
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

#[send_once_body]
#[ws]
async fn websocket(ctx: Context) {
    let _ = ctx.set_response_body("websocket").await;
}

#[send]
#[pre_hook(ctx_pre_hook)]
#[post_hook(ctx_post_hook)]
async fn ctx_hook(ctx: Context) {
    let _ = ctx.set_response_body("Testing hook macro").await;
}

#[closed]
#[send]
#[response_reason_phrase("OK")]
#[response_status_code(200)]
#[methods(get, post)]
#[http]
async fn get_post(ctx: Context) {
    let _ = ctx.set_response_body("get_post").await;
}

#[send]
#[attributes(request_attributes)]
async fn attributes(ctx: Context) {
    let response: String = format!("{:?}", request_attributes);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[route_params(request_route_params)]
async fn route_params(ctx: Context) {
    let response: String = format!("{:?}", request_route_params);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_querys(request_querys)]
async fn request_querys(ctx: Context) {
    let response: String = format!("{:?}", request_querys);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_headers(request_headers)]
async fn request_headers(ctx: Context) {
    let response: String = format!("{:?}", request_headers);
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
#[request_query("test" => request_query_option)]
async fn request_query(ctx: Context) {
    if let Some(data) = request_query_option {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[request_header(HOST => request_header_option)]
async fn request_header(ctx: Context) {
    if let Some(data) = request_header_option {
        let _ = ctx.set_response_body(data).await;
    }
}

#[send]
#[request_body(raw_body)]
async fn request_body(ctx: Context) {
    let response: String = format!("Raw body: {:?}", raw_body);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[host("localhost")]
async fn host(ctx: Context) {
    let _ = ctx
        .set_response_body("host string literal: localhost")
        .await;
}

#[send]
#[host_filter("filter.localhost")]
async fn host_filter(ctx: Context) {
    let _ = ctx.set_response_body("host filter string literal").await;
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
#[request_body_json(request_data_result: TestData)]
async fn request_body_json(ctx: Context) {
    if let Ok(data) = request_data_result {
        let response: String = format!("name={}, age={}", data.name, data.age);
        let _ = ctx.set_response_body(response).await;
    }
}

#[send]
#[referer("http://localhost")]
async fn referer(ctx: Context) {
    let _ = ctx
        .set_response_body("referer string literal: http://localhost")
        .await;
}

#[send]
#[referer_filter("http://localhost")]
async fn referer_filter(ctx: Context) {
    let _ = ctx.set_response_body("referer filter string literal").await;
}

#[send]
#[request_cookies(cookie_value)]
async fn cookies(ctx: Context) {
    let response: String = format!("All cookies: {:?}", cookie_value);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_cookie("test" => session_cookie_opt)]
async fn cookie(ctx: Context) {
    if let Some(session) = session_cookie_opt {
        let response: String = format!("Session cookie: {}", session);
        let _ = ctx.set_response_body(response).await;
    }
}

#[send]
#[request_version(http_version)]
async fn request_version_test(ctx: Context) {
    let response: String = format!("HTTP Version: {:?}", http_version);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[request_path(request_path)]
async fn request_path_test(ctx: Context) {
    let response: String = format!("Request Path: {:?}", request_path);
    let _ = ctx.set_response_body(response).await;
}

#[send]
#[response_header("X-Test-Header", "set-value")]
#[response_header("X-Replace-Header" => "replace-value")]
async fn header_operations_test(ctx: Context) {
    let _ = ctx
        .set_response_body("Testing header set and replace operations")
        .await;
}

#[send]
#[response_status_code(201)]
#[response_reason_phrase(HttpStatus::Created.to_string())]
#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
#[response_body("{\"message\": \"Resource created\"}")]
#[response_version(HttpVersion::HTTP1_1)]
async fn literals(ctx: Context) {}

#[tokio::main]
#[hyperlane(server)]
async fn main() {
    server.route("/response", response).await;
    server.route("/connect", connect).await;
    server.route("/delete", delete).await;
    server.route("/head", head).await;
    server.route("/options", options).await;
    server.route("/patch", patch).await;
    server.route("/put", put).await;
    server.route("/trace", trace).await;
    server.route("/h2c", h2c).await;
    server.route("/http", http_only).await;
    server.route("/http0_9", http0_9).await;
    server.route("/http1_0", http1_0).await;
    server.route("/http1_1", http1_1).await;
    server.route("/http2", http2).await;
    server.route("/http3", http3).await;
    server.route("/tls", tls).await;
    server.route("/http1_1_or_higher", http1_1_or_higher).await;
    server.route("/unknown_method", unknown_method).await;
    server.route("/unknown_upgrade", unknown_upgrade).await;
    server.route("/unknown_version", unknown_version).await;
    server.route("/unknown_all", unknown_all).await;
    server.route("/get", get).await;
    server.route("/post", post).await;
    server.route("/websocket", websocket).await;
    server.route("/ctx_hook", ctx_hook).await;
    server.route("/get_post", get_post).await;
    server.route("/attributes", attributes).await;
    server.route("/route_params/:test", route_params).await;
    server.route("/request_querys", request_querys).await;
    server.route("/request_headers", request_headers).await;
    server.route("/route_param/:test", route_param).await;
    server.route("/request_query", request_query).await;
    server.route("/request_header", request_header).await;
    server.route("/request_body", request_body).await;
    server.route("/host", host).await;
    server.route("/host_filter", host_filter).await;
    server.route("/attribute", attribute).await;
    server.route("/request_body_json", request_body_json).await;
    server.route("/referer", referer).await;
    server.route("/referer_filter", referer_filter).await;
    server.route("/cookies", cookies).await;
    server.route("/cookie", cookie).await;
    server.route("/request_version", request_version_test).await;
    server.route("/request_path", request_path_test).await;
    server
        .route("/header_operations", header_operations_test)
        .await;
    server.route("/literals", literals).await;
    let server_hook: ServerHook = server.run().await.unwrap_or_default();
    let server_hook_clone: ServerHook = server_hook.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_hook.shutdown().await;
    });
    server_hook_clone.wait().await;
}
