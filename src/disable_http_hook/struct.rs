use crate::*;

/// Represents the attributes for the `disable_http_hook` macro.
///
/// This struct parses the input attributes for the `disable_http_hook` macro,
/// specifically extracting the path for which the HTTP hook should be disabled.
pub(crate) struct DisableHttpHookAttr {
    /// The path expression for disabling the HTTP hook.
    pub(crate) path: Expr,
}
