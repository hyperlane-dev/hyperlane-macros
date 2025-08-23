use crate::*;

/// Represents the attributes for the `disable_ws_hook` macro.
///
/// This struct parses the input attributes for the `disable_ws_hook` macro,
/// specifically extracting the path for which the WebSocket hook should be disabled.
pub(crate) struct DisableWsHookAttr {
    /// The path expression for disabling the WebSocket hook.
    pub(crate) path: Expr,
}
