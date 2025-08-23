use crate::*;

/// Registers a request middleware.
///
/// This macro takes a function as input and registers it as a request middleware.
/// The registered function will be called before the main request handler.
///
/// # Arguments
///
/// - `_attr` - The attribute token stream (unused).
/// - `item` - The input token stream representing the function to be registered as a middleware.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the middleware registration.
pub(crate) fn request_middleware_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::RequestMiddleware,
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
