use crate::*;

pub(crate) fn body_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_param: BodyData = parse_macro_input!(attr as BodyData);
    let variable: Ident = body_param.variable;
    let type_name: Type = body_param.type_name;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: Result<#type_name, JsonError> = #context.get_request_body_json::<#type_name>().await;
        }
    })
}

pub(crate) fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attribute: AttributeData = parse_macro_input!(attr as AttributeData);
    let variable: Ident = attribute.variable;
    let type_name: Type = attribute.type_name;
    let key_name: Expr = attribute.key_name;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: Option<#type_name> = #context.get_attribute::<#type_name>(#key_name).await;
        }
    })
}

pub(crate) fn attributes_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes: AttributesData = parse_macro_input!(attr as AttributesData);
    let variable: Ident = attributes.variable;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: HashMapArcAnySendSync = #context.get_attributes().await;
        }
    })
}

pub(crate) fn route_param_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_param: RouteParamData = parse_macro_input!(attr as RouteParamData);
    let variable: Ident = route_param.variable;
    let key_name: Expr = route_param.key_name;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: OptionString = #context.get_route_param(#key_name).await;
        }
    })
}

pub(crate) fn route_params_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_params: RouteParamsData = parse_macro_input!(attr as RouteParamsData);
    let variable: Ident = route_params.variable;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: RouteParams = #context.get_route_params().await;
        }
    })
}

pub(crate) fn request_query_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_query: QueryData = parse_macro_input!(attr as QueryData);
    let variable: Ident = request_query.variable;
    let key_name: Expr = request_query.key_name;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: OptionRequestQuerysValue = #context.get_request_query(#key_name).await;
        }
    })
}

pub(crate) fn request_querys_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_query: QuerysData = parse_macro_input!(attr as QuerysData);
    let variable: Ident = request_query.variable;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: RequestQuerys = #context.get_request_querys().await;
        }
    })
}

pub(crate) fn request_header_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_header: HeaderData = parse_macro_input!(attr as HeaderData);
    let variable: Ident = request_header.variable;
    let key_name: Expr = request_header.key_name;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: OptionRequestHeadersValue = #context.get_request_header(#key_name).await;
        }
    })
}

pub(crate) fn request_headers_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_headers: HeadersData = parse_macro_input!(attr as HeadersData);
    let variable: Ident = request_headers.variable;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: RequestHeaders = #context.get_request_headers().await;
        }
    })
}
