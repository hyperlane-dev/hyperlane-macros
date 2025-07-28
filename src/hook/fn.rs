use crate::*;

/// Expands macro to add pre-hook function call.
///
/// # Arguments
///
/// - `TokenStream` - The hook function name token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with pre-hook call.
pub(crate) fn pre_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}

/// Expands macro to add post-hook function call.
///
/// # Arguments
///
/// - `TokenStream` - The hook function name token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with post-hook call.
pub(crate) fn post_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}
