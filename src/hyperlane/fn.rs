use crate::*;

/// Main macro for creating and configuring a Hyperlane server instance.
/// Supports both single and multiple variable-type pair initialization.
///
/// This macro expects an attribute in the format `#[hyperlane(variable_name: TypeName)]`
/// or `#[hyperlane(var1: Type1, var2: Type2, ...)]`.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, containing the variable and type name.
/// - `TokenStream` - The input token stream to process, typically an `async fn`.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with server initialization.
pub(crate) fn hyperlane_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let multi_hyperlane: MultiHyperlaneAttr = parse_macro_input!(attr as MultiHyperlaneAttr);
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

    for (var_name, type_name) in &multi_hyperlane.params {
        init_statements.push(quote! {
            let #var_name: #type_name = #type_name::new().await;
        });
        if type_name == SERVER_TYPE_KEY {
            init_statements.push(quote! {
                let mut hooks: Vec<::hyperlane::HookMacro> = inventory::iter().cloned().collect();
                assert_hook_unique_order(hooks.clone());
                hooks.sort_by_key(|hook| hook.hook_type.try_get());
                for hook in hooks {
                    #var_name.handle_hook(hook.clone()).await;
                }
            });
        }
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

inventory::submit! {
    InjectableMacro {
        name: "hyperlane",
        handler: Handler::WithAttr(hyperlane_macro),
    }
}
