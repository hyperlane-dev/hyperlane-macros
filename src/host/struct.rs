use crate::*;

/// Host data container storing host value expression.
///
/// Used for host matching in request processing.
pub(crate) struct HostData {
    /// The host value expression to match against.
    pub(crate) host_value: Expr,
}
