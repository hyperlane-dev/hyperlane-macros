use crate::*;

/// Defines the type of macro handler.
///
/// This enum is used to differentiate between simple macros and those that take attributes.
pub(crate) enum Handler {
    /// A simple macro handler that takes no attributes.
    Simple(MacroHandler),
    /// A macro handler that takes attributes.
    WithAttr(MacroHandlerWithAttr),
}
