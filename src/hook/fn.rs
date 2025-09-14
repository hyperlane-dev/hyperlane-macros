use crate::*;

/// Registers a panic hook.
///
/// This macro takes a function as input and registers it as a panic hook.
/// The registered function will be called when a panic occurs within the application.
///
/// # Arguments
///
/// - `TokenStream` - The attribute `TokenStream`, which can optionally specify an `order`.
/// - `TokenStream` - The input `TokenStream` representing the function to be registered as a hook.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the hook registration.
pub(crate) fn panic_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            ::hyperlane::HookMacro {
                hook_type: ::hyperlane::HookType::PanicHook(#order),
                handler: |ctx: ::hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}

inventory::submit! {
    InjectableMacro {
        name: "panic_hook",
        handler: Handler::WithAttr(panic_hook_macro),
    }
}

/// Expands macro to add pre-hook function call.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with pre-hook call.
pub(crate) fn prologue_hook_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    inject(position, item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "prologue_hook",
        handler: Handler::WithAttrPosition(prologue_hook_macro),
    }
}

/// Expands macro to add post-hook function call.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with post-hook call.
pub(crate) fn epilogue_hook_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    inject(position, item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "epilogue_hook",
        handler: Handler::WithAttrPosition(epilogue_hook_macro),
    }
}
