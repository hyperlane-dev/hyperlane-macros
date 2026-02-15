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

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TestData {
    name: String,
    age: u32,
}

#[task_panic]
#[task_panic(1)]
#[task_panic("2")]
struct TakPanicHook;

impl ServerHook for TakPanicHook {
    async fn new(_ctx: &mut Context) -> Self {
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
    async fn handle(self, ctx: &mut Context) {}
}

#[request_error]
#[request_error(1)]
#[request_error("2")]
struct RequestErrorHook;

impl ServerHook for RequestErrorHook {
    async fn new(_ctx: &mut Context) -> Self {
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
    async fn handle(self, ctx: &mut Context) {}
}

#[request_middleware]
struct RequestMiddleware;

impl ServerHook for RequestMiddleware {
    async fn new(_ctx: &mut Context) -> Self {
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
    async fn handle(self, ctx: &mut Context) {}
}

#[request_middleware(1)]
struct UpgradeHook;

impl ServerHook for UpgradeHook {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        ws_upgrade_type,
        response_body(&vec![]),
        response_status_code(101),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => &WebSocketFrame::generate_accept_key(ctx.get_request().get_header_back(SEC_WEBSOCKET_KEY))),
        response_header(STEP => "upgrade_hook"),
        send
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[request_middleware(2)]
struct ConnectedHook;

impl ServerHook for ConnectedHook {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    #[response_header(SERVER => HYPERLANE)]
    #[response_version(HttpVersion::Http1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(STEP => "connected_hook")]
    async fn handle(self, ctx: &mut Context) {}
}

#[response_middleware]
struct ResponseMiddleware1;

impl ServerHook for ResponseMiddleware1 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_header(STEP => "response_middleware_1")]
    async fn handle(self, ctx: &mut Context) {}
}

#[response_middleware(2)]
struct ResponseMiddleware2;

impl ServerHook for ResponseMiddleware2 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request().get_upgrade_type().is_ws()),
        response_header(STEP => "response_middleware_2")
    )]
    #[epilogue_macros(try_send, flush)]
    async fn handle(self, ctx: &mut Context) {}
}

#[response_middleware("3")]
struct ResponseMiddleware3;

impl ServerHook for ResponseMiddleware3 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        ws_upgrade_type,
        response_header(STEP => "response_middleware_3")
    )]
    #[epilogue_macros(try_send, flush)]
    async fn handle(self, ctx: &mut Context) {}
}

struct PrologueHooks;

impl ServerHook for PrologueHooks {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[get_method]
    #[http_version]
    async fn handle(self, _ctx: &mut Context) {}
}

struct EpilogueHooks;

impl ServerHook for EpilogueHooks {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    async fn handle(self, ctx: &mut Context) {}
}

async fn prologue_hooks_fn(ctx: &mut Context) {
    let hook = PrologueHooks::new(ctx).await;
    hook.handle(ctx).await;
}

async fn epilogue_hooks_fn(ctx: &mut Context) {
    let hook = EpilogueHooks::new(ctx).await;
    hook.handle(ctx).await;
}

#[route("/response")]
struct Response;

impl ServerHook for Response {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&RESPONSE_DATA)]
    #[response_reason_phrase(CUSTOM_REASON)]
    #[response_status_code(CUSTOM_STATUS_CODE)]
    #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/connect")]
struct ConnectMethod;

impl ServerHook for ConnectMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(connect_method, response_body("connect"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/delete")]
struct DeleteMethod;

impl ServerHook for DeleteMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(delete_method, response_body("delete"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/head")]
struct HeadMethod;

impl ServerHook for HeadMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(head_method, response_body("head"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/options")]
struct OptionsMethod;

impl ServerHook for OptionsMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(options_method, response_body("options"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/patch")]
struct PatchMethod;

impl ServerHook for PatchMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(patch_method, response_body("patch"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/put")]
struct PutMethod;

impl ServerHook for PutMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(put_method, response_body("put"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/trace")]
struct TraceMethod;

impl ServerHook for TraceMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(trace_method, response_body("trace"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/get_post_method")]
struct GetPostMethod;

impl ServerHook for GetPostMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[closed]
    #[prologue_macros(
        http_version,
        methods(get, post),
        response_body("get_post_method"),
        response_status_code(200),
        response_reason_phrase("OK")
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/get_method")]
struct GetMethod;

impl ServerHook for GetMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(ws_upgrade_type, get_method, response_body("get_method"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/post_method")]
struct PostMethod;

impl ServerHook for PostMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, response_body("post_method"), try_send)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/unknown_method")]
struct UnknownMethod;

impl ServerHook for UnknownMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(unknown_method, response_body("unknown_method"), try_send)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http0_9_version")]
struct Http09Version;

impl ServerHook for Http09Version {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http0_9_version, response_body("http0_9_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http1_0_version")]
struct Http10Version;

impl ServerHook for Http10Version {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http1_0_version, response_body("http1_0_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http1_1_version")]
struct Http11Version;

impl ServerHook for Http11Version {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1_version, response_body("http1_1_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http2_version")]
struct Http2Version;

impl ServerHook for Http2Version {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http2_version, response_body("http2_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http3_version")]
struct Http3Version;

impl ServerHook for Http3Version {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http3_version, response_body("http3_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http1_1_or_higher_version")]
struct Http11OrHigher;

impl ServerHook for Http11OrHigher {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1_or_higher_version, response_body("http1_1_or_higher_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/http_version")]
struct HttpAllVersion;

impl ServerHook for HttpAllVersion {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http_version, response_body("http_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/unknown_version")]
struct UnknownVersion;

impl ServerHook for UnknownVersion {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(unknown_version, response_body("unknown_version"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/ws_upgrade_type")]
struct WsUpgradeType;

impl ServerHook for WsUpgradeType {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    async fn handle(self, _ctx: &mut Context) {}
}

#[route("/h2c_upgrade_type")]
struct H2cUpgradeType;

impl ServerHook for H2cUpgradeType {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(h2c_upgrade_type, response_body("h2c_upgrade_type"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/tls_upgrade_type")]
struct Tls;

impl ServerHook for Tls {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(tls_upgrade_type, response_body("tls_upgrade_type"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/unknown_upgrade_type")]
struct UnknownUpgradeType;

impl ServerHook for UnknownUpgradeType {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(unknown_upgrade_type, response_body("unknown_upgrade_type"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/ws1")]
struct Websocket1;

impl ServerHook for Websocket1 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[ws_from_stream]
    async fn handle(self, ctx: &mut Context) {
        let body: &RequestBody = ctx.get_request().get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws2")]
struct Websocket2;

impl ServerHook for Websocket2 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[ws_from_stream(request)]
    async fn handle(self, ctx: &mut Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws3")]
struct Websocket3;

impl ServerHook for Websocket3 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[ws_from_stream(request)]
    async fn handle(self, ctx: &mut Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws4")]
struct Websocket4;

impl ServerHook for Websocket4 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[ws_from_stream(request)]
    async fn handle(self, ctx: &mut Context) {
        let body: &RequestBody = request.get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/ws5")]
struct Websocket5;

impl ServerHook for Websocket5 {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[ws_from_stream]
    async fn handle(self, ctx: &mut Context) {
        let body: &RequestBody = ctx.get_request().get_body();
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.send_body_list_with_data(&body_list).await;
    }
}

#[route("/hook")]
struct Hook;

impl ServerHook for Hook {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_hooks(prologue_hooks_fn)]
    #[epilogue_hooks(epilogue_hooks_fn)]
    #[response_body("Testing hook macro")]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/attributes")]
struct Attributes;

impl ServerHook for Attributes {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request attributes: {request_attributes:?}"))]
    #[attributes(request_attributes)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/route_params/:test")]
struct RouteParams;

impl ServerHook for RouteParams {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request route params: {request_route_params:?}"))]
    #[route_params(request_route_params)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/route_param_option/:test")]
struct RouteParamOption;

impl ServerHook for RouteParamOption {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param_option1:?} {request_route_param_option2:?} {request_route_param_option3:?}"))]
    #[route_param_option("test1" => request_route_param_option1)]
    #[route_param_option("test2" => request_route_param_option2, "test3" => request_route_param_option3)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/route_param/:test")]
struct RouteParam;

impl ServerHook for RouteParam {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param1} {request_route_param2} {request_route_param3}"))]
    #[route_param("test1" => request_route_param1)]
    #[route_param("test2" => request_route_param2, "test3" => request_route_param3)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/host")]
struct Host;

impl ServerHook for Host {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[host("localhost")]
    #[epilogue_macros(
        response_body("host string literal: localhost"),
        send,
        http_from_stream
    )]
    #[prologue_macros(response_body("host string literal: localhost"), send)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_query_option")]
struct RequestQueryOption;

impl ServerHook for RequestQueryOption {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query_option("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send,
        http_from_stream
    )]
    #[prologue_macros(
        request_query_option("test" => request_query_option),
        response_body(&format!("request query: {request_query_option:?}")),
        send
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_query")]
struct RequestQuery;

impl ServerHook for RequestQuery {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query("test" => request_query),
        response_body(&format!("request query: {request_query}")),
        send,
        http_from_stream
    )]
    #[prologue_macros(
        request_query("test" => request_query),
        response_body(&format!("request query: {request_query}")),
        send
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_header_option")]
struct RequestHeaderOption;

impl ServerHook for RequestHeaderOption {
    async fn new(_ctx: &mut Context) -> Self {
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
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_header")]
struct RequestHeader;

impl ServerHook for RequestHeader {
    async fn new(_ctx: &mut Context) -> Self {
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
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_querys")]
struct RequestQuerys;

impl ServerHook for RequestQuerys {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send,
        http_from_stream(_request)
    )]
    #[prologue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_headers")]
struct RequestHeaders;

impl ServerHook for RequestHeaders {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send,
        http_from_stream(_request)
    )]
    #[prologue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_body")]
struct RequestBodyRoute;

impl ServerHook for RequestBodyRoute {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("raw body: {raw_body:?}"))]
    #[request_body(raw_body)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/reject_host")]
struct RejectHost;

impl ServerHook for RejectHost {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_host("filter.localhost"),
        response_body("host filter string literal")
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/attribute_option")]
struct AttributeOption;

impl ServerHook for AttributeOption {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute_option:?}"))]
    #[attribute_option(TEST_ATTRIBUTE_KEY => request_attribute_option: TestData)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/attribute")]
struct Attribute;

impl ServerHook for Attribute {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute:?}"))]
    #[attribute(TEST_ATTRIBUTE_KEY => request_attribute: TestData)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_body_json_result")]
struct RequestBodyJsonResult;

impl ServerHook for RequestBodyJsonResult {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json_result(request_data_result: TestData)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_body_json")]
struct RequestBodyJson;

impl ServerHook for RequestBodyJson {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json(request_data_result: TestData)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/referer")]
struct Referer;

impl ServerHook for Referer {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        referer("http://localhost"),
        response_body("referer string literal: http://localhost")
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/reject_referer")]
struct RejectReferer;

impl ServerHook for RejectReferer {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_referer("http://localhost"),
        response_body("referer filter string literal")
    )]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/cookies")]
struct Cookies;

impl ServerHook for Cookies {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("All cookies: {cookie_value:?}"))]
    #[request_cookies(cookie_value)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_cookie_option")]
struct CookieOption;

impl ServerHook for CookieOption {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
    #[request_cookie_option("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_cookie")]
struct Cookie;

impl ServerHook for Cookie {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1}, {session_cookie2}"))]
    #[request_cookie("test1" => session_cookie1, "test2" => session_cookie2)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_version")]
struct RequestVersionTest;

impl ServerHook for RequestVersionTest {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("HTTP Version: {http_version}"))]
    #[request_version(http_version)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/request_path")]
struct RequestPathTest;

impl ServerHook for RequestPathTest {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("Request Path: {request_path}"))]
    #[request_path(request_path)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/response_header")]
struct ResponseHeaderTest;

impl ServerHook for ResponseHeaderTest {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_body("Testing header set and replace operations")]
    #[response_header("X-Add-Header", "add-value")]
    #[response_header("X-Set-Header" => "set-value")]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/literals")]
struct Literals;

impl ServerHook for Literals {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_status_code(201)]
    #[response_header(CONTENT_TYPE => APPLICATION_JSON)]
    #[response_body("{\"message\": \"Resource created\"}")]
    #[response_reason_phrase(HttpStatus::Created.to_string())]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/inject/response_body")]
struct InjectResponseBody;

impl ServerHook for InjectResponseBody {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.response_body_with_ref_self(ctx).await;
    }
}

impl InjectResponseBody {
    #[response_body("response body with ref self")]
    async fn response_body_with_ref_self(&self, ctx: &mut Context) {}
}

#[route("/inject/post_method")]
struct InjectPostMethod;

impl ServerHook for InjectPostMethod {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.post_method_with_ref_self(ctx).await;
    }
}

impl InjectPostMethod {
    #[prologue_macros(post_method, response_body("post method with ref self"))]
    async fn post_method_with_ref_self(&self, ctx: &mut Context) {}
}

#[route("/inject/send_flush")]
struct InjectSendFlush;

impl ServerHook for InjectSendFlush {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.send_and_flush_with_ref_self(ctx).await;
    }
}

impl InjectSendFlush {
    #[epilogue_macros(try_send, flush)]
    async fn send_and_flush_with_ref_self(&self, ctx: &mut Context) {}
}

#[route("/inject/request_body")]
struct InjectRequestBody;

impl ServerHook for InjectRequestBody {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.extract_request_body_with_ref_self(ctx).await;
    }
}

impl InjectRequestBody {
    #[request_body(_raw_body)]
    async fn extract_request_body_with_ref_self(&self, _ctx: &mut Context) {}
}

#[route("/inject/multiple_methods")]
struct InjectMultipleMethods;

impl ServerHook for InjectMultipleMethods {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.multiple_methods_with_ref_self(ctx).await;
    }
}

impl InjectMultipleMethods {
    #[methods(get, post)]
    async fn multiple_methods_with_ref_self(&self, ctx: &mut Context) {}

    #[unknown_method]
    async fn unknown_method_with_ref_self(&self, ctx: &mut Context) {}
}

#[route("/inject/http_stream")]
struct InjectHttpStream;

impl ServerHook for InjectHttpStream {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.http_stream_handler_with_ref_self(ctx).await;
    }
}

impl InjectHttpStream {
    #[http_from_stream(_request)]
    async fn http_stream_handler_with_ref_self(&self, _ctx: &mut Context) {}
}

#[route("/inject/ws_stream")]
struct InjectWsStream;

impl ServerHook for InjectWsStream {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.websocket_stream_handler_with_ref_self(ctx).await;
    }
}

impl InjectWsStream {
    #[ws_from_stream(_request)]
    async fn websocket_stream_handler_with_ref_self(&self, _ctx: &mut Context) {}
}

#[route("/inject/complex_post")]
struct InjectComplexPost;

impl ServerHook for InjectComplexPost {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        self.complex_post_handler_with_ref_self(ctx).await;
    }
}

impl InjectComplexPost {
    #[prologue_macros(
        post_method,
        http_version,
        request_body(raw_body),
        response_status_code(201),
        response_header(CONTENT_TYPE => APPLICATION_JSON),
        response_body(&format!("Received: {raw_body:?}"))
    )]
    #[epilogue_macros(try_send, flush)]
    async fn complex_post_handler_with_ref_self(&self, ctx: &mut Context) {}
}

impl InjectComplexPost {
    #[post_method]
    async fn test_with_bool_param(_a: bool, ctx: &mut Context) {}

    #[get_method]
    async fn test_with_multiple_params(_a: bool, ctx: &mut Context, _b: i32) {}
}

#[route("/test/send")]
struct TestSend;

impl ServerHook for TestSend {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test send operation")
    )]
    #[epilogue_macros(send)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/send_body")]
struct TestSendBody;

impl ServerHook for TestSendBody {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test send body operation")
    )]
    #[epilogue_macros(send_body)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/send_body_with_data")]
struct TestSendBodyWithData;

impl ServerHook for TestSendBodyWithData {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN)
    )]
    #[epilogue_macros(send_body_with_data("Custom data from send_body_with_data"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/try_send")]
struct TestTrySend;

impl ServerHook for TestTrySend {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try send operation")
    )]
    #[epilogue_macros(try_send)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/try_send_body")]
struct TestTrySendBody;

impl ServerHook for TestTrySendBody {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try send body operation")
    )]
    #[epilogue_macros(try_send_body)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/try_send_body_with_data")]
struct TestTrySendBodyWithData;

impl ServerHook for TestTrySendBodyWithData {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN)
    )]
    #[epilogue_macros(try_send_body_with_data("Custom data from try_send_body_with_data"))]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/try_flush")]
struct TestTryFlush;

impl ServerHook for TestTryFlush {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try flush operation")
    )]
    #[epilogue_macros(try_flush)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/aborted")]
struct TestAborted;

impl ServerHook for TestAborted {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test aborted operation")
    )]
    #[epilogue_macros(aborted)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/closed")]
struct TestClosed;

impl ServerHook for TestClosed {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test closed operation")
    )]
    #[epilogue_macros(closed)]
    async fn handle(self, ctx: &mut Context) {}
}

#[route("/test/flush")]
struct TestFlush;

impl ServerHook for TestFlush {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test flush operation")
    )]
    #[epilogue_macros(flush)]
    async fn handle(self, ctx: &mut Context) {}
}

#[response_body("standalone response body")]
async fn standalone_response_body_handler(ctx: &mut Context) {}

#[prologue_macros(get_method, response_body("standalone get handler"))]
async fn standalone_get_handler(ctx: &mut Context) {}

#[epilogue_macros(try_send, flush)]
async fn standalone_send_and_flush_handler(ctx: &mut Context) {}

#[request_body(_raw_body)]
async fn standalone_request_body_extractor(ctx: &mut Context) {}

#[methods(get, post)]
async fn standalone_multiple_methods_handler(ctx: &mut Context) {}

#[http_from_stream]
async fn standalone_http_stream_handler(ctx: &mut Context) {}

#[ws_from_stream]
async fn standalone_websocket_stream_handler(ctx: &mut Context) {}

#[aborted]
async fn standalone_aborted_handler(ctx: &mut Context) {}

#[closed]
async fn standalone_closed_handler(ctx: &mut Context) {}

#[flush]
async fn standalone_flush_handler(ctx: &mut Context) {}

#[try_flush]
async fn standalone_try_flush_handler(ctx: &mut Context) {}

#[prologue_macros(
    get_method,
    http_version,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("standalone complex handler")
)]
#[epilogue_macros(try_send, flush)]
async fn standalone_complex_get_handler(ctx: &mut Context) {}

#[request_body(body1, body2, body3)]
async fn test_multi_request_body(ctx: &mut Context) {
    println!("body1: {:?}, body2: {:?}, body3: {:?}", body1, body2, body3);
}

#[route("/test_multi_request_body_json")]
#[derive(Debug, serde::Deserialize)]
struct User {
    name: String,
}

impl ServerHook for User {
    async fn new(_ctx: &mut Context) -> Self {
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
    async fn handle(self, ctx: &mut Context) {}
}

#[attribute("key1" => attr1: String, "key2" => attr2: i32)]
async fn test_multi_attribute(ctx: &mut Context) {
    println!("attr1: {:?}, attr2: {:?}", attr1, attr2);
}

#[attributes(attrs1, attrs2)]
async fn test_multi_attributes(ctx: &mut Context) {
    println!("attrs1: {:?}, attrs2: {:?}", attrs1, attrs2);
}

#[route_params(params1, params2)]
async fn test_multi_route_params(ctx: &mut Context) {
    println!("params1: {:?}, params2: {:?}", params1, params2);
}

#[request_querys(querys1, querys2)]
async fn test_multi_request_querys(ctx: &mut Context) {
    println!("querys1: {:?}, querys2: {:?}", querys1, querys2);
}

#[request_headers(headers1, headers2)]
async fn test_multi_request_headers(ctx: &mut Context) {
    println!("headers1: {:?}, headers2: {:?}", headers1, headers2);
}

#[request_cookies(cookies1, cookies2)]
async fn test_multi_request_cookies(ctx: &mut Context) {
    println!("cookies1: {:?}, cookies2: {:?}", cookies1, cookies2);
}

#[request_version(version1, version2)]
async fn test_multi_request_version(ctx: &mut Context) {
    println!("version1: {:?}, version2: {:?}", version1, version2);
}

#[request_path(path1, path2)]
async fn test_multi_request_path(ctx: &mut Context) {
    println!("path1: {:?}, path2: {:?}", path1, path2);
}

#[host("localhost", "127.0.0.1")]
async fn test_multi_host(ctx: &mut Context) {
    println!("Host check passed");
}

#[reject_host("localhost", "127.0.0.1")]
async fn test_multi_reject_host(ctx: &mut Context) {
    println!("Reject host check passed");
}

#[referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_referer(ctx: &mut Context) {
    println!("Referer check passed");
}

#[reject_referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_reject_referer(ctx: &mut Context) {
    println!("Reject referer check passed");
}

#[hyperlane(server1: Server, server2: Server)]
async fn test_multi_hyperlane() {
    println!("server1 and server2 initialized");
}

#[response_status_code(200)]
async fn standalone_response_status_code_handler(_ctx: &mut Context) {}

#[response_reason_phrase("Custom Reason")]
async fn standalone_response_reason_phrase_handler(_ctx: &mut Context) {}

#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
async fn standalone_response_header_handler(_ctx: &mut Context) {}

#[response_header("X-Custom-Header", "custom-value")]
async fn standalone_response_header_with_comma_handler(_ctx: &mut Context) {}

#[response_version(HttpVersion::Http1_1)]
async fn standalone_response_version_handler(_ctx: &mut Context) {}

#[connect_method]
async fn standalone_connect_handler(_ctx: &mut Context) {}

#[delete_method]
async fn standalone_delete_handler(_ctx: &mut Context) {}

#[head_method]
async fn standalone_head_handler(_ctx: &mut Context) {}

#[options_method]
async fn standalone_options_handler(_ctx: &mut Context) {}

#[patch_method]
async fn standalone_patch_handler(_ctx: &mut Context) {}

#[put_method]
async fn standalone_put_handler(_ctx: &mut Context) {}

#[trace_method]
async fn standalone_trace_handler(_ctx: &mut Context) {}

#[get_method]
async fn standalone_get_handler_with_param(_a: bool, ctx: &mut Context) {}

#[unknown_method]
async fn standalone_unknown_method_handler(_ctx: &mut Context) {}

#[methods(get, post, put)]
async fn standalone_methods_multiple_handler(_ctx: &mut Context) {}

#[http0_9_version]
async fn standalone_http0_9_version_handler(_ctx: &mut Context) {}

#[http1_0_version]
async fn standalone_http1_0_version_handler(_ctx: &mut Context) {}

#[http1_1_version]
async fn standalone_http1_1_handler(_ctx: &mut Context) {}

#[http2_version]
async fn standalone_http2_version_handler(_ctx: &mut Context) {}

#[http3_version]
async fn standalone_http3_version_handler(_ctx: &mut Context) {}

#[http1_1_or_higher_version]
async fn standalone_http1_1_or_higher_version_handler(_ctx: &mut Context) {}

#[unknown_version]
async fn standalone_unknown_version_handler(_ctx: &mut Context) {}

#[h2c_upgrade_type]
async fn standalone_h2c_upgrade_type_handler(_ctx: &mut Context) {}

#[tls_upgrade_type]
async fn standalone_tls_upgrade_type_handler(_ctx: &mut Context) {}

#[ws_upgrade_type]
async fn standalone_ws_handler(ctx: &mut Context) {}

#[unknown_upgrade_type]
async fn standalone_unknown_upgrade_type_handler(_ctx: &mut Context) {}

#[filter(_ctx.get_request().get_method().is_get())]
async fn standalone_filter_handler(_ctx: &mut Context) {}

#[reject(_ctx.get_request().get_method().is_post())]
async fn standalone_reject_handler(_ctx: &mut Context) {}

#[reject_host("example.com")]
async fn standalone_reject_host_handler(_ctx: &mut Context) {}

#[referer("https://example.com")]
async fn standalone_referer_handler(_ctx: &mut Context) {}

#[reject_referer("https://malicious.com")]
async fn standalone_reject_referer_handler(_ctx: &mut Context) {}

#[request_query("param" => _value)]
async fn standalone_request_query_handler(_ctx: &mut Context) {}

#[request_query_option("optional_param" => _optional_value)]
async fn standalone_request_query_option_handler(_ctx: &mut Context) {}

#[request_header(HOST => _host_value)]
async fn standalone_request_header_handler(_ctx: &mut Context) {}

#[request_header_option(USER_AGENT => _user_agent)]
async fn standalone_request_header_option_handler(_ctx: &mut Context) {}

#[request_querys(_querys)]
async fn standalone_request_querys_handler(_ctx: &mut Context) {}

#[request_headers(_headers)]
async fn standalone_request_headers_handler(_ctx: &mut Context) {}

#[request_cookies(_cookies)]
async fn standalone_request_cookies_handler(_ctx: &mut Context) {}

#[request_cookie("session" => _session_cookie)]
async fn standalone_request_cookie_handler(_ctx: &mut Context) {}

#[request_cookie_option("optional_cookie" => _optional_cookie)]
async fn standalone_request_cookie_option_handler(_ctx: &mut Context) {}

#[request_version(_version)]
async fn standalone_request_version_handler(_ctx: &mut Context) {}

#[request_path(_path)]
async fn standalone_request_path_handler(_ctx: &mut Context) {}

#[attribute("key" => _attr_value: String)]
async fn standalone_attribute_handler(_ctx: &mut Context) {}

#[attribute_option("optional_key" => _optional_attr: String)]
async fn standalone_attribute_option_handler(_ctx: &mut Context) {}

#[attributes(_attrs)]
async fn standalone_attributes_handler(_ctx: &mut Context) {}

#[route_params(_params)]
async fn standalone_route_params_handler(_ctx: &mut Context) {}

#[route_param("param" => _param_value)]
async fn standalone_route_param_handler(_ctx: &mut Context) {}

#[route_param_option("optional_param" => _optional_param_value)]
async fn standalone_route_param_option_handler(_ctx: &mut Context) {}

#[request_body_json(_user: TestData)]
async fn standalone_request_body_json_handler(_ctx: &mut Context) {}

#[request_body_json_result(_user_result: TestData)]
async fn standalone_request_body_json_result_handler(_ctx: &mut Context) {}

#[http_from_stream]
async fn standalone_http_from_stream_with_config_handler(_ctx: &mut Context) {}

#[ws_from_stream]
async fn standalone_ws_from_stream_with_config_handler(_ctx: &mut Context) {}

#[http_from_stream(_request)]
async fn standalone_http_from_stream_with_request_handler(_ctx: &mut Context) {}

#[ws_from_stream(_request)]
async fn standalone_ws_from_stream_with_request_handler(_ctx: &mut Context) {}

#[http_from_stream(_request)]
async fn standalone_http_from_stream_full_handler(_ctx: &mut Context) {}

#[ws_from_stream(_request)]
async fn standalone_ws_from_stream_full_handler(_ctx: &mut Context) {}

#[send]
async fn standalone_send_handler_2(_ctx: &mut Context) {}

#[send_body]
async fn standalone_send_body_handler_2(_ctx: &mut Context) {}

#[send_body_with_data("Custom send body data")]
async fn standalone_send_body_with_data_handler_2(_ctx: &mut Context) {}

#[try_send]
async fn standalone_try_send_handler_2(_ctx: &mut Context) {}

#[try_send_body]
async fn standalone_try_send_body_handler_2(_ctx: &mut Context) {}

#[try_send_body_with_data("Custom try send body data")]
async fn standalone_try_send_body_with_data_handler_2(_ctx: &mut Context) {}

#[flush]
async fn standalone_flush_handler_2(_ctx: &mut Context) {}

#[try_flush]
async fn standalone_try_flush_handler_2(_ctx: &mut Context) {}

#[aborted]
async fn standalone_aborted_handler_2(_ctx: &mut Context) {}

#[closed]
async fn standalone_closed_handler_2(_ctx: &mut Context) {}

#[clear_response_headers]
async fn standalone_clear_response_headers_handler(_ctx: &mut Context) {}

#[prologue_macros(
    get_method,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("prologue macros test")
)]
async fn standalone_prologue_macros_complex_handler(_ctx: &mut Context) {}

#[epilogue_macros(
    response_status_code(201),
    response_header(CONTENT_TYPE => APPLICATION_JSON),
    response_body("epilogue macros test"),
    try_send,
    flush
)]
async fn standalone_epilogue_macros_complex_handler(_ctx: &mut Context) {}

#[prologue_hooks(prologue_hooks_fn)]
async fn standalone_prologue_hooks_handler(_ctx: &mut Context) {}

#[epilogue_hooks(epilogue_hooks_fn)]
async fn standalone_epilogue_hooks_handler(_ctx: &mut Context) {}

#[route("/hooks_expression")]
struct HooksExpression;

impl ServerHook for HooksExpression {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[get_method]
    #[prologue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
    #[epilogue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
    #[response_body("hooks expression test")]
    async fn handle(self, ctx: &mut Context) {}
}

impl HooksExpression {
    async fn new_hook(_ctx: &mut Context) {}

    async fn method_hook(_ctx: &mut Context) {}
}

#[route("/server_config")]
struct MultiServerConfig;

impl ServerHook for MultiServerConfig {
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[get_method]
    #[response_body("multi server config test")]
    async fn handle(self, ctx: &mut Context) {}
}

impl MultiServerConfig {
    #[hyperlane(server_config: ServerConfig)]
    async fn server_config_1() -> ServerConfig {
        server_config
    }

    #[hyperlane(server_config: ServerConfig)]
    async fn server_config_2(self) -> ServerConfig {
        server_config
    }

    #[hyperlane(server_config: ServerConfig)]
    async fn server_config_3(&self) -> ServerConfig {
        server_config
    }
}

#[hyperlane(server: Server)]
#[hyperlane(config: ServerConfig)]
#[tokio::main]
async fn main() {
    config.set_nodelay(Some(false));
    server.server_config(config);
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
