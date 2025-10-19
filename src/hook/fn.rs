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
    let input_struct: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_name: &Ident = &input_struct.ident;
    let factory_fn_name: Ident =
        generate_factory_fn_name("__panic_hook_factory", struct_name, &attr_args.order);
    let gen_code: TokenStream2 = quote! {
        #input_struct
        #[inline]
        #[allow(non_snake_case)]
        fn #factory_fn_name() -> ::hyperlane::ServerHookHandler {
            ::std::sync::Arc::new(|ctx: &::hyperlane::Context| {
                ::std::boxed::Box::pin(#struct_name(ctx.clone()))
            })
        }
        inventory::submit! {
            ::hyperlane::HookMacro {
                hook_type: ::hyperlane::HookType::PanicHook(#order),
                handler: ::hyperlane::HookHandler::Factory(#factory_fn_name),
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

/// Expands macro to add multiple pre-hook function calls.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream containing a list of function names.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with multiple pre-hook calls.
pub(crate) fn prologue_hooks_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let functions: Punctuated<Ident, Token![,]> =
        parse_macro_input!(attr with Punctuated::parse_terminated);
    inject(position, item, |context| {
        let hook_calls = functions.iter().map(|function_name| {
            quote! {
                let _ = #function_name(#context.clone()).await;
            }
        });
        quote! {
            #(#hook_calls)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "prologue_hooks",
        handler: Handler::WithAttrPosition(prologue_hooks_macro),
    }
}

/// Expands macro to add multiple post-hook function calls.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream containing a list of function names.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with multiple post-hook calls.
pub(crate) fn epilogue_hooks_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let functions: Punctuated<Ident, Token![,]> =
        parse_macro_input!(attr with Punctuated::parse_terminated);
    inject(position, item, |context| {
        let hook_calls = functions.iter().map(|function_name| {
            quote! {
                let _ = #function_name(#context.clone()).await;
            }
        });
        quote! {
            #(#hook_calls)*
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "epilogue_hooks",
        handler: Handler::WithAttrPosition(epilogue_hooks_macro),
    }
}
