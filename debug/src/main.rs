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

#[task_panic]
#[task_panic(1)]
#[task_panic("2")]
struct TakPanicHook;

impl ServerHook for TakPanicHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        task_panic_data_option(task_panic_data_option),
        task_panic_data(task_panic_data)
    )]
    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(500),
        response_body(format!("{task_panic_data} {task_panic_data_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[request_error]
#[request_error(1)]
#[request_error("2")]
struct RequestErrorHook;

impl ServerHook for RequestErrorHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        request_error_data_option(request_error_data_option),
        request_error_data(request_error_data)
    )]
    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(500),
        response_body(format!("{request_error_data} {request_error_data_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[request_middleware]
struct RequestMiddleware;

impl ServerHook for RequestMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(200),
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
        response_header(SEC_WEBSOCKET_ACCEPT => &WebSocketFrame::generate_accept_key(ctx.get_request_header_back(SEC_WEBSOCKET_KEY).await)),
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
    #[response_version(HttpVersion::Http1_1)]
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
    #[epilogue_macros(try_send, flush)]
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
    #[epilogue_macros(try_send, flush)]
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

async fn prologue_hooks_fn(ctx: &Context) {
    let hook = PrologueHooks::new(ctx).await;
    hook.handle(ctx).await;
}

async fn epilogue_hooks_fn(ctx: &Context) {
    let hook = EpilogueHooks::new(ctx).await;
    hook.handle(ctx).await;
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

    #[prologue_macros(ws, get, response_body("get"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/post")]
struct Post;

impl ServerHook for Post {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(post, response_body("post"), try_send)]
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
        ctx.send_body_list_with_data(&body_list).await;
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
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws3")]
struct Websocket3;

impl ServerHook for Websocket3 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(RequestConfigData::default(), request)]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws4")]
struct Websocket4;

impl ServerHook for Websocket4 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(request, RequestConfigData::default())]
    async fn handle(self, ctx: &Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws5")]
struct Websocket5;

impl ServerHook for Websocket5 {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[ws_from_stream(RequestConfigData::default())]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        ctx.send_body_list_with_data(&body_list).await;
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

#[route("/route_param_option/:test")]
struct RouteParamOption;

impl ServerHook for RouteParamOption {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param_option1:?} {request_route_param_option2:?} {request_route_param_option3:?}"))]
    #[route_param_option("test1" => request_route_param_option1)]
    #[route_param_option("test2" => request_route_param_option2, "test3" => request_route_param_option3)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/route_param/:test")]
struct RouteParam;

impl ServerHook for RouteParam {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param1} {request_route_param2} {request_route_param3}"))]
    #[route_param("test1" => request_route_param1)]
    #[route_param("test2" => request_route_param2, "test3" => request_route_param3)]
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

#[route("/request_query_option")]
struct RequestQueryOption;

impl ServerHook for RequestQueryOption {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query_option("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send,
        http_from_stream(RequestConfigData::default())
    )]
    #[prologue_macros(
        request_query_option("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_query")]
struct RequestQuery;

impl ServerHook for RequestQuery {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query("test" => request_query),
        response_body(&format!("request query: {request_query}")),
        send,
        http_from_stream(RequestConfigData::default())
    )]
    #[prologue_macros(
        request_query("test" => request_query),
        response_body(&format!("request query: {request_query}")),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_header_option")]
struct RequestHeaderOption;

impl ServerHook for RequestHeaderOption {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_header_option(HOST => request_header_option),
        response_body(&format!("request header: {request_header_option:?}")),
        send,
        http_from_stream(_request)
    )]
    #[prologue_macros(
        request_header_option(HOST => request_header_option),
        response_body(&format!("request header: {request_header_option:?}")),
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
        request_header(HOST => request_header),
        response_body(&format!("request header: {request_header}")),
        send,
        http_from_stream(_request)
    )]
    #[prologue_macros(
        request_header(HOST => request_header),
        response_body(&format!("request header: {request_header}")),
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
        http_from_stream(RequestConfigData::default(), _request)
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
        http_from_stream(_request, RequestConfigData::default())
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

#[route("/attribute_option")]
struct AttributeOption;

impl ServerHook for AttributeOption {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute_option:?}"))]
    #[attribute_option(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/attribute")]
struct Attribute;

impl ServerHook for Attribute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute:?}"))]
    #[attribute(TEST_ATTRIBUTE_KEY => request_attribute: TestData)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_body_json_result")]
struct RequestBodyJsonResult;

impl ServerHook for RequestBodyJsonResult {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json_result(request_data_result: TestData)]
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

#[route("/request_cookie_option")]
struct CookieOption;

impl ServerHook for CookieOption {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
    #[request_cookie_option("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/request_cookie")]
struct Cookie;

impl ServerHook for Cookie {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1}, {session_cookie2}"))]
    #[request_cookie("test1" => session_cookie1, "test2" => session_cookie2)]
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

#[route("/inject/response_body")]
struct InjectResponseBody;

impl ServerHook for InjectResponseBody {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.response_body_with_ref_self(ctx).await;
    }
}

impl InjectResponseBody {
    #[response_body("response body with ref self")]
    async fn response_body_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/post_method")]
struct InjectPostMethod;

impl ServerHook for InjectPostMethod {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.post_method_with_ref_self(ctx).await;
    }
}

impl InjectPostMethod {
    #[prologue_macros(post, response_body("post method with ref self"))]
    async fn post_method_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/send_flush")]
struct InjectSendFlush;

impl ServerHook for InjectSendFlush {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.send_and_flush_with_ref_self(ctx).await;
    }
}

impl InjectSendFlush {
    #[epilogue_macros(try_send, flush)]
    async fn send_and_flush_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/request_body")]
struct InjectRequestBody;

impl ServerHook for InjectRequestBody {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.extract_request_body_with_ref_self(ctx).await;
    }
}

impl InjectRequestBody {
    #[request_body(_raw_body)]
    async fn extract_request_body_with_ref_self(&self, _ctx: &Context) {}
}

#[route("/inject/multiple_methods")]
struct InjectMultipleMethods;

impl ServerHook for InjectMultipleMethods {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.multiple_methods_with_ref_self(ctx).await;
    }
}

impl InjectMultipleMethods {
    #[methods(get, post)]
    async fn multiple_methods_with_ref_self(&self, ctx: &Context) {}
}

#[route("/inject/http_stream")]
struct InjectHttpStream;

impl ServerHook for InjectHttpStream {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.http_stream_handler_with_ref_self(ctx).await;
    }
}

impl InjectHttpStream {
    #[http_from_stream(RequestConfigData::default(), _request)]
    async fn http_stream_handler_with_ref_self(&self, _ctx: &Context) {}
}

#[route("/inject/ws_stream")]
struct InjectWsStream;

impl ServerHook for InjectWsStream {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.websocket_stream_handler_with_ref_self(ctx).await;
    }
}

impl InjectWsStream {
    #[ws_from_stream(_request)]
    async fn websocket_stream_handler_with_ref_self(&self, _ctx: &Context) {}
}

#[route("/inject/complex_post")]
struct InjectComplexPost;

impl ServerHook for InjectComplexPost {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        self.complex_post_handler_with_ref_self(ctx).await;
    }
}

impl InjectComplexPost {
    #[prologue_macros(
        post,
        http,
        request_body(raw_body),
        response_status_code(201),
        response_header(CONTENT_TYPE => APPLICATION_JSON),
        response_body(&format!("Received: {raw_body:?}"))
    )]
    #[epilogue_macros(try_send, flush)]
    async fn complex_post_handler_with_ref_self(&self, ctx: &Context) {}
}

impl InjectComplexPost {
    #[post]
    async fn test_with_bool_param(_a: bool, ctx: &Context) {}

    #[get]
    async fn test_with_multiple_params(_a: bool, ctx: &Context, _b: i32) {}
}

#[route("/test/send")]
struct TestSend;

impl ServerHook for TestSend {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test send operation")
    )]
    #[epilogue_macros(send)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/send_body")]
struct TestSendBody;

impl ServerHook for TestSendBody {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test send body operation")
    )]
    #[epilogue_macros(send_body)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/send_body_with_data")]
struct TestSendBodyWithData;

impl ServerHook for TestSendBodyWithData {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN)
    )]
    #[epilogue_macros(send_body_with_data("Custom data from send_body_with_data"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/try_send")]
struct TestTrySend;

impl ServerHook for TestTrySend {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try send operation")
    )]
    #[epilogue_macros(try_send)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/try_send_body")]
struct TestTrySendBody;

impl ServerHook for TestTrySendBody {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try send body operation")
    )]
    #[epilogue_macros(try_send_body)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/try_send_body_with_data")]
struct TestTrySendBodyWithData;

impl ServerHook for TestTrySendBodyWithData {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN)
    )]
    #[epilogue_macros(try_send_body_with_data("Custom data from try_send_body_with_data"))]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/try_flush")]
struct TestTryFlush;

impl ServerHook for TestTryFlush {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try flush operation")
    )]
    #[epilogue_macros(try_flush)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/aborted")]
struct TestAborted;

impl ServerHook for TestAborted {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test aborted operation")
    )]
    #[epilogue_macros(aborted)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/closed")]
struct TestClosed;

impl ServerHook for TestClosed {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test closed operation")
    )]
    #[epilogue_macros(closed)]
    async fn handle(self, ctx: &Context) {}
}

#[route("/test/flush")]
struct TestFlush;

impl ServerHook for TestFlush {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test flush operation")
    )]
    #[epilogue_macros(flush)]
    async fn handle(self, ctx: &Context) {}
}

#[response_body("standalone response body")]
async fn standalone_response_body_handler(ctx: &Context) {}

#[prologue_macros(get, response_body("standalone get handler"))]
async fn standalone_get_handler(ctx: &Context) {}

#[epilogue_macros(try_send, flush)]
async fn standalone_send_and_flush_handler(ctx: &Context) {}

#[request_body(_raw_body)]
async fn standalone_request_body_extractor(ctx: &Context) {}

#[methods(get, post)]
async fn standalone_multiple_methods_handler(ctx: &Context) {}

#[http_from_stream]
async fn standalone_http_stream_handler(ctx: &Context) {}

#[ws_from_stream]
async fn standalone_websocket_stream_handler(ctx: &Context) {}

#[aborted]
async fn standalone_aborted_handler(ctx: &Context) {}

#[closed]
async fn standalone_closed_handler(ctx: &Context) {}

#[flush]
async fn standalone_flush_handler(ctx: &Context) {}

#[try_flush]
async fn standalone_try_flush_handler(ctx: &Context) {}

#[ws]
async fn standalone_ws_handler(ctx: &Context) {}

#[prologue_macros(
    get,
    http,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("standalone complex handler")
)]
#[epilogue_macros(try_send, flush)]
async fn standalone_complex_get_handler(ctx: &Context) {}

#[get]
async fn standalone_get_handler_with_param(_a: bool, ctx: &Context) {}

#[request_body(body1, body2, body3)]
async fn test_multi_request_body(ctx: &Context) {
    println!("body1: {:?}, body2: {:?}, body3: {:?}", body1, body2, body3);
}

#[route("/test_multi_request_body_json")]
#[derive(Debug, serde::Deserialize)]
struct User {
    name: String,
}

impl ServerHook for User {
    async fn new(_ctx: &Context) -> Self {
        Self {
            name: String::from("test"),
        }
    }

    #[prologue_macros(
        request_body_json(user1: User, user2: User),
        response_body(format!(
            "user1: {:?}, user2: {:?}",
            user1.name,
            user2.name
        )),
        try_send
    )]
    async fn handle(self, ctx: &Context) {}
}

#[attribute("key1" => attr1: String, "key2" => attr2: i32)]
async fn test_multi_attribute(ctx: &Context) {
    println!("attr1: {:?}, attr2: {:?}", attr1, attr2);
}

#[attributes(attrs1, attrs2)]
async fn test_multi_attributes(ctx: &Context) {
    println!("attrs1: {:?}, attrs2: {:?}", attrs1, attrs2);
}

#[route_params(params1, params2)]
async fn test_multi_route_params(ctx: &Context) {
    println!("params1: {:?}, params2: {:?}", params1, params2);
}

#[request_querys(querys1, querys2)]
async fn test_multi_request_querys(ctx: &Context) {
    println!("querys1: {:?}, querys2: {:?}", querys1, querys2);
}

#[request_headers(headers1, headers2)]
async fn test_multi_request_headers(ctx: &Context) {
    println!("headers1: {:?}, headers2: {:?}", headers1, headers2);
}

#[request_cookies(cookies1, cookies2)]
async fn test_multi_request_cookies(ctx: &Context) {
    println!("cookies1: {:?}, cookies2: {:?}", cookies1, cookies2);
}

#[request_version(version1, version2)]
async fn test_multi_request_version(ctx: &Context) {
    println!("version1: {:?}, version2: {:?}", version1, version2);
}

#[request_path(path1, path2)]
async fn test_multi_request_path(ctx: &Context) {
    println!("path1: {:?}, path2: {:?}", path1, path2);
}

#[host("localhost", "127.0.0.1")]
async fn test_multi_host(ctx: &Context) {
    println!("Host check passed");
}

#[reject_host("localhost", "127.0.0.1")]
async fn test_multi_reject_host(ctx: &Context) {
    println!("Reject host check passed");
}

#[referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_referer(ctx: &Context) {
    println!("Referer check passed");
}

#[reject_referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_reject_referer(ctx: &Context) {
    println!("Reject referer check passed");
}

#[hyperlane(server1: Server, server2: Server)]
async fn test_multi_hyperlane() {
    println!("server1 and server2 initialized");
}

#[response_status_code(200)]
async fn standalone_response_status_code_handler(_ctx: &Context) {}

#[response_reason_phrase("Custom Reason")]
async fn standalone_response_reason_phrase_handler(_ctx: &Context) {}

#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
async fn standalone_response_header_handler(_ctx: &Context) {}

#[response_header("X-Custom-Header", "custom-value")]
async fn standalone_response_header_with_comma_handler(_ctx: &Context) {}

#[response_version(HttpVersion::Http1_1)]
async fn standalone_response_version_handler(_ctx: &Context) {}

#[connect]
async fn standalone_connect_handler(_ctx: &Context) {}

#[delete]
async fn standalone_delete_handler(_ctx: &Context) {}

#[head]
async fn standalone_head_handler(_ctx: &Context) {}

#[options]
async fn standalone_options_handler(_ctx: &Context) {}

#[patch]
async fn standalone_patch_handler(_ctx: &Context) {}

#[put]
async fn standalone_put_handler(_ctx: &Context) {}

#[h2c]
async fn standalone_h2c_handler(_ctx: &Context) {}

#[http0_9]
async fn standalone_http0_9_handler(_ctx: &Context) {}

#[http1_0]
async fn standalone_http1_0_handler(_ctx: &Context) {}

#[http1_1]
async fn standalone_http1_1_handler(_ctx: &Context) {}

#[http1_1_or_higher]
async fn standalone_http1_1_or_higher_handler(_ctx: &Context) {}

#[http3]
async fn standalone_http3_handler(_ctx: &Context) {}

#[tls]
async fn standalone_tls_handler(_ctx: &Context) {}

#[methods(get, post, put)]
async fn standalone_methods_multiple_handler(_ctx: &Context) {}

#[filter(_ctx.get_request().await.is_get())]
async fn standalone_filter_handler(_ctx: &Context) {}

#[reject(_ctx.get_request().await.is_post())]
async fn standalone_reject_handler(_ctx: &Context) {}

#[reject_host("example.com")]
async fn standalone_reject_host_handler(_ctx: &Context) {}

#[referer("https://example.com")]
async fn standalone_referer_handler(_ctx: &Context) {}

#[reject_referer("https://malicious.com")]
async fn standalone_reject_referer_handler(_ctx: &Context) {}

#[request_query("param" => _value)]
async fn standalone_request_query_handler(_ctx: &Context) {}

#[request_query_option("optional_param" => _optional_value)]
async fn standalone_request_query_option_handler(_ctx: &Context) {}

#[request_header(HOST => _host_value)]
async fn standalone_request_header_handler(_ctx: &Context) {}

#[request_header_option(USER_AGENT => _user_agent)]
async fn standalone_request_header_option_handler(_ctx: &Context) {}

#[request_querys(_querys)]
async fn standalone_request_querys_handler(_ctx: &Context) {}

#[request_headers(_headers)]
async fn standalone_request_headers_handler(_ctx: &Context) {}

#[request_cookies(_cookies)]
async fn standalone_request_cookies_handler(_ctx: &Context) {}

#[request_cookie("session" => _session_cookie)]
async fn standalone_request_cookie_handler(_ctx: &Context) {}

#[request_cookie_option("optional_cookie" => _optional_cookie)]
async fn standalone_request_cookie_option_handler(_ctx: &Context) {}

#[request_version(_version)]
async fn standalone_request_version_handler(_ctx: &Context) {}

#[request_path(_path)]
async fn standalone_request_path_handler(_ctx: &Context) {}

#[attribute("key" => _attr_value: String)]
async fn standalone_attribute_handler(_ctx: &Context) {}

#[attribute_option("optional_key" => _optional_attr: String)]
async fn standalone_attribute_option_handler(_ctx: &Context) {}

#[attributes(_attrs)]
async fn standalone_attributes_handler(_ctx: &Context) {}

#[route_params(_params)]
async fn standalone_route_params_handler(_ctx: &Context) {}

#[route_param("param" => _param_value)]
async fn standalone_route_param_handler(_ctx: &Context) {}

#[route_param_option("optional_param" => _optional_param_value)]
async fn standalone_route_param_option_handler(_ctx: &Context) {}

#[request_body_json(_user: TestData)]
async fn standalone_request_body_json_handler(_ctx: &Context) {}

#[request_body_json_result(_user_result: TestData)]
async fn standalone_request_body_json_result_handler(_ctx: &Context) {}

#[http_from_stream(RequestConfigData::default())]
async fn standalone_http_from_stream_with_config_handler(_ctx: &Context) {}

#[ws_from_stream(RequestConfigData::default())]
async fn standalone_ws_from_stream_with_config_handler(_ctx: &Context) {}

#[http_from_stream(_request)]
async fn standalone_http_from_stream_with_request_handler(_ctx: &Context) {}

#[ws_from_stream(_request)]
async fn standalone_ws_from_stream_with_request_handler(_ctx: &Context) {}

#[http_from_stream(RequestConfigData::default(), _request)]
async fn standalone_http_from_stream_full_handler(_ctx: &Context) {}

#[ws_from_stream(RequestConfigData::default(), _request)]
async fn standalone_ws_from_stream_full_handler(_ctx: &Context) {}

#[send]
async fn standalone_send_handler_2(_ctx: &Context) {}

#[send_body]
async fn standalone_send_body_handler_2(_ctx: &Context) {}

#[send_body_with_data("Custom send body data")]
async fn standalone_send_body_with_data_handler_2(_ctx: &Context) {}

#[try_send]
async fn standalone_try_send_handler_2(_ctx: &Context) {}

#[try_send_body]
async fn standalone_try_send_body_handler_2(_ctx: &Context) {}

#[try_send_body_with_data("Custom try send body data")]
async fn standalone_try_send_body_with_data_handler_2(_ctx: &Context) {}

#[flush]
async fn standalone_flush_handler_2(_ctx: &Context) {}

#[try_flush]
async fn standalone_try_flush_handler_2(_ctx: &Context) {}

#[aborted]
async fn standalone_aborted_handler_2(_ctx: &Context) {}

#[closed]
async fn standalone_closed_handler_2(_ctx: &Context) {}

#[clear_response_headers]
async fn standalone_clear_response_headers_handler(_ctx: &Context) {}

#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("prologue macros test")
)]
async fn standalone_prologue_macros_complex_handler(_ctx: &Context) {}

#[epilogue_macros(
    response_status_code(201),
    response_header(CONTENT_TYPE => APPLICATION_JSON),
    response_body("epilogue macros test"),
    try_send,
    flush
)]
async fn standalone_epilogue_macros_complex_handler(_ctx: &Context) {}

#[prologue_hooks(prologue_hooks_fn)]
async fn standalone_prologue_hooks_handler(_ctx: &Context) {}

#[epilogue_hooks(epilogue_hooks_fn)]
async fn standalone_epilogue_hooks_handler(_ctx: &Context) {}

#[route("/hooks_expression")]
struct HooksExpression;

impl ServerHook for HooksExpression {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[get]
    #[prologue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
    #[epilogue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
    #[response_body("hooks expression test")]
    async fn handle(self, ctx: &Context) {}
}

impl HooksExpression {
    async fn new_hook(_ctx: &Context) {}

    async fn method_hook(_ctx: &Context) {}
}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.disable_nodelay().await;
    server.server_config(config).await;
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
