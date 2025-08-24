use crate::*;

/// Gets raw request body and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body extraction.
pub(crate) fn request_body_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_param: RequestBodyData = parse_macro_input!(attr as RequestBodyData);
    let variable: Ident = body_param.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::RequestBody = #context.get_request_body().await;
        }
    })
}

/// Parses request body as JSON and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with JSON parsing.
pub(crate) fn request_body_json_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_param: RequestBodyJsonData = parse_macro_input!(attr as RequestBodyJsonData);
    let variable: Ident = body_param.variable;
    let type_name: Type = body_param.type_name;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::ResultJsonError<#type_name> = #context.get_request_body_json::<#type_name>().await;
        }
    })
}

/// Gets request attribute by key and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attribute extraction.
pub(crate) fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attribute: AttributeData = parse_macro_input!(attr as AttributeData);
    let variable: Ident = attribute.variable;
    let type_name: Type = attribute.type_name;
    let key_name: Expr = attribute.key_name;
    inject_at_start(item, |context| {
        quote! {
            let #variable: Option<#type_name> = #context.get_attribute::<#type_name>(#key_name).await;
        }
    })
}

/// Gets all request attributes and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attributes extraction.
pub(crate) fn attributes_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes: AttributesData = parse_macro_input!(attr as AttributesData);
    let variable: Ident = attributes.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::HashMapArcAnySendSync = #context.get_attributes().await;
        }
    })
}

/// Gets route parameter by key and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route param extraction.
pub(crate) fn route_param_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_param: RouteParamData = parse_macro_input!(attr as RouteParamData);
    let variable: Ident = route_param.variable;
    let key_name: Expr = route_param.key_name;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::OptionString = #context.get_route_param(#key_name).await;
        }
    })
}

/// Gets all route parameters and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route params extraction.
pub(crate) fn route_params_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_params: RouteParamsData = parse_macro_input!(attr as RouteParamsData);
    let variable: Ident = route_params.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::RouteParams = #context.get_route_params().await;
        }
    })
}

/// Gets request query parameter by key and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query param extraction.
pub(crate) fn request_query_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_query: QueryData = parse_macro_input!(attr as QueryData);
    let variable: Ident = request_query.variable;
    let key_name: Expr = request_query.key_name;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::OptionRequestQuerysValue = #context.get_request_query(#key_name).await;
        }
    })
}

/// Gets all request query parameters and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query params extraction.
pub(crate) fn request_querys_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_query: QuerysData = parse_macro_input!(attr as QuerysData);
    let variable: Ident = request_query.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::RequestQuerys = #context.get_request_querys().await;
        }
    })
}

/// Gets request header by key and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header extraction.
pub(crate) fn request_header_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_header: HeaderData = parse_macro_input!(attr as HeaderData);
    let variable: Ident = request_header.variable;
    let key_name: Expr = request_header.key_name;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::OptionRequestHeadersValueItem = #context.get_request_header_back(#key_name).await;
        }
    })
}

/// Gets all request headers and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with headers extraction.
pub(crate) fn request_headers_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_headers: HeadersData = parse_macro_input!(attr as HeadersData);
    let variable: Ident = request_headers.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::RequestHeaders = #context.get_request_headers().await;
        }
    })
}

/// Gets request cookie by key and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookie extraction.
pub(crate) fn request_cookie_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cookie_data: CookieData = parse_macro_input!(attr as CookieData);
    let variable: Ident = cookie_data.variable;
    let key: Expr = cookie_data.key_name;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::OptionCookiesValue = #context.get_request_cookie(#key).await;
        }
    })
}

/// Gets all request cookies and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookies extraction.
pub(crate) fn request_cookies_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cookies_data: CookiesData = parse_macro_input!(attr as CookiesData);
    let variable: Ident = cookies_data.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::Cookies = #context.get_request_cookies().await;
        }
    })
}

/// Gets request version and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with version extraction.
pub(crate) fn request_version_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let version_data: RequestVersionData = parse_macro_input!(attr as RequestVersionData);
    let variable: Ident = version_data.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::RequestVersion = #context.get_request_version().await;
        }
    })
}

/// Gets request path and assigns to specified variable.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with path extraction.
pub(crate) fn request_path_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_data: RequestPathData = parse_macro_input!(attr as RequestPathData);
    let variable: Ident = path_data.variable;
    inject_at_start(item, |context| {
        quote! {
            let #variable: ::hyperlane::RequestPath = #context.get_request_path().await;
        }
    })
}
