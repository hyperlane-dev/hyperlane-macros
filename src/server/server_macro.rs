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
pub(crate) fn hyperlane_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let variable_name: Ident = match parse_variable_name(attr) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error().into(),
    };
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    let stmts: &Vec<Stmt> = &block.stmts;
    let gen_code: TokenStream2 = quote! {
        #(#attrs)*
        #vis #sig {
            let #variable_name: Server = Server::new();
            #(#stmts)*
        }
    };
    gen_code.into()
}

/// Parses the server variable name from attribute tokens.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to parse.
///
/// # Returns
///
/// - `syn::Result<Ident>` - The parsed identifier or error.
fn parse_variable_name(attr: TokenStream) -> syn::Result<Ident> {
    let attr_tokens: TokenStream2 = attr.into();
    if attr_tokens.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected variable name as argument",
        ));
    }
    let parsed: Ident = syn::parse2(attr_tokens)?;
    Ok(parsed)
}
