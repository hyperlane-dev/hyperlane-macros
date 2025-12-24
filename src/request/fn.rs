use crate::*;

/// Gets raw request body and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body extraction.
pub(crate) fn request_body_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_body: MultiRequestBodyData = parse_macro_input!(attr as MultiRequestBodyData);
    inject(position, item, |context| {
        let statements = multi_body.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::RequestBody = #context.get_request_body().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_body",
        handler: Handler::WithAttrPosition(request_body_macro),
    }
}

/// Parses request body as JSON and assigns to specified variable.
/// Supports both single and multiple variable-type pair extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with JSON parsing.
pub(crate) fn request_body_json_result_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_body_json: MultiRequestBodyJsonData =
        parse_macro_input!(attr as MultiRequestBodyJsonData);
    inject(position, item, |context| {
        let statements = multi_body_json.params.iter().map(|(variable, type_name)| {
            quote! {
                let #variable: ::hyperlane::ResultJsonError<#type_name> = #context.try_get_request_body_json::<#type_name>().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_body_json_result",
        handler: Handler::WithAttrPosition(request_body_json_result_macro),
    }
}

/// Parses request body as JSON and assigns to specified variable.
/// Supports both single and multiple variable-type pair extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with JSON parsing.
pub(crate) fn request_body_json_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_body_json: MultiRequestBodyJsonData =
        parse_macro_input!(attr as MultiRequestBodyJsonData);
    inject(position, item, |context| {
        let statements = multi_body_json.params.iter().map(|(variable, type_name)| {
            quote! {
                let #variable: #type_name = #context.get_request_body_json::<#type_name>().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_body_json",
        handler: Handler::WithAttrPosition(request_body_json_macro),
    }
}

/// Gets request attribute by key and assigns to specified variable.
/// Supports both single and multiple attribute extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attribute extraction.
pub(crate) fn attribute_option_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_attr: MultiAttributeData = parse_macro_input!(attr as MultiAttributeData);
    inject(position, item, |context| {
        let statements = multi_attr
            .params
            .iter()
            .map(|(key_name, variable, type_name)| {
                quote! {
                    let #variable: Option<#type_name> = #context.try_get_attribute(&#key_name).await;
                }
            });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "attribute_option",
        handler: Handler::WithAttrPosition(attribute_option_macro),
    }
}

/// Gets request attribute by key and assigns to specified variable.
/// Supports both single and multiple attribute extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attribute extraction.
pub(crate) fn attribute_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_attr: MultiAttributeData = parse_macro_input!(attr as MultiAttributeData);
    inject(position, item, |context| {
        let statements = multi_attr
            .params
            .iter()
            .map(|(key_name, variable, type_name)| {
                quote! {
                    let #variable: #type_name = #context.get_attribute(&#key_name).await;
                }
            });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "attribute",
        handler: Handler::WithAttrPosition(attribute_macro),
    }
}

/// Gets all request attributes and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attributes extraction.
pub(crate) fn attributes_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_attrs: MultiAttributesData = parse_macro_input!(attr as MultiAttributesData);
    inject(position, item, |context| {
        let statements = multi_attrs.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::ThreadSafeAttributeStore = #context.get_attributes().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "attributes",
        handler: Handler::WithAttrPosition(attributes_macro),
    }
}

/// Gets route parameter by key and assigns to specified variable.
/// Supports both single and multiple route parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route param extraction.
pub(crate) fn route_param_option_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_param: MultiRouteParamData = parse_macro_input!(attr as MultiRouteParamData);
    inject(position, item, |context| {
        let statements = multi_param.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::OptionString = #context.try_get_route_param(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "route_param_option",
        handler: Handler::WithAttrPosition(route_param_option_macro),
    }
}

/// Gets route parameter by key and assigns to specified variable.
/// Supports both single and multiple route parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route param extraction.
pub(crate) fn route_param_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_param: MultiRouteParamData = parse_macro_input!(attr as MultiRouteParamData);
    inject(position, item, |context| {
        let statements = multi_param.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: String = #context.get_route_param(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "route_param",
        handler: Handler::WithAttrPosition(route_param_macro),
    }
}

/// Gets all route parameters and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route params extraction.
pub(crate) fn route_params_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_route_params: MultiRouteParamsData = parse_macro_input!(attr as MultiRouteParamsData);
    inject(position, item, |context| {
        let statements = multi_route_params.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::RouteParams = #context.get_route_params().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "route_params",
        handler: Handler::WithAttrPosition(route_params_macro),
    }
}

/// Gets request query parameter by key and assigns to specified variable.
/// Supports both single and multiple parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query param extraction.
pub(crate) fn request_query_option_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_query: MultiQueryData = parse_macro_input!(attr as MultiQueryData);
    inject(position, item, |context| {
        let statements = multi_query.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::OptionRequestQuerysValue = #context.try_get_request_query(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_query_option",
        handler: Handler::WithAttrPosition(request_query_option_macro),
    }
}

/// Gets request query parameter by key and assigns to specified variable.
/// Supports both single and multiple parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query param extraction.
pub(crate) fn request_query_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_query: MultiQueryData = parse_macro_input!(attr as MultiQueryData);
    inject(position, item, |context| {
        let statements = multi_query.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::RequestQuerysValue = #context.get_request_query(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_query",
        handler: Handler::WithAttrPosition(request_query_macro),
    }
}

/// Gets all request query parameters and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query params extraction.
pub(crate) fn request_querys_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_querys: MultiQuerysData = parse_macro_input!(attr as MultiQuerysData);
    inject(position, item, |context| {
        let statements = multi_querys.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::RequestQuerys = #context.get_request_querys().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_querys",
        handler: Handler::WithAttrPosition(request_querys_macro),
    }
}

/// Gets request header by key and assigns to specified variable.
/// Supports both single and multiple header extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header extraction.
pub(crate) fn request_header_option_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_header: MultiHeaderData = parse_macro_input!(attr as MultiHeaderData);
    inject(position, item, |context| {
        let statements = multi_header.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::OptionRequestHeadersValueItem = #context.try_get_request_header_back(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_header_option",
        handler: Handler::WithAttrPosition(request_header_option_macro),
    }
}

/// Gets request header by key and assigns to specified variable.
/// Supports both single and multiple header extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header extraction.
pub(crate) fn request_header_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_header: MultiHeaderData = parse_macro_input!(attr as MultiHeaderData);
    inject(position, item, |context| {
        let statements = multi_header.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::RequestHeadersValueItem = #context.get_request_header_back(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_header",
        handler: Handler::WithAttrPosition(request_header_macro),
    }
}

/// Gets all request headers and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with headers extraction.
pub(crate) fn request_headers_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_headers: MultiHeadersData = parse_macro_input!(attr as MultiHeadersData);
    inject(position, item, |context| {
        let statements = multi_headers.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::RequestHeaders = #context.get_request_headers().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_headers",
        handler: Handler::WithAttrPosition(request_headers_macro),
    }
}

/// Gets request cookie by key and assigns to specified variable.
/// Supports both single and multiple cookie extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookie extraction.
pub(crate) fn request_cookie_option_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_cookie: MultiCookieData = parse_macro_input!(attr as MultiCookieData);
    inject(position, item, |context| {
        let statements = multi_cookie.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::OptionCookieValue = #context.try_get_request_cookie(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_cookie_option",
        handler: Handler::WithAttrPosition(request_cookie_option_macro),
    }
}

/// Gets request cookie by key and assigns to specified variable.
/// Supports both single and multiple cookie extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookie extraction.
pub(crate) fn request_cookie_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_cookie: MultiCookieData = parse_macro_input!(attr as MultiCookieData);
    inject(position, item, |context| {
        let statements = multi_cookie.params.iter().map(|(key_name, variable)| {
            quote! {
                let #variable: ::hyperlane::CookieValue = #context.get_request_cookie(#key_name).await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_cookie",
        handler: Handler::WithAttrPosition(request_cookie_macro),
    }
}

/// Gets all request cookies and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookies extraction.
pub(crate) fn request_cookies_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_cookies: MultiCookiesData = parse_macro_input!(attr as MultiCookiesData);
    inject(position, item, |context| {
        let statements = multi_cookies.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::Cookies = #context.get_request_cookies().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_cookies",
        handler: Handler::WithAttrPosition(request_cookies_macro),
    }
}

/// Gets request version and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with version extraction.
pub(crate) fn request_version_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_version: MultiRequestVersionData =
        parse_macro_input!(attr as MultiRequestVersionData);
    inject(position, item, |context| {
        let statements = multi_version.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::RequestVersion = #context.get_request_version().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_version",
        handler: Handler::WithAttrPosition(request_version_macro),
    }
}

/// Gets request path and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with path extraction.
pub(crate) fn request_path_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_path: MultiRequestPathData = parse_macro_input!(attr as MultiRequestPathData);
    inject(position, item, |context| {
        let statements = multi_path.variables.iter().map(|variable| {
            quote! {
                let #variable: ::hyperlane::RequestPath = #context.get_request_path().await;
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "request_path",
        handler: Handler::WithAttrPosition(request_path_macro),
    }
}
