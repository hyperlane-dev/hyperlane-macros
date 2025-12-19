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
    let input_struct: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_name: &Ident = &input_struct.ident;
    let gen_code: TokenStream2 = quote! {
        #input_struct
        ::hyperlane::inventory::submit! {
            ::hyperlane::HookMacro {
                hook_type: ::hyperlane::HookType::Route(#path),
                handler: ::hyperlane::HookHandler::Factory(|| ::hyperlane::server_hook_factory::<#struct_name>()),
            }
        }
    };
    gen_code.into()
}

inventory::submit! {
    InjectableMacro {
        name: "route",
        handler: Handler::WithAttr(route_macro),
    }
}
