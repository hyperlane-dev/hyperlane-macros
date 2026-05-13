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
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        try_get_task_panic_data(try_get_task_panic_data),
        task_panic_data(task_panic_data)
    )]
    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(500),
        response_body(format!("{task_panic_data} {try_get_task_panic_data:?}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[request_error]
#[request_error(1)]
#[request_error("2")]
struct RequestErrorHook;

impl ServerHook for RequestErrorHook {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        try_get_request_error_data(try_get_request_error_data),
        request_error_data(request_error_data)
    )]
    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(500),
        response_body(format!("{request_error_data} {try_get_request_error_data:?}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[request_middleware]
struct RequestMiddleware;

impl ServerHook for RequestMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
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
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[request_middleware(1)]
struct UpgradeHook;

impl ServerHook for UpgradeHook {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        ws_upgrade_type,
        response_body(Vec::new()),
        response_status_code(101),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => &WebSocketFrame::generate_accept_key(ctx.get_request().get_header_back(SEC_WEBSOCKET_KEY))),
        response_header(STEP => "upgrade_hook"),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[request_middleware(2)]
struct ConnectedHook;

impl ServerHook for ConnectedHook {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    #[response_header(SERVER => HYPERLANE)]
    #[response_version(HttpVersion::Http1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(STEP => "connected_hook")]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[response_middleware]
struct ResponseMiddleware1;

impl ServerHook for ResponseMiddleware1 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_header(STEP => "response_middleware_1")]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[response_middleware(2)]
struct ResponseMiddleware2;

impl ServerHook for ResponseMiddleware2 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request().get_upgrade_type().is_ws()),
        response_header(STEP => "response_middleware_2")
    )]
    #[epilogue_macros(try_send, flush)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[response_middleware("3")]
struct ResponseMiddleware3;

impl ServerHook for ResponseMiddleware3 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        ws_upgrade_type,
        response_header(STEP => "response_middleware_3")
    )]
    #[epilogue_macros(try_send, flush)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

struct PrologueHooks;

impl ServerHook for PrologueHooks {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[get_method]
    #[http_version]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

struct EpilogueHooks;

impl ServerHook for EpilogueHooks {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

async fn prologue_hooks_fn(stream: &mut Stream, ctx: &mut Context) -> Status {
    let hook: PrologueHooks = PrologueHooks::new(stream, ctx).await;
    hook.handle(stream, ctx).await
}

async fn epilogue_hooks_fn(stream: &mut Stream, ctx: &mut Context) -> Status {
    let hook: EpilogueHooks = EpilogueHooks::new(stream, ctx).await;
    hook.handle(stream, ctx).await
}

#[route("/response")]
struct Response;

impl ServerHook for Response {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&RESPONSE_DATA)]
    #[response_reason_phrase(CUSTOM_REASON)]
    #[response_status_code(CUSTOM_STATUS_CODE)]
    #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/connect")]
struct ConnectMethod;

impl ServerHook for ConnectMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(connect_method, response_body("connect"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/delete")]
struct DeleteMethod;

impl ServerHook for DeleteMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(delete_method, response_body("delete"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/head")]
struct HeadMethod;

impl ServerHook for HeadMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(head_method, response_body("head"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/options")]
struct OptionsMethod;

impl ServerHook for OptionsMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(options_method, response_body("options"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/patch")]
struct PatchMethod;

impl ServerHook for PatchMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(patch_method, response_body("patch"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/put")]
struct PutMethod;

impl ServerHook for PutMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(put_method, response_body("put"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/trace")]
struct TraceMethod;

impl ServerHook for TraceMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(trace_method, response_body("trace"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/get_post_method")]
struct GetPostMethod;

impl ServerHook for GetPostMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
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
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/get_method")]
struct GetMethod;

impl ServerHook for GetMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(ws_upgrade_type, get_method, response_body("get_method"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/post_method")]
struct PostMethod;

impl ServerHook for PostMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, response_body("post_method"), try_send)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/unknown_method")]
struct UnknownMethod;

impl ServerHook for UnknownMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(unknown_method, response_body("unknown_method"), try_send)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http0_9_version")]
struct Http09Version;

impl ServerHook for Http09Version {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http0_9_version, response_body("http0_9_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http1_0_version")]
struct Http10Version;

impl ServerHook for Http10Version {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http1_0_version, response_body("http1_0_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http1_1_version")]
struct Http11Version;

impl ServerHook for Http11Version {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1_version, response_body("http1_1_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http2_version")]
struct Http2Version;

impl ServerHook for Http2Version {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http2_version, response_body("http2_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http3_version")]
struct Http3Version;

impl ServerHook for Http3Version {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http3_version, response_body("http3_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http1_1_or_higher_version")]
struct Http11OrHigher;

impl ServerHook for Http11OrHigher {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http1_1_or_higher_version, response_body("http1_1_or_higher_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/http_version")]
struct HttpAllVersion;

impl ServerHook for HttpAllVersion {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(http_version, response_body("http_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/unknown_version")]
struct UnknownVersion;

impl ServerHook for UnknownVersion {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(unknown_version, response_body("unknown_version"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/ws_upgrade_type")]
struct WsUpgradeType;

impl ServerHook for WsUpgradeType {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/h2c_upgrade_type")]
struct H2cUpgradeType;

impl ServerHook for H2cUpgradeType {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(h2c_upgrade_type, response_body("h2c_upgrade_type"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/tls_upgrade_type")]
struct Tls;

impl ServerHook for Tls {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(tls_upgrade_type, response_body("tls_upgrade_type"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/unknown_upgrade_type")]
struct UnknownUpgradeType;

impl ServerHook for UnknownUpgradeType {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(unknown_upgrade_type, response_body("unknown_upgrade_type"))]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/ws1")]
struct Websocket1;

impl ServerHook for Websocket1 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[try_get_websocket_request(body)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        stream.send_list(body_list).await;
    }
}

#[route("/ws2")]
struct Websocket2;

impl ServerHook for Websocket2 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[try_get_websocket_request(request)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&request);
        stream.send_list(body_list).await;
    }
}

#[route("/ws3")]
struct Websocket3;

impl ServerHook for Websocket3 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[try_get_websocket_request(request)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&request);
        stream.send_list(body_list).await;
    }
}

#[route("/ws4")]
struct Websocket4;

impl ServerHook for Websocket4 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[try_get_websocket_request(request)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&request);
        stream.send_list(body_list).await;
    }
}

#[route("/ws5")]
struct Websocket5;

impl ServerHook for Websocket5 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[ws_upgrade_type]
    #[try_get_websocket_request(body)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        stream.send_list(body_list).await;
    }
}

#[route("/hook")]
struct Hook;

impl ServerHook for Hook {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_hooks(prologue_hooks_fn)]
    #[epilogue_hooks(epilogue_hooks_fn)]
    #[response_body("Testing hook macro")]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/attributes")]
struct Attributes;

impl ServerHook for Attributes {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request attributes: {request_attributes:?}"))]
    #[attributes(request_attributes)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/route_params/:test")]
struct RouteParams;

impl ServerHook for RouteParams {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request route params: {request_route_params:?}"))]
    #[route_params(request_route_params)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/try_get_route_param/:test")]
struct RouteParamOption;

impl ServerHook for RouteParamOption {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_try_get_route_param1:?} {request_try_get_route_param2:?} {request_try_get_route_param3:?}"))]
    #[try_get_route_param("test1" => request_try_get_route_param1)]
    #[try_get_route_param("test2" => request_try_get_route_param2, "test3" => request_try_get_route_param3)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/route_param/:test")]
struct RouteParam;

impl ServerHook for RouteParam {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("route param: {request_route_param1} {request_route_param2} {request_route_param3}"))]
    #[route_param("test1" => request_route_param1)]
    #[route_param("test2" => request_route_param2, "test3" => request_route_param3)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/host")]
struct Host;

impl ServerHook for Host {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[host("localhost")]
    #[epilogue_macros(response_body("host string literal: localhost"), send)]
    #[prologue_macros(response_body("host string literal: localhost"), send)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/try_get_request_query")]
struct RequestQueryOption;

impl ServerHook for RequestQueryOption {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        try_get_request_query("test" => try_get_request_query),
        response_body(&format!("request query: {try_get_request_query:?}")),
        send
    )]
    #[prologue_macros(
        try_get_request_query("test" => try_get_request_query),
        response_body(&format!("request query: {try_get_request_query:?}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_query")]
struct RequestQuery;

impl ServerHook for RequestQuery {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_query("test" => request_query),
        response_body(&format!("request query: {request_query}")),
        send
    )]
    #[prologue_macros(
        request_query("test" => request_query),
        response_body(&format!("request query: {request_query}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/try_get_request_header")]
struct RequestHeaderOption;

impl ServerHook for RequestHeaderOption {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        try_get_request_header(HOST => try_get_request_header),
        response_body(&format!("request header: {try_get_request_header:?}")),
        send
    )]
    #[prologue_macros(
        try_get_request_header(HOST => try_get_request_header),
        response_body(&format!("request header: {try_get_request_header:?}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_header")]
struct RequestHeader;

impl ServerHook for RequestHeader {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_header(HOST => request_header),
        response_body(&format!("request header: {request_header}")),
        send
    )]
    #[prologue_macros(
        request_header(HOST => request_header),
        response_body(&format!("request header: {request_header}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_querys")]
struct RequestQuerys;

impl ServerHook for RequestQuerys {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send
    )]
    #[prologue_macros(
        request_querys(request_querys),
        response_body(&format!("request querys: {request_querys:?}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_headers")]
struct RequestHeaders;

impl ServerHook for RequestHeaders {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send
    )]
    #[prologue_macros(
        request_headers(request_headers),
        response_body(&format!("request headers: {request_headers:?}")),
        send
    )]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_body")]
struct RequestBodyRoute;

impl ServerHook for RequestBodyRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("raw body: {raw_body:?}"))]
    #[request_body(raw_body)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/reject_host")]
struct RejectHost;

impl ServerHook for RejectHost {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_host("filter.localhost"),
        response_body("host filter string literal")
    )]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/try_get_attribute")]
struct AttributeOption;

impl ServerHook for AttributeOption {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_try_get_attribute:?}"))]
    #[try_get_attribute(TEST_ATTRIBUTE_KEY => request_try_get_attribute: TestData)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/attribute")]
struct Attribute;

impl ServerHook for Attribute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request attribute: {request_attribute:?}"))]
    #[attribute(TEST_ATTRIBUTE_KEY => request_attribute: TestData)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_body_json_result")]
struct RequestBodyJsonResult;

impl ServerHook for RequestBodyJsonResult {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json_result(request_data_result: TestData)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_body_json")]
struct RequestBodyJson;

impl ServerHook for RequestBodyJson {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("request data: {request_data_result:?}"))]
    #[request_body_json(request_data_result: TestData)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/referer")]
struct Referer;

impl ServerHook for Referer {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        referer("http://localhost"),
        response_body("referer string literal: http://localhost")
    )]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/reject_referer")]
struct RejectReferer;

impl ServerHook for RejectReferer {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject_referer("http://localhost"),
        response_body("referer filter string literal")
    )]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/cookies")]
struct Cookies;

impl ServerHook for Cookies {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("All cookies: {cookie_value:?}"))]
    #[request_cookies(cookie_value)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/try_get_request_cookie")]
struct CookieOption;

impl ServerHook for CookieOption {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
    #[try_get_request_cookie("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_cookie")]
struct Cookie;

impl ServerHook for Cookie {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("Session cookie: {session_cookie1}, {session_cookie2}"))]
    #[request_cookie("test1" => session_cookie1, "test2" => session_cookie2)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_version")]
struct RequestVersionTest;

impl ServerHook for RequestVersionTest {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("HTTP Version: {http_version}"))]
    #[request_version(http_version)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/request_path")]
struct RequestPathTest;

impl ServerHook for RequestPathTest {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body(&format!("Request Path: {request_path}"))]
    #[request_path(request_path)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/response_header")]
struct ResponseHeaderTest;

impl ServerHook for ResponseHeaderTest {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_body("Testing header set and replace operations")]
    #[response_header("X-Add-Header", "add-value")]
    #[response_header("X-Set-Header" => "set-value")]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/literals")]
struct Literals;

impl ServerHook for Literals {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[response_status_code(201)]
    #[response_header(CONTENT_TYPE => APPLICATION_JSON)]
    #[response_body("{\"message\": \"Resource created\"}")]
    #[response_reason_phrase(HttpStatus::Created.to_string())]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/inject/response_body")]
struct InjectResponseBody;

impl ServerHook for InjectResponseBody {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.response_body_with_ref_self(stream, ctx).await
    }
}

impl InjectResponseBody {
    #[response_body("response body with ref self")]
    async fn response_body_with_ref_self(&self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/inject/post_method")]
struct InjectPostMethod;

impl ServerHook for InjectPostMethod {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.post_method_with_ref_self(stream, ctx).await
    }
}

impl InjectPostMethod {
    #[prologue_macros(post_method, response_body("post method with ref self"))]
    async fn post_method_with_ref_self(&self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/inject/send_flush")]
struct InjectSendFlush;

impl ServerHook for InjectSendFlush {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.send_and_flush_with_ref_self(stream, ctx).await
    }
}

impl InjectSendFlush {
    #[epilogue_macros(try_send, flush)]
    async fn send_and_flush_with_ref_self(&self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/inject/request_body")]
struct InjectRequestBody;

impl ServerHook for InjectRequestBody {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.extract_request_body_with_ref_self(stream, ctx).await
    }
}

impl InjectRequestBody {
    #[request_body(_raw_body)]
    async fn extract_request_body_with_ref_self(
        &self,
        _stream: &mut Stream,
        ctx: &mut Context,
    ) -> Status {
        Status::Continue
    }
}

#[route("/inject/multiple_methods")]
struct InjectMultipleMethods;

impl ServerHook for InjectMultipleMethods {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.multiple_methods_with_ref_self(stream, ctx).await
    }
}

impl InjectMultipleMethods {
    #[methods(get, post)]
    async fn multiple_methods_with_ref_self(
        &self,
        _stream: &mut Stream,
        ctx: &mut Context,
    ) -> Status {
        Status::Continue
    }

    #[unknown_method]
    async fn unknown_method_with_ref_self(
        &self,
        _stream: &mut Stream,
        ctx: &mut Context,
    ) -> Status {
        Status::Continue
    }
}

#[route("/inject/http_")]
struct InjectHttpStream;

impl ServerHook for InjectHttpStream {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.http_handler_with_ref_self(stream, ctx).await
    }
}

impl InjectHttpStream {
    #[try_get_http_request(_request)]
    async fn http_handler_with_ref_self(&self, stream: &mut Stream, ctx: &mut Context) -> Status {}
}

#[route("/inject/ws_")]
struct InjectWsStream;

impl ServerHook for InjectWsStream {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.websocket_handler_with_ref_self(stream, ctx).await
    }
}

impl InjectWsStream {
    #[try_get_websocket_request(_request)]
    async fn websocket_handler_with_ref_self(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
    ) -> Status {
    }
}

#[route("/inject/complex_post")]
struct InjectComplexPost;

impl ServerHook for InjectComplexPost {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        self.complex_post_handler_with_ref_self(stream, ctx).await
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
    async fn complex_post_handler_with_ref_self(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
    ) -> Status {
        Status::Continue
    }
}

impl InjectComplexPost {
    #[post_method]
    async fn test_with_bool_param(_a: bool, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }

    #[get_method]
    async fn test_with_multiple_params(
        _a: bool,
        _stream: &mut Stream,
        ctx: &mut Context,
        _b: i32,
    ) -> Status {
        Status::Continue
    }
}

#[route("/test/send")]
struct TestSend;

impl ServerHook for TestSend {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test send operation")
    )]
    #[epilogue_macros(send)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/test/try_send")]
struct TestTrySend;

impl ServerHook for TestTrySend {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try send operation")
    )]
    #[epilogue_macros(try_send)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/test/try_flush")]
struct TestTryFlush;

impl ServerHook for TestTryFlush {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test try flush operation")
    )]
    #[epilogue_macros(try_flush)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/test/closed")]
struct TestClosed;

impl ServerHook for TestClosed {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test closed operation")
    )]
    #[epilogue_macros(closed)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/test/flush")]
struct TestFlush;

impl ServerHook for TestFlush {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_PLAIN),
        response_body("Test flush operation")
    )]
    #[epilogue_macros(flush)]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[response_body("standalone response body")]
async fn standalone_response_body_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[prologue_macros(get_method, response_body("standalone get handler"))]
async fn standalone_get_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[epilogue_macros(try_send, flush)]
async fn standalone_send_and_flush_handler(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_body(_raw_body)]
async fn standalone_request_body_extractor(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[methods(get, post)]
async fn standalone_multiple_methods_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_get_http_request]
async fn standalone_http_handler(stream: &mut Stream, ctx: &mut Context) -> Status {}

#[try_get_websocket_request]
async fn standalone_websocket_handler(stream: &mut Stream, ctx: &mut Context) -> Status {}

#[closed]
async fn standalone_closed_handler(stream: &mut Stream, _ctx: &mut Context) -> Status {
    Status::Continue
}

#[flush]
async fn standalone_flush_handler(stream: &mut Stream, _ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_flush]
async fn standalone_try_flush_handler(stream: &mut Stream, _ctx: &mut Context) -> Status {
    Status::Continue
}

#[prologue_macros(
    get_method,
    http_version,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("standalone complex handler")
)]
#[epilogue_macros(try_send, flush)]
async fn standalone_complex_get_handler(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_body(body1, body2, body3)]
async fn test_multi_request_body(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("body1: {:?}, body2: {:?}, body3: {:?}", body1, body2, body3);
    Status::Continue
}

#[route("/test_multi_request_body_json")]
#[derive(Debug, serde::Deserialize)]
struct User {
    name: String,
}

impl ServerHook for User {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
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
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[attribute("key1" => attr1: String, "key2" => attr2: i32)]
async fn test_multi_attribute(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("attr1: {:?}, attr2: {:?}", attr1, attr2);
    Status::Continue
}

#[attributes(attrs1, attrs2)]
async fn test_multi_attributes(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("attrs1: {:?}, attrs2: {:?}", attrs1, attrs2);
    Status::Continue
}

#[route_params(params1, params2)]
async fn test_multi_route_params(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("params1: {:?}, params2: {:?}", params1, params2);
    Status::Continue
}

#[request_querys(querys1, querys2)]
async fn test_multi_request_querys(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("querys1: {:?}, querys2: {:?}", querys1, querys2);
    Status::Continue
}

#[request_headers(headers1, headers2)]
async fn test_multi_request_headers(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("headers1: {:?}, headers2: {:?}", headers1, headers2);
    Status::Continue
}

#[request_cookies(cookies1, cookies2)]
async fn test_multi_request_cookies(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("cookies1: {:?}, cookies2: {:?}", cookies1, cookies2);
    Status::Continue
}

#[request_version(version1, version2)]
async fn test_multi_request_version(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("version1: {:?}, version2: {:?}", version1, version2);
    Status::Continue
}

#[request_path(path1, path2)]
async fn test_multi_request_path(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("path1: {:?}, path2: {:?}", path1, path2);
    Status::Continue
}

#[host("localhost", "127.0.0.1")]
async fn test_multi_host(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("Host check passed");
    Status::Continue
}

#[reject_host("localhost", "127.0.0.1")]
async fn test_multi_reject_host(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("Reject host check passed");
    Status::Continue
}

#[referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_referer(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("Referer check passed");
    Status::Continue
}

#[reject_referer("http://localhost", "http://127.0.0.1")]
async fn test_multi_reject_referer(_stream: &mut Stream, ctx: &mut Context) -> Status {
    println!("Reject referer check passed");
    Status::Continue
}

#[hyperlane(server1: Server, server2: Server)]
async fn test_multi_hyperlane() {
    println!("server1 and server2 initialized");
}

#[response_status_code(200)]
async fn standalone_response_status_code_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[response_reason_phrase("Custom Reason")]
async fn standalone_response_reason_phrase_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
async fn standalone_response_header_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[response_header("X-Custom-Header", "custom-value")]
async fn standalone_response_header_with_comma_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[response_version(HttpVersion::Http1_1)]
async fn standalone_response_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[connect_method]
async fn standalone_connect_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[delete_method]
async fn standalone_delete_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[head_method]
async fn standalone_head_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[options_method]
async fn standalone_options_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[patch_method]
async fn standalone_patch_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[put_method]
async fn standalone_put_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[trace_method]
async fn standalone_trace_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[get_method]
async fn standalone_get_handler_with_param(
    _a: bool,
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[unknown_method]
async fn standalone_unknown_method_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[methods(get, post, put)]
async fn standalone_methods_multiple_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[http0_9_version]
async fn standalone_http0_9_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[http1_0_version]
async fn standalone_http1_0_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[http1_1_version]
async fn standalone_http1_1_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[http2_version]
async fn standalone_http2_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[http3_version]
async fn standalone_http3_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[http1_1_or_higher_version]
async fn standalone_http1_1_or_higher_version_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[unknown_version]
async fn standalone_unknown_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[h2c_upgrade_type]
async fn standalone_h2c_upgrade_type_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[tls_upgrade_type]
async fn standalone_tls_upgrade_type_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[ws_upgrade_type]
async fn standalone_ws_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[unknown_upgrade_type]
async fn standalone_unknown_upgrade_type_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[filter(ctx.get_request().get_method().is_get())]
async fn standalone_filter_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[reject(ctx.get_request().get_method().is_post())]
async fn standalone_reject_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[reject_host("example.com")]
async fn standalone_reject_host_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[referer("https://example.com")]
async fn standalone_referer_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[reject_referer("https://malicious.com")]
async fn standalone_reject_referer_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_query("param" => _value)]
async fn standalone_request_query_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_get_request_query("optional_param" => _optional_value)]
async fn standalone_try_get_request_query_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[request_header(HOST => _host_value)]
async fn standalone_request_header_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_get_request_header(USER_AGENT => _user_agent)]
async fn standalone_try_get_request_header_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[request_querys(_querys)]
async fn standalone_request_querys_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_headers(_headers)]
async fn standalone_request_headers_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_cookies(_cookies)]
async fn standalone_request_cookies_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_cookie("session" => _session_cookie)]
async fn standalone_request_cookie_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_get_request_cookie("optional_cookie" => _optional_cookie)]
async fn standalone_try_get_request_cookie_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[request_version(_version)]
async fn standalone_request_version_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_path(_path)]
async fn standalone_request_path_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[attribute("key" => _attr_value: String)]
async fn standalone_attribute_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_get_attribute("optional_key" => _optional_attr: String)]
async fn standalone_try_get_attribute_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[attributes(_attrs)]
async fn standalone_attributes_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[route_params(_params)]
async fn standalone_route_params_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[route_param("param" => _param_value)]
async fn standalone_route_param_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_get_route_param("optional_param" => _optional_param_value)]
async fn standalone_try_get_route_param_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_body_json(_user: TestData)]
async fn standalone_request_body_json_handler(_stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[request_body_json_result(_user_result: TestData)]
async fn standalone_request_body_json_result_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[try_get_http_request]
async fn standalone_try_get_http_request_with_config_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
}

#[try_get_websocket_request]
async fn standalone_try_get_websocket_request_with_config_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
}

#[try_get_http_request(_request)]
async fn standalone_try_get_http_request_with_request_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
}

#[try_get_websocket_request(_request)]
async fn standalone_try_get_websocket_request_with_request_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
}

#[try_get_http_request(_request)]
async fn standalone_try_get_http_request_full_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
}

#[try_get_websocket_request(_request)]
async fn standalone_try_get_websocket_request_full_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
}

#[send]
async fn standalone_send_handler_2(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[send(ctx.get_mut_response().build())]
async fn standalone_send_with_data_handler(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_send]
async fn standalone_try_send_handler_2(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_send(ctx.get_mut_response().build())]
async fn standalone_try_send_with_data_handler(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[flush]
async fn standalone_flush_handler_2(stream: &mut Stream, _ctx: &mut Context) -> Status {
    Status::Continue
}

#[try_flush]
async fn standalone_try_flush_handler_2(stream: &mut Stream, _ctx: &mut Context) -> Status {
    Status::Continue
}

#[closed]
async fn standalone_closed_handler_2(stream: &mut Stream, _ctx: &mut Context) -> Status {
    Status::Continue
}

#[clear_response_headers]
async fn standalone_clear_response_headers_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[prologue_macros(
    get_method,
    response_status_code(200),
    response_header(CONTENT_TYPE => TEXT_PLAIN),
    response_body("prologue macros test")
)]
async fn standalone_prologue_macros_complex_handler(
    _stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[epilogue_macros(
    response_status_code(201),
    response_header(CONTENT_TYPE => APPLICATION_JSON),
    response_body("epilogue macros test"),
    try_send,
    flush
)]
async fn standalone_epilogue_macros_complex_handler(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Status {
    Status::Continue
}

#[prologue_hooks(prologue_hooks_fn)]
async fn standalone_prologue_hooks_handler(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[epilogue_hooks(epilogue_hooks_fn)]
async fn standalone_epilogue_hooks_handler(stream: &mut Stream, ctx: &mut Context) -> Status {
    Status::Continue
}

#[route("/hooks_expression")]
struct HooksExpression;

impl ServerHook for HooksExpression {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[get_method]
    #[prologue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
    #[epilogue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
    #[response_body("hooks expression test")]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

impl HooksExpression {
    async fn new_hook(_stream: &mut Stream, _ctx: &mut Context) -> Status {
        Status::Continue
    }

    async fn method_hook(_stream: &mut Stream, _ctx: &mut Context) -> Status {
        Status::Continue
    }
}

#[route("/server_config")]
struct MultiServerConfig;

impl ServerHook for MultiServerConfig {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[get_method]
    #[response_body("multi server config test")]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
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
