use crate::*;

/// Internal implementation for the `route` attribute macro.
///
/// This function processes the route attribute and generates code to register
/// the decorated function as a route handler in the inventory system.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream containing route parameters (path and optional server)
/// - `TokenStream` - The function token stream being decorated
///
/// # Returns
///
/// A `TokenStream` containing the original function and inventory registration code
///
/// # Generated Code
///
/// The macro generates:
/// - The original function unchanged
/// - An `inventory::submit!` block that registers a `HookMacro` instance
/// - A handler closure that wraps the function in `Box::pin` for async execution
pub(crate) fn route_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_attr: RouteAttr = parse_macro_input!(attr as RouteAttr);
    let path: &Expr = &route_attr.path;
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::Route(#path),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
