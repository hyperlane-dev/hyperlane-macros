use crate::*;

/// Host data container storing host value expression.
///
/// Used for host matching in request processing.
pub(crate) struct HostData {
    /// The host value expression to match against.
    pub(crate) host_value: Expr,
}

/// Multiple host data container storing multiple host value expressions.
///
/// Used for multiple host matching in request processing.
pub(crate) struct MultiHostData {
    /// Vector of host value expressions to match against.
    pub(crate) host_values: Vec<Expr>,
}
