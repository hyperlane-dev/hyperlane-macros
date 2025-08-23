use crate::*;

/// Disables the default HTTP hook for a specific path.
///
/// This macro takes a function as input and registers it to disable the default
/// HTTP hook for requests matching the specified path.
///
/// # Arguments
///
/// - `attr` - The attribute token stream, containing the path to disable the hook for.
/// - `item` - The input token stream representing the function to be registered.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook disabling registration.
pub(crate) fn disable_http_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: DisableHttpHookAttr = parse_macro_input!(attr as DisableHttpHookAttr);
    let path: &Expr = &attr.path;
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::DisableHttpHook(#path),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
