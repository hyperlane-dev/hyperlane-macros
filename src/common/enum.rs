use crate::*;

pub(crate) enum Handler {
    Simple(MacroHandler),
    WithAttr(MacroHandlerWithAttr),
}
