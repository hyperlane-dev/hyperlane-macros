use crate::*;

/// Disables the default WebSocket hook for a specific path.
///
/// This macro takes a function as input and registers it to disable the default
/// WebSocket hook for requests matching the specified path.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, containing the path to disable the hook for and an optional order.
/// - `TokenStream` - The input token stream representing the function to be registered.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook disabling registration.
pub(crate) fn disable_ws_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: PathAndOrderAttr = parse_macro_input!(attr as PathAndOrderAttr);
    let path: &Expr = &attr_args.path;
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::DisableWsHook(#path, #order),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
