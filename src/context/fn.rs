use crate::*;

/// Implements the context! procedural macro.
///
/// This macro generates a let statement that converts a context pointer into a mutable reference.
/// The generated code performs an unsafe conversion from a raw pointer to a mutable reference
/// through the `Into` trait.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream containing the variable name identifier.
///
/// # Returns
///
/// - `TokenStream` - The generated let statement binding the variable to the converted context reference.
pub(crate) fn context_macro(input: TokenStream) -> TokenStream {
    let context_input: ContextInput = match parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };
    let source_ctx: Ident = context_input.source_ctx;
    into_new_context(&source_ctx).into()
}
