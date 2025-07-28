use crate::*;

/// Referer data container storing referer value expression.
///
/// Used for Referer header matching in request processing.
pub(crate) struct RefererData {
    /// The referer value expression to match against.
    pub(crate) referer_value: Expr,
}
