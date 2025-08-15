use crate::*;

/// Main macro for creating and configuring a Hyperlane server instance.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with server initialization.
pub(crate) fn server_config_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let variable_name: Ident = match parse_variable_name(attr) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error().into(),
    };
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let ident: &Ident = &sig.ident;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    let stmts: &Vec<Stmt> = &block.stmts;
    let inputs: &Punctuated<FnArg, token::Comma> = &sig.inputs;
    let output: &ReturnType = &sig.output;
    let gen_code: TokenStream2 = quote! {
        #(#attrs)*
        #vis async fn #ident(#inputs) #output {
            let #variable_name: ServerConfig = ServerConfig::new().await;
            #(#stmts)*
        }
    };
    gen_code.into()
}
