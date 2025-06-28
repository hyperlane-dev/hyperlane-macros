use crate::*;

pub(crate) fn body_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_params: BodyParams = parse_macro_input!(attr as BodyParams);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let variable: &Ident = &body_params.variable;
            let type_name: &Type = &body_params.type_name;
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    let #variable: Result<#type_name, JsonError> = #context.get_request_body_json::<#type_name>().await;
                    #(#stmts)*
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
