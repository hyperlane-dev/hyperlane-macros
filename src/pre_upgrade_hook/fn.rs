use crate::*;

/// Registers a pre-upgrade hook.
///
/// This macro takes a function as input and registers it as a pre-upgrade hook.
/// The registered function will be called before any protocol upgrade (e.g., WebSocket upgrade).
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, which can optionally specify an `order`.
/// - `TokenStream` - The input token stream representing the function to be registered as a hook.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook registration.
pub(crate) fn pre_upgrade_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: HookAttr = parse_macro_input!(attr as HookAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::PreUpgradeHook(#order),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
