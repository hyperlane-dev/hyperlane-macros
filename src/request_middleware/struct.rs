use crate::*;

/// Represents the attributes for the request middleware macro.
///
/// This struct is used to parse the optional priority attribute
/// provided to the `request_middleware` macro.
pub(crate) struct RequestMiddlewareAttr {
    /// The priority of the middleware.
    pub(crate) priority: Expr,
}
