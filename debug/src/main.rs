use hyperlane::*;
use hyperlane_macros::*;
use serde::{Deserialize, Serialize};

const STEP: &str = "step";
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

#[send]
#[panic_hook]
#[panic_hook(1)]
#[panic_hook("2")]
#[response_body("panic_hook")]
async fn panic_hook(ctx: Context) {}

#[route("/disable_http_hook")]
#[response_body("disable_http_hook")]
#[disable_http_hook("/disable_http_hook")]
async fn disable_http_hook(ctx: Context) {}

#[route("/disable_ws_hook")]
#[response_body("disable_ws_hook")]
#[disable_ws_hook("/disable_ws_hook")]
async fn disable_ws_hook(ctx: Context) {}

#[connected_hook]
#[connected_hook(1)]
#[connected_hook("2")]
#[response_header(STEP => "connected_hook")]
async fn connected_hook(ctx: Context) {}

#[pre_upgrade_hook]
#[pre_upgrade_hook(1)]
#[pre_upgrade_hook("2")]
#[response_header(STEP => "pre_upgrade_hook")]
async fn pre_upgrade_hook(ctx: Context) {}

#[request_middleware]
#[response_header(SERVER => HYPERLANE)]
#[response_version(HttpVersion::HTTP1_1)]
#[response_header(STEP => "request_middleware_1")]
async fn request_middleware_1(ctx: Context) {}

#[request_middleware(2)]
#[response_header(STEP => "request_middleware_2")]
async fn request_middleware_2(ctx: Context) {}

#[request_middleware("3")]
#[response_header(STEP => "request_middleware_3")]
async fn request_middleware_3(ctx: Context) {}

#[response_middleware]
#[response_header(STEP => "response_middleware_1")]
async fn response_middleware_1(ctx: Context) {}

#[send]
#[flush]
#[response_middleware(2)]
#[response_header(STEP => "response_middleware_2")]
#[reject(ctx.get_request().await.is_ws())]
async fn response_middleware_2(ctx: Context) {}

#[send_body]
#[flush]
#[response_middleware("3")]
#[response_header(STEP => "response_middleware_3")]
#[ws]
async fn response_middleware_3(ctx: Context) {}

#[get]
#[http]
async fn pre_hook(ctx: Context) {}

#[response_status_code(200)]
async fn post_hook(ctx: Context) {}

#[route("/response")]
#[response_body(RESPONSE_DATA)]
#[response_reason_phrase(CUSTOM_REASON)]
#[response_status_code(CUSTOM_STATUS_CODE)]
#[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
async fn response(ctx: Context) {}

#[route("/connect")]
#[response_body("connect")]
#[connect]
async fn connect(ctx: Context) {}

#[route("/delete")]
#[response_body("delete")]
#[delete]
async fn delete(ctx: Context) {}

#[route("/head")]
#[response_body("head")]
#[head]
async fn head(ctx: Context) {}

#[route("/options")]
#[response_body("options")]
#[options]
async fn options(ctx: Context) {}

#[route("/patch")]
#[response_body("patch")]
#[patch]
async fn patch(ctx: Context) {}

#[route("/put")]
#[response_body("put")]
#[put]
async fn put(ctx: Context) {}

#[route("/trace")]
#[response_body("trace")]
#[trace]
async fn trace(ctx: Context) {}

#[route("/h2c")]
#[response_body("h2c")]
#[h2c]
async fn h2c(ctx: Context) {}

#[route("/http")]
#[response_body("http")]
#[http]
async fn http_only(ctx: Context) {}

#[route("/http0_9")]
#[response_body("http0.9")]
#[http0_9]
async fn http0_9(ctx: Context) {}

#[route("/http1_0")]
#[response_body("http1.0")]
#[http1_0]
async fn http1_0(ctx: Context) {}

#[route("/http1_1")]
#[response_body("http1.1")]
#[http1_1]
async fn http1_1(ctx: Context) {}

#[route("/http2")]
#[response_body("http2")]
#[http2]
async fn http2(ctx: Context) {}

#[route("/http3")]
#[response_body("http3")]
#[http3]
async fn http3(ctx: Context) {}

#[route("/tls")]
#[response_body("tls")]
#[tls]
async fn tls(ctx: Context) {}

#[http1_1_or_higher]
#[route("/http1_1_or_higher")]
#[response_body("http1.1+")]
async fn http1_1_or_higher(ctx: Context) {}

#[route("/unknown_method")]
#[response_body("unknown method")]
#[filter(ctx.get_request().await.is_unknown_method())]
async fn unknown_method(ctx: Context) {}

#[route("/get")]
#[send_once_body]
#[response_body("get")]
#[ws]
#[get]
async fn get(ctx: Context) {}

#[send_once]
#[route("/post")]
#[response_body("post")]
#[post]
async fn post(ctx: Context) {}

#[route("/websocket")]
#[response_body("websocket")]
#[ws]
async fn websocket(ctx: Context) {}

#[route("/hook")]
#[pre_hook(pre_hook)]
#[post_hook(post_hook)]
#[response_body("Testing hook macro")]
async fn hook(ctx: Context) {}

#[closed]
#[route("/get_post")]
#[methods(get, post)]
#[response_reason_phrase("OK")]
#[response_status_code(200)]
#[response_body("get_post")]
#[http]
async fn get_post(ctx: Context) {}

#[route("/attributes")]
#[response_body(format!("request attributes: {request_attributes:?}"))]
#[attributes(request_attributes)]
async fn attributes(ctx: Context) {}

#[route("/route_params/:test")]
#[response_body(format!("request route params: {request_route_params:?}"))]
#[route_params(request_route_params)]
async fn route_params(ctx: Context) {}

#[route("/request_querys")]
#[response_body(format!("request querys: {request_querys:?}"))]
#[request_querys(request_querys)]
async fn request_querys(ctx: Context) {}

#[route("/request_headers")]
#[response_body(format!("request headers: {request_headers:?}"))]
#[request_headers(request_headers)]
async fn request_headers(ctx: Context) {}

#[route("/route_param/:test")]
#[response_body(format!("route param: {request_route_param:?}"))]
#[route_param("test" => request_route_param)]
async fn route_param(ctx: Context) {}

#[route("/request_query")]
#[response_body(format!("request query: {request_query_option:?}"))]
#[request_query("test" => request_query_option)]
async fn request_query(ctx: Context) {}

#[route("/request_header")]
#[response_body(format!("request header: {request_header_option:?}"))]
#[request_header(HOST => request_header_option)]
async fn request_header(ctx: Context) {}

#[response_body(format!("raw body: {raw_body:?}"))]
#[request_body(raw_body)]
#[route("/request_body")]
async fn request_body(ctx: Context) {}

#[route("/host")]
#[host("localhost")]
#[response_body("host string literal: localhost")]
async fn host(ctx: Context) {}

#[route("/host_filter")]
#[host_filter("filter.localhost")]
#[response_body("host filter string literal")]
async fn host_filter(ctx: Context) {}

#[route("/attribute")]
#[response_body(format!("request attribute: {request_attribute_option:?}"))]
#[attribute(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
async fn attribute(ctx: Context) {}

#[route("/request_body_json")]
#[response_body(format!("request data: {request_data_result:?}"))]
#[request_body_json(request_data_result: TestData)]
async fn request_body_json(ctx: Context) {}

#[route("/referer")]
#[referer("http://localhost")]
#[response_body("referer string literal: http://localhost")]
async fn referer(ctx: Context) {}

#[route("/referer_filter")]
#[referer_filter("http://localhost")]
#[response_body("referer filter string literal")]
async fn referer_filter(ctx: Context) {}

#[route("/cookies")]
#[response_body(format!("All cookies: {cookie_value:?}"))]
#[request_cookies(cookie_value)]
async fn cookies(ctx: Context) {}

#[route("/cookie")]
#[response_body(format!("Session cookie: {session_cookie_opt:?}"))]
#[request_cookie("test" => session_cookie_opt)]
async fn cookie(ctx: Context) {}

#[route("/request_version")]
#[response_body(format!("HTTP Version: {http_version}"))]
#[request_version(http_version)]
async fn request_version_test(ctx: Context) {}

#[route("/request_path")]
#[response_body(format!("Request Path: {request_path}"))]
#[request_path(request_path)]
async fn request_path_test(ctx: Context) {}

#[route("/response_header")]
#[response_body("Testing header set and replace operations")]
#[response_header("X-Add-Header", "add-value")]
#[response_header("X-Set-Header" => "set-value")]
async fn response_header_test(ctx: Context) {}

#[route("/literals")]
#[response_status_code(201)]
#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
#[response_body("{\"message\": \"Resource created\"}")]
#[response_reason_phrase(HttpStatus::Created.to_string())]
async fn literals(ctx: Context) {}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.disable_nodelay().await;
    server.config(config).await;
    let server_hook: ServerHook = server.run().await.unwrap_or_default();
    let server_hook_clone: ServerHook = server_hook.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_hook.shutdown().await;
    });
    server_hook_clone.wait().await;
}
