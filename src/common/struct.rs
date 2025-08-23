use crate::*;

/// Represents a parsed attribute containing a `path`.
///
/// This struct is used by macro parsing implementations to hold the extracted path expression.
pub(crate) struct PathAttr {
    /// The path expression provided in the macro attribute.
    pub(crate) path: Expr,
}

/// Represents a parsed macro attribute containing an optional order.
///
/// This struct is used during macro parsing to hold the extracted order expression.
/// Hooks or attributes that do not specify an order will have `None`.
#[derive(Clone)]
pub(crate) struct OrderAttr {
    /// The optional order expression provided in the macro attribute.
    pub(crate) order: Option<Expr>,
}
