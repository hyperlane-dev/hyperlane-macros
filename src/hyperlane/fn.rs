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
            for route in inventory::iter::<hyperlane::RouteMacro> {
                #var_name.route(&route.path, route.handler).await;
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
