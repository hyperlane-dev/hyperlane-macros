use crate::*;

/// Expands macro to generate async try_flush call.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with try_flush call.
pub(crate) fn try_flush_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        let new_context: TokenStream2 = leak_mut_context(context);
        quote! {
            let _ = #new_context.try_flush().await;
        }
    })
}

/// Expands macro to generate async flush call.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with flush call.
pub(crate) fn flush_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {{
            let new_context: &mut Context = (#context as *mut Context as usize).into();
            new_context.flush().await;
        }}
    })
}
