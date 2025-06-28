use crate::*;

pub(crate) fn pre_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}

pub(crate) fn post_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    expand_macro_with_after_insertion(item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}
