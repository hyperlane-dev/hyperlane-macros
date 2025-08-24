use crate::*;

/// Registers a connected hook.
///
/// This macro takes a function as input and registers it as a connected hook.
/// The registered function will be called when a connection is established.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, which can optionally specify an `order`.
/// - `TokenStream` - The input token stream representing the function to be registered as a hook.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook registration.
pub(crate) fn connected_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::ConnectedHook(#order),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}

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
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook registration.
pub(crate) fn pre_upgrade_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
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

/// Registers a panic hook.
///
/// This macro takes a function as input and registers it as a panic hook.
/// The registered function will be called when a panic occurs within the application.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, which can optionally specify an `order`.
/// - `TokenStream` - The input token stream representing the function to be registered as a hook.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook registration.
pub(crate) fn panic_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
    let order: TokenStream2 = expr_to_isize(&attr_args.order);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            hyperlane::HookMacro {
                hook_type: hyperlane::HookType::PanicHook(#order),
                handler: |ctx: hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}

/// Disables the default HTTP hook for a specific path.
///
/// This macro takes a function as input and registers it to disable the default
/// HTTP hook for requests matching the specified path.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, containing the path to disable the hook for and an optional order.
/// - `TokenStream` - The input token stream representing the function to be registered.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook disabling registration.
pub(crate) fn disable_http_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: PathAttr = parse_macro_input!(attr as PathAttr);
    let path: &Expr = &attr_args.path;
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

/// Disables the default WebSocket hook for a specific path.
///
/// This macro takes a function as input and registers it to disable the default
/// WebSocket hook for requests matching the specified path.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, containing the path to disable the hook for and an optional order.
/// - `TokenStream` - The input token stream representing the function to be registered.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the hook disabling registration.
pub(crate) fn disable_ws_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: PathAttr = parse_macro_input!(attr as PathAttr);
    let path: &Expr = &attr_args.path;
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &Ident = &input_fn.sig.ident;
    let gen_code: TokenStream2 = quote! {
        #input_fn
        inventory::submit! {
            ::hyperlane::HookMacro {
                hook_type: ::hyperlane::HookType::DisableWsHook(#path),
                handler: |ctx: ::hyperlane::Context| Box::pin(#fn_name(ctx)),
            }
        }
    };
    gen_code.into()
}

/// Expands macro to add pre-hook function call.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with pre-hook call.
pub(crate) fn pre_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    inject_at_start(item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}

/// Expands macro to add post-hook function call.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with post-hook call.
pub(crate) fn post_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    inject_at_end(item, |context| {
        quote! {
            let _ = #function_name(#context.clone()).await;
        }
    })
}
