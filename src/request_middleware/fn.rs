use crate::*;

pub(crate) fn request_middleware_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: RequestMiddlewareAttr = parse_macro_input!(attr as RequestMiddlewareAttr);
    let path = match attr.path {
        Some(p) => quote! { Some(#p) },
        None => quote! { None },
    };
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::RequestMiddleware,
                path: #path,
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
