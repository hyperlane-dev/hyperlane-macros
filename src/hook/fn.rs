use crate::*;

pub(crate) fn pre_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    let _ = #function_name(#context.clone()).await;
                    #(#stmts)*
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn post_hook_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name: Ident = parse_macro_input!(attr as Ident);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    #(#stmts)*
                    let _ = #function_name(#context.clone()).await;
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
