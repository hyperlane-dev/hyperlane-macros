use crate::*;

pub(crate) struct RequestMethods {
    pub(crate) methods: Punctuated<Ident, Token![,]>,
}

pub(crate) struct RequestBodyData {
    pub(crate) variable: Ident,
}

pub(crate) struct RequestBodyJsonData {
    pub(crate) variable: Ident,
    pub(crate) type_name: Type,
}

pub(crate) struct AttributeData {
    pub(crate) variable: Ident,
    pub(crate) type_name: Type,
    pub(crate) key_name: Expr,
}

pub(crate) struct AttributesData {
    pub(crate) variable: Ident,
}

pub(crate) struct RouteParamData {
    pub(crate) variable: Ident,
    pub(crate) key_name: Expr,
}

pub(crate) struct RouteParamsData {
    pub(crate) variable: Ident,
}

pub(crate) struct QueryData {
    pub(crate) variable: Ident,
    pub(crate) key_name: Expr,
}

pub(crate) struct QuerysData {
    pub(crate) variable: Ident,
}

pub(crate) struct HeaderData {
    pub(crate) variable: Ident,
    pub(crate) key_name: Expr,
}

pub(crate) struct HeadersData {
    pub(crate) variable: Ident,
}

pub(crate) struct CookieData {
    pub(crate) variable: Ident,
    pub(crate) key_name: Expr,
}

pub(crate) struct CookiesData {
    pub(crate) variable: Ident,
}

pub(crate) struct RequestVersionData {
    pub(crate) variable: Ident,
}

pub(crate) struct RequestPathData {
    pub(crate) variable: Ident,
}
