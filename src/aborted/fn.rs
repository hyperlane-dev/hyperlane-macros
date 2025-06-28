use crate::*;

pub(crate) fn aborted_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.aborted().await;
        }
    })
}
