use crate::*;

/// Registers a response middleware.
///
/// This macro takes a function as input and registers it as a response middleware.
/// The registered function will be called after the main request handler but before the response is sent.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream (unused).
/// - `TokenStream` - The input token stream representing the function to be registered as a middleware.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the middleware registration.
pub(crate) fn response_middleware_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::ResponseMiddleware(#order),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}
