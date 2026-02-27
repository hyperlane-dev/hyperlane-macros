use crate::*;

/// Implementation of Parse trait for ContextInput.
///
/// Parses a single identifier from the input stream.
/// The identifier is the source context variable.
impl Parse for ContextInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let source_ctx: Ident = input.parse()?;
        Ok(ContextInput { source_ctx })
    }
}
