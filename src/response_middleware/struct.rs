use crate::*;

/// Represents the attributes for the response middleware macro.
///
/// This struct is used to parse the optional priority attribute
/// provided to the `response_middleware` macro.
pub(crate) struct ResponseMiddlewareAttr {
    /// The priority of the middleware.
    pub(crate) priority: Expr,
}
