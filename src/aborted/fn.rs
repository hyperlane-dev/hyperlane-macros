use crate::*;

/// Expands the macro to generate an asynchronous aborted call.
///
/// This macro takes a `TokenStream` as input, which typically represents
/// the context of a function or block, and inserts a call to `.aborted().await`
/// on that context. This is useful for ensuring that a component gracefully
/// handles being aborted.
///
/// # Arguments
///
/// - `TokenStream` - The input `TokenStream` to process.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the aborted call inserted.
pub(crate) fn aborted_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.aborted().await;
        }
    })
}
