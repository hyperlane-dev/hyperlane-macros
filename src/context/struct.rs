use crate::*;

/// Parses the input for context macro.
///
/// This struct represents the source context identifier parsed from the macro input.
pub(crate) struct ContextInput {
    /// The source context variable identifier.
    pub(crate) source_ctx: Ident,
}
