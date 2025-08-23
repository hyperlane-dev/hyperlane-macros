use crate::*;

/// Registers a request middleware.
///
/// This macro takes a function as input and registers it as a request middleware.
/// The registered function will be called before the main request handler.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream (unused).
/// - `TokenStream` - The input token stream representing the function to be registered as a middleware.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the middleware registration.
pub(crate) fn request_middleware_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: PathAndOrderAttr = parse_macro_input!(attr as PathAndOrderAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::RequestMiddleware(#order),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
