use crate::*;

/// Referer data container storing referer value expression.
///
/// Used for Referer header matching in request processing.
pub(crate) struct RefererData {
    /// The referer value expression to match against.
    pub(crate) referer_value: Expr,
}

/// Multiple referer data container storing multiple referer value expressions.
///
/// Used for multiple Referer header matching in request processing.
pub(crate) struct MultiRefererData {
    /// Vector of referer value expressions to match against.
    pub(crate) referer_values: Vec<Expr>,
}
