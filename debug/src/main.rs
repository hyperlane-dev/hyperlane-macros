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

#[panic_hook]
#[panic_hook(1)]
#[panic_hook("2")]
struct PanicHook;

impl ServerHook for PanicHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(response_body("panic_hook"), send)]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware]
struct RequestMiddleware;

impl ServerHook for RequestMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        response_status_code(200),
        response_version(HttpVersion::HTTP1_1),
        response_header(SERVER => HYPERLANE),
        response_header(CONNECTION => KEEP_ALIVE),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY),
        response_header(STEP => "request_middleware"),
    )]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware(1)]
struct UpgradeHook;

impl ServerHook for UpgradeHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        ws,
        response_body(&vec![]),
        response_status_code(101),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => &WebSocketFrame::generate_accept_key(ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await.unwrap())),
        response_header(STEP => "upgrade_hook"),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware(2)]
struct ConnectedHook;

impl ServerHook for ConnectedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    #[response_header(SERVER => HYPERLANE)]
    #[response_version(HttpVersion::HTTP1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(STEP => "connected_hook")]
    async fn handle(self, ctx: &Context) {}
}

#[response_middleware]
struct ResponseMiddleware1;

impl ServerHook for ResponseMiddleware1 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_header(STEP => "response_middleware_1")]
    async fn handle(self, ctx: &Context) {}
}

#[response_middleware(2)]
struct ResponseMiddleware2;

impl ServerHook for ResponseMiddleware2 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request().await.is_ws()),
        response_header(STEP => "response_middleware_2")
    )]
    #[epilogue_macros(send, flush)]
    async fn handle(self, ctx: &Context) {}
}

#[response_middleware("3")]
struct ResponseMiddleware3;

impl ServerHook for ResponseMiddleware3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        ws,
        response_header(STEP => "response_middleware_3")
    )]
    #[epilogue_macros(send_body, flush)]
    async fn handle(self, ctx: &Context) {}
}

struct PrologueHooks;

impl ServerHook for PrologueHooks {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[get]
    #[http]
    async fn handle(self, _ctx: &Context) {}
}

struct EpilogueHooks;

impl ServerHook for EpilogueHooks {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    async fn handle(self, ctx: &Context) {}
}

async fn prologue_hooks_fn(ctx: Context) {
    let hook = PrologueHooks::new(&ctx).await;
    hook.handle(&ctx).await;
}

async fn epilogue_hooks_fn(ctx: Context) {
    let hook = EpilogueHooks::new(&ctx).await;
    hook.handle(&ctx).await;
}

#[route("/response")]
struct Response;

impl ServerHook for Response {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&RESPONSE_DATA)]
    #[response_reason_phrase(CUSTOM_REASON)]
    #[response_status_code(CUSTOM_STATUS_CODE)]
    #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/connect")]
struct Connect;

impl ServerHook for Connect {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(connect, response_body("connect"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/delete")]
struct Delete;

impl ServerHook for Delete {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(delete, response_body("delete"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/head")]
struct Head;

impl ServerHook for Head {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(head, response_body("head"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/options")]
struct Options;

impl ServerHook for Options {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(options, response_body("options"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/patch")]
struct Patch;

impl ServerHook for Patch {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(patch, response_body("patch"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/put")]
struct Put;

impl ServerHook for Put {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(put, response_body("put"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/trace")]
struct Trace;

impl ServerHook for Trace {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(trace, response_body("trace"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/h2c")]
struct H2c;

impl ServerHook for H2c {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(h2c, response_body("h2c"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http")]
struct HttpOnly;

impl ServerHook for HttpOnly {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http, response_body("http"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http0_9")]
struct Http09;

impl ServerHook for Http09 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http0_9, response_body("http0_9"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http1_0")]
struct Http10;

impl ServerHook for Http10 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http1_0, response_body("http1_0"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http1_1")]
struct Http11;

impl ServerHook for Http11 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1, response_body("http1_1"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http2")]
struct Http2;

impl ServerHook for Http2 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http2, response_body("http2"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http3")]
struct Http3;

impl ServerHook for Http3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http3, response_body("http3"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/tls")]
struct Tls;

impl ServerHook for Tls {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(tls, response_body("tls"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/http1_1_or_higher")]
struct Http11OrHigher;

impl ServerHook for Http11OrHigher {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1_or_higher, response_body("http1_1_or_higher"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/unknown_method")]
struct UnknownMethod;

impl ServerHook for UnknownMethod {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        clear_response_headers,
        filter(ctx.get_request().await.is_unknown_method()),
        response_body("unknown_method")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/get")]
struct Get;

impl ServerHook for Get {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws, get, response_body("get"), send_body_once)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/post")]
struct Post;

impl ServerHook for Post {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(post, response_body("post"), send_once)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/ws1")]
struct Websocket1;

impl ServerHook for Websocket1 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws2")]
struct Websocket2;

impl ServerHook for Websocket2 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(request)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws3")]
struct Websocket3;

impl ServerHook for Websocket3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(1024, request)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws4")]
struct Websocket4;

impl ServerHook for Websocket4 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(request, 1024)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/ws5")]
struct Websocket5;

impl ServerHook for Websocket5 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(1024)]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        ctx.send_body_list_with_data(&body_list).await.unwrap();
    }
}

#[route("/hook")]
struct Hook;

impl ServerHook for Hook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_hooks(prologue_hooks_fn)]
    #[epilogue_hooks(epilogue_hooks_fn)]
    #[response_body("Testing hook macro")]
    async fn handle(self, ctx: &Context) {}
}

#[route("/get_post")]
struct GetPost;

impl ServerHook for GetPost {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[closed]
    #[prologue_macros(
        http,
        methods(get, post),
        response_body("get_post"),
        response_status_code(200),
        response_reason_phrase("OK")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/attributes")]
struct Attributes;

impl ServerHook for Attributes {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request attributes: {request_attributes:?}"))]
    #[attributes(request_attributes)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/route_params/:test")]
struct RouteParams;

impl ServerHook for RouteParams {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request route params: {request_route_params:?}"))]
    #[route_params(request_route_params)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/route_param/:test")]
struct RouteParam;

impl ServerHook for RouteParam {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param:?}"))]
    #[route_param("test" => request_route_param)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/host")]
struct Host;

impl ServerHook for Host {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[host("localhost")]
    #[epilogue_macros(
        response_body("host string literal: localhost"),
        send,
        http_from_stream
    )]
    #[prologue_macros(response_body("host string literal: localhost"), send)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_query")]
struct RequestQuery;

impl ServerHook for RequestQuery {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send,
        http_from_stream(1024)
    )]
    #[prologue_macros(
        request_query("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_header")]
struct RequestHeader;

impl ServerHook for RequestHeader {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_header(HOST => request_header_option),
        response_body(&format!("request header: {request_header_option:?}")),
        send,
        http_from_stream(_request)
    )]
    #[prologue_macros(
        request_header(HOST => request_header_option),
        response_body(&format!("request header: {request_header_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_querys")]
struct RequestQuerys;

impl ServerHook for RequestQuerys {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send,
        http_from_stream(1024, _request)
    )]
    #[prologue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_headers")]
struct RequestHeaders;

impl ServerHook for RequestHeaders {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send,
        http_from_stream(_request, 1024)
    )]
    #[prologue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_body")]
struct RequestBodyRoute;

impl ServerHook for RequestBodyRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("raw body: {raw_body:?}"))]
    #[request_body(raw_body)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/reject_host")]
struct RejectHost;

impl ServerHook for RejectHost {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_host("filter.localhost"),
        response_body("host filter string literal")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/attribute")]
struct Attribute;

impl ServerHook for Attribute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute_option:?}"))]
    #[attribute(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_body_json")]
struct RequestBodyJson;

impl ServerHook for RequestBodyJson {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json(request_data_result: TestData)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/referer")]
struct Referer;

impl ServerHook for Referer {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        referer("http://localhost"),
        response_body("referer string literal: http://localhost")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/reject_referer")]
struct RejectReferer;

impl ServerHook for RejectReferer {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_referer("http://localhost"),
        response_body("referer filter string literal")
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/cookies")]
struct Cookies;

impl ServerHook for Cookies {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("All cookies: {cookie_value:?}"))]
    #[request_cookies(cookie_value)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/cookie")]
struct Cookie;

impl ServerHook for Cookie {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie_opt:?}"))]
    #[request_cookie("test" => session_cookie_opt)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_version")]
struct RequestVersionTest;

impl ServerHook for RequestVersionTest {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("HTTP Version: {http_version}"))]
    #[request_version(http_version)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_path")]
struct RequestPathTest;

impl ServerHook for RequestPathTest {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("Request Path: {request_path}"))]
    #[request_path(request_path)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/response_header")]
struct ResponseHeaderTest;

impl ServerHook for ResponseHeaderTest {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body("Testing header set and replace operations")]
    #[response_header("X-Add-Header", "add-value")]
    #[response_header("X-Set-Header" => "set-value")]
    async fn handle(self, ctx: &Context) {}
}

#[route("/literals")]
struct Literals;

impl ServerHook for Literals {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(201)]
    #[response_header(CONTENT_TYPE => APPLICATION_JSON)]
    #[response_body("{\"message\": \"Resource created\"}")]
    #[response_reason_phrase(HttpStatus::Created.to_string())]
    async fn handle(self, ctx: &Context) {}
}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.disable_nodelay().await;
    server.config(config).await;
    let server_hook: ServerControlHook = server.run().await.unwrap_or_default();
    let server_hook_clone: ServerControlHook = server_hook.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_hook.shutdown().await;
    });
    server_hook_clone.wait().await;
}
