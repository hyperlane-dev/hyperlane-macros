use crate::*;

/// Expands the macro to generate an asynchronous closed call.
///
/// This macro takes a `TokenStream` as input, which typically represents
/// the context of a function or block, and inserts a call to `.closed().await`
/// on that context. This is useful for ensuring that a component gracefully
/// handles being closed.
///
/// # Arguments
///
/// - `item` - The input `TokenStream` to process.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the closed call inserted.
pub(crate) fn closed_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.closed().await;
        }
    })
}
