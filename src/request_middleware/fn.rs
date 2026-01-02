use crate::*;

/// Registers a request middleware.
///
/// This macro takes a struct as input and registers it as a request middleware.
/// The registered struct will be used to create handlers that are called before the main request handler.
///
/// # Arguments
///
/// - `TokenStream` - The attribute `TokenStream`, which can optionally specify an `order`.
/// - `TokenStream` - The input token stream representing the struct to be registered as a middleware.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the middleware registration.
pub(crate) fn request_middleware_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_struct: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_name: &Ident = &input_struct.ident;
    let gen_code: TokenStream2 = quote! {
        #input_struct
        ::hyperlane::inventory::submit! {
            ::hyperlane::HookType::RequestMiddleware(#order, || ::hyperlane::server_hook_factory::<#struct_name>())
        }
    };
    gen_code.into()
}

inventory::submit! {
    InjectableMacro {
        name: "request_middleware",
        handler: Handler::WithAttr(request_middleware_macro),
    }
}
