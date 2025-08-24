use crate::*;

/// Expands macro to generate async flush call.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with flush call.
pub(crate) fn flush_macro(item: TokenStream) -> TokenStream {
    inject_at_end(item, |context| {
        quote! {
            let _ = #context.flush().await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "flush",
        handler: Handler::Simple(flush_macro),
    }
}
