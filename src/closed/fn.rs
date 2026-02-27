use crate::*;

/// Expands the macro to generate an asynchronous closed call.
///
/// This macro takes a `TokenStream` as input, which typically represents
/// the context of a function or block, and inserts a call to `.set_closed(true)`
/// on that context. This is useful for ensuring that a component gracefully
/// handles being closed.
///
/// # Arguments
///
/// - `TokenStream` - The input `TokenStream` to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the closed call inserted.
pub(crate) fn closed_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        let new_context: TokenStream2 = into_new_context(context);
        quote! {
            #new_context.set_closed(true);
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "closed",
        handler: Handler::NoAttrPosition(closed_macro),
    }
}
