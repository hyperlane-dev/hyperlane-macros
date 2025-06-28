use crate::*;

pub(crate) fn closed_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.closed().await;
        }
    })
}
