use crate::*;

/// Defines the type of macro handler.
///
/// This enum is used to differentiate between simple macros and those that take attributes.
pub(crate) enum Handler {
    /// A simple macro handler that takes no attributes.
    SimplePosition(MacroHandlerPosition),
    /// A macro handler that takes attributes.
    WithAttr(MacroHandlerWithAttr),
    WithAttrPosition(MacroHandlerWithAttrPosition),
}

pub(crate) enum Position {
    Start,
    End,
}
