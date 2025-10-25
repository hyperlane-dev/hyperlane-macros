use crate::*;

/// Container for HTTP methods data.
///
/// Used to store parsed HTTP methods from macro input.
pub(crate) struct RequestMethods {
    /// The parsed HTTP methods as punctuated identifiers.
    pub(crate) methods: Punctuated<Ident, Token![,]>,
}

/// Container for raw request body data.
///
/// Used to store parsed request body variable from macro input.
pub(crate) struct RequestBodyData {
    /// The variable name to store the request body.
    pub(crate) variable: Ident,
}

/// Container for JSON request body data.
///
/// Used to store parsed JSON request body variable and type from macro input.
pub(crate) struct RequestBodyJsonData {
    /// The variable name to store the parsed JSON.
    pub(crate) variable: Ident,
    /// The type to parse the JSON into.
    pub(crate) type_name: Type,
}

/// Container for request attribute data.
///
/// Used to store parsed attribute key, variable and type from macro input.
pub(crate) struct AttributeData {
    /// The variable name to store the attribute value.
    pub(crate) variable: Ident,
    /// The type to parse the attribute into.
    pub(crate) type_name: Type,
    /// The attribute key name.
    pub(crate) key_name: Expr,
}

/// Container for request attributes data.
///
/// Used to store parsed attributes variable from macro input.
pub(crate) struct AttributesData {
    /// The variable name to store all attributes.
    pub(crate) variable: Ident,
}

/// Container for route parameter data.
///
/// Used to store parsed route parameter key and variable from macro input.
pub(crate) struct RouteParamData {
    /// The variable name to store the route parameter value.
    pub(crate) variable: Ident,
    /// The route parameter key name.
    pub(crate) key_name: Expr,
}

/// Container for multiple route parameters data.
///
/// Used to store parsed multiple route parameter key-variable pairs from macro input.
pub(crate) struct MultiRouteParamData {
    /// Vector of route parameter key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for route parameters data.
///
/// Used to store parsed route parameters variable from macro input.
pub(crate) struct RouteParamsData {
    /// The variable name to store all route parameters.
    pub(crate) variable: Ident,
}

/// Container for query parameter data.
///
/// Used to store parsed query parameter key and variable from macro input.
pub(crate) struct QueryData {
    /// The variable name to store the query parameter value.
    pub(crate) variable: Ident,
    /// The query parameter key name.
    pub(crate) key_name: Expr,
}

/// Container for multiple query parameters data.
///
/// Used to store parsed multiple query parameter key-variable pairs from macro input.
pub(crate) struct MultiQueryData {
    /// Vector of query parameter key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for query parameters data.
///
/// Used to store parsed query parameters variable from macro input.
pub(crate) struct QuerysData {
    /// The variable name to store all query parameters.
    pub(crate) variable: Ident,
}

/// Container for request header data.
///
/// Used to store parsed header key and variable from macro input.
pub(crate) struct HeaderData {
    /// The variable name to store the header value.
    pub(crate) variable: Ident,
    /// The header key name.
    pub(crate) key_name: Expr,
}

/// Container for multiple request headers data.
///
/// Used to store parsed multiple header key-variable pairs from macro input.
pub(crate) struct MultiHeaderData {
    /// Vector of header key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for request headers data.
///
/// Used to store parsed headers variable from macro input.
pub(crate) struct HeadersData {
    /// The variable name to store all headers.
    pub(crate) variable: Ident,
}

/// Container for request cookie data.
///
/// Used to store parsed cookie key and variable from macro input.
pub(crate) struct CookieData {
    /// The variable name to store the cookie value.
    pub(crate) variable: Ident,
    /// The cookie key name.
    pub(crate) key_name: Expr,
}

/// Container for multiple request cookies data.
///
/// Used to store parsed multiple cookie key-variable pairs from macro input.
pub(crate) struct MultiCookieData {
    /// Vector of cookie key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for request cookies data.
///
/// Used to store parsed cookies variable from macro input.
pub(crate) struct CookiesData {
    /// The variable name to store all cookies.
    pub(crate) variable: Ident,
}

/// Container for request version data.
///
/// Used to store parsed request version variable from macro input.
pub(crate) struct RequestVersionData {
    /// The variable name to store the request version.
    pub(crate) variable: Ident,
}

/// Container for request path data.
///
/// Used to store parsed request path variable from macro input.
pub(crate) struct RequestPathData {
    /// The variable name to store the request path.
    pub(crate) variable: Ident,
}
