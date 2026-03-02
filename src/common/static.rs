use crate::*;

/// Static array of all injectable macros.
///
/// This array contains all the macro handlers that can be injected using the `inject` macro.
pub(crate) static INJECTABLE_MACROS: &[InjectableMacro] = &[
    InjectableMacro {
        name: "aborted",
        handler: Handler::NoAttrPosition(aborted_macro),
    },
    InjectableMacro {
        name: "closed",
        handler: Handler::NoAttrPosition(closed_macro),
    },
    InjectableMacro {
        name: "filter",
        handler: Handler::WithAttrPosition(filter_macro),
    },
    InjectableMacro {
        name: "try_flush",
        handler: Handler::NoAttrPosition(try_flush_macro),
    },
    InjectableMacro {
        name: "flush",
        handler: Handler::NoAttrPosition(flush_macro),
    },
    InjectableMacro {
        name: "task_panic",
        handler: Handler::WithAttr(task_panic_macro),
    },
    InjectableMacro {
        name: "request_error",
        handler: Handler::WithAttr(request_error_macro),
    },
    InjectableMacro {
        name: "prologue_hooks",
        handler: Handler::WithAttrPosition(prologue_hooks_macro),
    },
    InjectableMacro {
        name: "epilogue_hooks",
        handler: Handler::WithAttrPosition(epilogue_hooks_macro),
    },
    InjectableMacro {
        name: "host",
        handler: Handler::WithAttrPosition(host_macro),
    },
    InjectableMacro {
        name: "reject_host",
        handler: Handler::WithAttrPosition(reject_host_macro),
    },
    InjectableMacro {
        name: "hyperlane",
        handler: Handler::WithAttr(hyperlane_macro),
    },
    InjectableMacro {
        name: "methods",
        handler: Handler::WithAttrPosition(methods_macro),
    },
    InjectableMacro {
        name: "get_method",
        handler: Handler::NoAttrPosition(get_method_handler),
    },
    InjectableMacro {
        name: "post_method",
        handler: Handler::NoAttrPosition(post_method_handler),
    },
    InjectableMacro {
        name: "put_method",
        handler: Handler::NoAttrPosition(put_method_handler),
    },
    InjectableMacro {
        name: "delete_method",
        handler: Handler::NoAttrPosition(delete_method_handler),
    },
    InjectableMacro {
        name: "patch_method",
        handler: Handler::NoAttrPosition(patch_method_handler),
    },
    InjectableMacro {
        name: "head_method",
        handler: Handler::NoAttrPosition(head_method_handler),
    },
    InjectableMacro {
        name: "options_method",
        handler: Handler::NoAttrPosition(options_method_handler),
    },
    InjectableMacro {
        name: "connect_method",
        handler: Handler::NoAttrPosition(connect_method_handler),
    },
    InjectableMacro {
        name: "trace_method",
        handler: Handler::NoAttrPosition(trace_method_handler),
    },
    InjectableMacro {
        name: "unknown_method",
        handler: Handler::NoAttrPosition(unknown_method_handler),
    },
    InjectableMacro {
        name: "referer",
        handler: Handler::WithAttrPosition(referer_macro),
    },
    InjectableMacro {
        name: "reject_referer",
        handler: Handler::WithAttrPosition(reject_referer_macro),
    },
    InjectableMacro {
        name: "reject",
        handler: Handler::WithAttrPosition(reject_macro),
    },
    InjectableMacro {
        name: "request_body",
        handler: Handler::WithAttrPosition(request_body_macro),
    },
    InjectableMacro {
        name: "request_body_json_result",
        handler: Handler::WithAttrPosition(request_body_json_result_macro),
    },
    InjectableMacro {
        name: "request_body_json",
        handler: Handler::WithAttrPosition(request_body_json_macro),
    },
    InjectableMacro {
        name: "attribute_option",
        handler: Handler::WithAttrPosition(attribute_option_macro),
    },
    InjectableMacro {
        name: "attribute",
        handler: Handler::WithAttrPosition(attribute_macro),
    },
    InjectableMacro {
        name: "attributes",
        handler: Handler::WithAttrPosition(attributes_macro),
    },
    InjectableMacro {
        name: "task_panic_data_option",
        handler: Handler::WithAttrPosition(task_panic_data_option_macro),
    },
    InjectableMacro {
        name: "task_panic_data",
        handler: Handler::WithAttrPosition(task_panic_data_macro),
    },
    InjectableMacro {
        name: "request_error_data_option",
        handler: Handler::WithAttrPosition(request_error_data_option_macro),
    },
    InjectableMacro {
        name: "request_error_data",
        handler: Handler::WithAttrPosition(request_error_data_macro),
    },
    InjectableMacro {
        name: "route_param_option",
        handler: Handler::WithAttrPosition(route_param_option_macro),
    },
    InjectableMacro {
        name: "route_param",
        handler: Handler::WithAttrPosition(route_param_macro),
    },
    InjectableMacro {
        name: "route_params",
        handler: Handler::WithAttrPosition(route_params_macro),
    },
    InjectableMacro {
        name: "request_query_option",
        handler: Handler::WithAttrPosition(request_query_option_macro),
    },
    InjectableMacro {
        name: "request_query",
        handler: Handler::WithAttrPosition(request_query_macro),
    },
    InjectableMacro {
        name: "request_querys",
        handler: Handler::WithAttrPosition(request_querys_macro),
    },
    InjectableMacro {
        name: "request_header_option",
        handler: Handler::WithAttrPosition(request_header_option_macro),
    },
    InjectableMacro {
        name: "request_header",
        handler: Handler::WithAttrPosition(request_header_macro),
    },
    InjectableMacro {
        name: "request_headers",
        handler: Handler::WithAttrPosition(request_headers_macro),
    },
    InjectableMacro {
        name: "request_cookie_option",
        handler: Handler::WithAttrPosition(request_cookie_option_macro),
    },
    InjectableMacro {
        name: "request_cookie",
        handler: Handler::WithAttrPosition(request_cookie_macro),
    },
    InjectableMacro {
        name: "request_cookies",
        handler: Handler::WithAttrPosition(request_cookies_macro),
    },
    InjectableMacro {
        name: "request_version",
        handler: Handler::WithAttrPosition(request_version_macro),
    },
    InjectableMacro {
        name: "request_path",
        handler: Handler::WithAttrPosition(request_path_macro),
    },
    InjectableMacro {
        name: "request_middleware",
        handler: Handler::WithAttr(request_middleware_macro),
    },
    InjectableMacro {
        name: "response_status_code",
        handler: Handler::WithAttrPosition(response_status_code_macro),
    },
    InjectableMacro {
        name: "response_reason_phrase",
        handler: Handler::WithAttrPosition(response_reason_phrase_macro),
    },
    InjectableMacro {
        name: "response_header",
        handler: Handler::WithAttrPosition(response_header_macro),
    },
    InjectableMacro {
        name: "response_body",
        handler: Handler::WithAttrPosition(response_body_macro),
    },
    InjectableMacro {
        name: "clear_response_headers",
        handler: Handler::NoAttrPosition(clear_response_headers_macro),
    },
    InjectableMacro {
        name: "response_version",
        handler: Handler::WithAttrPosition(response_version_macro),
    },
    InjectableMacro {
        name: "response_middleware",
        handler: Handler::WithAttr(response_middleware_macro),
    },
    InjectableMacro {
        name: "route",
        handler: Handler::WithAttr(route_macro),
    },
    InjectableMacro {
        name: "try_send",
        handler: Handler::NoAttrPosition(try_send_macro),
    },
    InjectableMacro {
        name: "send",
        handler: Handler::NoAttrPosition(send_macro),
    },
    InjectableMacro {
        name: "try_send_body",
        handler: Handler::NoAttrPosition(try_send_body_macro),
    },
    InjectableMacro {
        name: "send_body",
        handler: Handler::NoAttrPosition(send_body_macro),
    },
    InjectableMacro {
        name: "try_send_body_with_data",
        handler: Handler::WithAttrPosition(try_send_body_with_data_macro),
    },
    InjectableMacro {
        name: "send_body_with_data",
        handler: Handler::WithAttrPosition(send_body_with_data_macro),
    },
    InjectableMacro {
        name: "http_from_stream",
        handler: Handler::WithAttr(http_from_stream_macro),
    },
    InjectableMacro {
        name: "ws_from_stream",
        handler: Handler::WithAttr(ws_from_stream_macro),
    },
    InjectableMacro {
        name: "ws_upgrade_type",
        handler: Handler::NoAttrPosition(ws_upgrade_type_macro),
    },
    InjectableMacro {
        name: "h2c_upgrade_type",
        handler: Handler::NoAttrPosition(h2c_upgrade_type_macro),
    },
    InjectableMacro {
        name: "tls_upgrade_type",
        handler: Handler::NoAttrPosition(tls_upgrade_type_macro),
    },
    InjectableMacro {
        name: "unknown_upgrade_type",
        handler: Handler::NoAttrPosition(unknown_upgrade_type_macro),
    },
    InjectableMacro {
        name: "http0_9_version",
        handler: Handler::NoAttrPosition(http0_9_version_macro),
    },
    InjectableMacro {
        name: "http1_0_version",
        handler: Handler::NoAttrPosition(http1_0_version_macro),
    },
    InjectableMacro {
        name: "http1_1_version",
        handler: Handler::NoAttrPosition(http1_1_version_macro),
    },
    InjectableMacro {
        name: "http2_version",
        handler: Handler::NoAttrPosition(http2_version_macro),
    },
    InjectableMacro {
        name: "http3_version",
        handler: Handler::NoAttrPosition(http3_version_macro),
    },
    InjectableMacro {
        name: "http1_1_or_higher_version",
        handler: Handler::NoAttrPosition(http1_1_or_higher_version_macro),
    },
    InjectableMacro {
        name: "http_version",
        handler: Handler::NoAttrPosition(http_version_macro),
    },
    InjectableMacro {
        name: "unknown_version",
        handler: Handler::NoAttrPosition(unknown_version_macro),
    },
];
