use crate::*;

/// Expands the macro to generate an asynchronous aborted call.
///
/// This macro takes a `TokenStream` as input, which typically represents
/// the context of a function or block, and inserts a call to `.set_aborted(true)`
/// on that context. This is useful for ensuring that a component gracefully
/// handles being aborted.
///
/// # Arguments
///
/// - `TokenStream` - The input `TokenStream` to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the aborted call inserted.
pub(crate) fn aborted_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context| {
        quote! {
            std::convert::Into::<&mut ::hyperlane::Context>::into(#context as *mut ::hyperlane::Context as usize).set_aborted(true);
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "aborted",
        handler: Handler::NoAttrPosition(aborted_macro),
    }
}
