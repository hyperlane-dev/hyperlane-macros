use crate::*;

/// Main macro for creating and configuring a Hyperlane server instance.
///
/// This macro expects an attribute in the format `#[hyperlane(variable_name: TypeName)]`.
///
/// # Arguments
///
/// - `TokenStream`: The attribute token stream, containing the variable and type name.
/// - `TokenStream`: The input token stream to process, typically an `async fn`.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with server initialization.
pub(crate) fn hyperlane_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let hyperlane_attr: HyperlaneAttr = parse_macro_input!(attr as HyperlaneAttr);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let ident: &Ident = &sig.ident;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    let stmts: &Vec<Stmt> = &block.stmts;
    let inputs: &Punctuated<FnArg, token::Comma> = &sig.inputs;
    let output: &ReturnType = &sig.output;
    let mut init_statements: Vec<TokenStream2> = Vec::new();
    let var_name: &Ident = &hyperlane_attr.var_name;
    let type_name: &Ident = &hyperlane_attr.type_name;
    init_statements.push(quote! {
        let #var_name: #type_name = #type_name::new().await;
    });
    if type_name == "Server" {
        init_statements.push(quote! {
            let mut hooks: Vec<_> = inventory::iter::<hyperlane::HookMacro>.into_iter().collect();
            hooks.sort_by_key(|hook| {
                match hook.hook_type {
                    hyperlane::HookType::RequestMiddleware(order) => order,
                    hyperlane::HookType::ResponseMiddleware(order) => order,
                    hyperlane::HookType::PanicHook(order) => order,
                    hyperlane::HookType::ConnectedHook(order) => order,
                    hyperlane::HookType::PreUpgradeHook(order) => order,
                    _ => 0,
                }
            });
            for hook in hooks {
                match hook.hook_type {
                    hyperlane::HookType::PanicHook(_) => {
                        #var_name.panic_hook(hook.handler).await;
                    },
                    hyperlane::HookType::DisableHttpHook(path) => {
                        #var_name.disable_http_hook(path).await;
                    },
                    hyperlane::HookType::DisableWsHook(path) => {
                        #var_name.disable_ws_hook(path).await;
                    },
                    hyperlane::HookType::ConnectedHook(_) => {
                        #var_name.connected_hook(hook.handler).await;
                    },
                    hyperlane::HookType::PreUpgradeHook(_) => {
                        #var_name.pre_upgrade_hook(hook.handler).await;
                    },
                    hyperlane::HookType::RequestMiddleware(_) => {
                        #var_name.request_middleware(hook.handler).await;
                    },
                    hyperlane::HookType::Route(path) => {
                        #var_name.route(path, hook.handler).await;
                    },
                    hyperlane::HookType::ResponseMiddleware(_) => {
                        #var_name.response_middleware(hook.handler).await;
                    },
                }
            }
        });
    }
    let gen_code: TokenStream2 = quote! {
        #(#attrs)*
        #vis async fn #ident(#inputs) #output {
            #(#init_statements)*
            #(#stmts)*
        }
    };
    gen_code.into()
}
