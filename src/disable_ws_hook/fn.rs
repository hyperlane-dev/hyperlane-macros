use crate::*;

pub(crate) fn disable_ws_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: DisableWsHookAttr = parse_macro_input!(attr as DisableWsHookAttr);
    let path: &Expr = &attr.path;
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::DisableWsHook(#path),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
