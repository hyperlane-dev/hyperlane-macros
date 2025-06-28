use crate::*;

pub(crate) fn send_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.send().await;
        }
    })
}

pub(crate) fn send_body_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.send_body().await;
        }
    })
}

pub(crate) fn send_once_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.send_once().await;
        }
    })
}

pub(crate) fn send_once_body_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #context.send_once_body().await;
        }
    })
}
