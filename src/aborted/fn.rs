use crate::*;

/// Expands macro to generate async aborted call.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with aborted call.
pub(crate) fn aborted_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.aborted().await;
        }
    })
}
