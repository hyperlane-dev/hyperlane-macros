use crate::*;

/// Represents the attributes for the hook macro.
///
/// This struct is used to parse the optional order attribute
/// provided to the hook macros.
pub(crate) struct HookAttr {
    /// The order of the hook.
    pub(crate) order: Expr,
}

/// Represents a parsed attribute containing a `path` and an optional `order`.
///
/// This struct is used by the `PathAndOrderAttr` macro parsing implementation
/// to hold the extracted expressions. If the `order` is not specified in the macro,
/// it defaults to `0`.
pub(crate) struct PathAndOrderAttr {
    /// The path expression provided in the macro attribute.
    pub(crate) path: Expr,
    /// The order expression provided in the macro attribute.
    /// Defaults to `0` if not specified.
    pub(crate) order: Expr,
}
