use crate::*;

/// Implementation of Parse trait for RequestMethods.
///
/// Parses HTTP methods from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RequestMethods>` - Parsed RequestMethods or error.
impl Parse for RequestMethods {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RequestMethods {
            methods: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

/// Implementation of Parse trait for RequestBodyData.
///
/// Parses request body variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RequestBodyData>` - Parsed RequestBodyData or error.
impl Parse for RequestBodyData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(RequestBodyData { variable })
    }
}

/// Implementation of Parse trait for MultiRequestBodyData.
///
/// Parses multiple request body variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestBodyData>` - Parsed MultiRequestBodyData or error.
impl Parse for MultiRequestBodyData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestBodyData { variables })
    }
}

/// Implementation of Parse trait for RequestBodyJsonData.
///
/// Parses request body JSON variable and type from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RequestBodyJsonData>` - Parsed RequestBodyJsonData or error.
impl Parse for RequestBodyJsonData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_name: Type = input.parse()?;
        Ok(RequestBodyJsonData {
            variable,
            type_name,
        })
    }
}

/// Implementation of Parse trait for MultiRequestBodyJsonData.
///
/// Parses multiple request body JSON variable-type pairs from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestBodyJsonData>` - Parsed MultiRequestBodyJsonData or error.
impl Parse for MultiRequestBodyJsonData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Ident, Type)> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Type = input.parse()?;
            params.push((variable, type_name));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestBodyJsonData { params })
    }
}

/// Implementation of Parse trait for AttributeData.
///
/// Parses attribute key, variable and type from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<AttributeData>` - Parsed AttributeData or error.
impl Parse for AttributeData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_name: Type = input.parse()?;
        Ok(AttributeData {
            variable,
            key_name,
            type_name,
        })
    }
}

/// Implementation of Parse trait for MultiAttributeData.
///
/// Parses multiple attribute key-variable-type tuples from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiAttributeData>` - Parsed MultiAttributeData or error.
impl Parse for MultiAttributeData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident, Type)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Type = input.parse()?;
            params.push((key_name, variable, type_name));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiAttributeData { params })
    }
}

/// Implementation of Parse trait for AttributesData.
///
/// Parses attributes variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<AttributesData>` - Parsed AttributesData or error.
impl Parse for AttributesData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(AttributesData { variable })
    }
}

/// Implementation of Parse trait for MultiAttributesData.
///
/// Parses multiple attributes variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiAttributesData>` - Parsed MultiAttributesData or error.
impl Parse for MultiAttributesData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiAttributesData { variables })
    }
}

/// Implementation of Parse trait for RouteParamData.
///
/// Parses route parameter key and variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RouteParamData>` - Parsed RouteParamData or error.
impl Parse for RouteParamData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(RouteParamData { key_name, variable })
    }
}

/// Implementation of Parse trait for MultiRouteParamData.
///
/// Parses multiple route parameter key-variable pairs from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRouteParamData>` - Parsed MultiRouteParamData or error.
impl Parse for MultiRouteParamData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRouteParamData { params })
    }
}

/// Implementation of Parse trait for RouteParamsData.
///
/// Parses route parameters variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RouteParamsData>` - Parsed RouteParamsData or error.
impl Parse for RouteParamsData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(RouteParamsData { variable })
    }
}

/// Implementation of Parse trait for MultiRouteParamsData.
///
/// Parses multiple route parameters variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRouteParamsData>` - Parsed MultiRouteParamsData or error.
impl Parse for MultiRouteParamsData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRouteParamsData { variables })
    }
}

/// Implementation of Parse trait for QueryData.
///
/// Parses query parameter key and variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<QueryData>` - Parsed QueryData or error.
impl Parse for QueryData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(QueryData { key_name, variable })
    }
}

/// Implementation of Parse trait for MultiQueryData.
///
/// Parses multiple query parameter key-variable pairs from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiQueryData>` - Parsed MultiQueryData or error.
impl Parse for MultiQueryData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiQueryData { params })
    }
}

/// Implementation of Parse trait for QuerysData.
///
/// Parses query parameters variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<QuerysData>` - Parsed QuerysData or error.
impl Parse for QuerysData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(QuerysData { variable })
    }
}

/// Implementation of Parse trait for MultiQuerysData.
///
/// Parses multiple query parameters variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiQuerysData>` - Parsed MultiQuerysData or error.
impl Parse for MultiQuerysData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiQuerysData { variables })
    }
}

/// Implementation of Parse trait for HeaderData.
///
/// Parses header key and variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<HeaderData>` - Parsed HeaderData or error.
impl Parse for HeaderData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(HeaderData { key_name, variable })
    }
}

/// Implementation of Parse trait for MultiHeaderData.
///
/// Parses multiple header key-variable pairs from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiHeaderData>` - Parsed MultiHeaderData or error.
impl Parse for MultiHeaderData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiHeaderData { params })
    }
}

/// Implementation of Parse trait for HeadersData.
///
/// Parses headers variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<HeadersData>` - Parsed HeadersData or error.
impl Parse for HeadersData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(HeadersData { variable })
    }
}

/// Implementation of Parse trait for MultiHeadersData.
///
/// Parses multiple headers variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiHeadersData>` - Parsed MultiHeadersData or error.
impl Parse for MultiHeadersData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiHeadersData { variables })
    }
}

/// Implementation of Parse trait for CookieData.
///
/// Parses cookie key and variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<CookieData>` - Parsed CookieData or error.
impl Parse for CookieData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_name: Expr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let variable: Ident = input.parse()?;
        Ok(CookieData { variable, key_name })
    }
}

/// Implementation of Parse trait for MultiCookieData.
///
/// Parses multiple cookie key-variable pairs from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiCookieData>` - Parsed MultiCookieData or error.
impl Parse for MultiCookieData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiCookieData { params })
    }
}

/// Implementation of Parse trait for CookiesData.
///
/// Parses cookies variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<CookiesData>` - Parsed CookiesData or error.
impl Parse for CookiesData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(CookiesData { variable })
    }
}

/// Implementation of Parse trait for MultiCookiesData.
///
/// Parses multiple cookies variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiCookiesData>` - Parsed MultiCookiesData or error.
impl Parse for MultiCookiesData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiCookiesData { variables })
    }
}

/// Implementation of Parse trait for RequestVersionData.
///
/// Parses request version variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RequestVersionData>` - Parsed RequestVersionData or error.
impl Parse for RequestVersionData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(RequestVersionData { variable })
    }
}

/// Implementation of Parse trait for MultiRequestVersionData.
///
/// Parses multiple request version variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestVersionData>` - Parsed MultiRequestVersionData or error.
impl Parse for MultiRequestVersionData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestVersionData { variables })
    }
}

/// Implementation of Parse trait for RequestPathData.
///
/// Parses request path variable from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RequestPathData>` - Parsed RequestPathData or error.
impl Parse for RequestPathData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        Ok(RequestPathData { variable })
    }
}

/// Implementation of Parse trait for MultiRequestPathData.
///
/// Parses multiple request path variables from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestPathData>` - Parsed MultiRequestPathData or error.
impl Parse for MultiRequestPathData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestPathData { variables })
    }
}
