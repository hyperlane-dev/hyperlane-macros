use crate::*;

/// Represents a parsed attribute containing a `path`.
///
/// This struct is used by macro parsing implementations to hold the extracted path expression.
pub(crate) struct PathAttr {
    /// The path expression provided in the macro attribute.
    pub(crate) path: Expr,
}

/// Represents a parsed attribute containing an optional `order`.
///
/// This struct is used by macro parsing implementations to hold the extracted order expression.
/// If the `order` is not specified in the macro, it defaults to `0`.
pub(crate) struct OrderAttr {
    /// The order expression provided in the macro attribute.
    /// Defaults to `0` if not specified.
    pub(crate) order: Expr,
}
