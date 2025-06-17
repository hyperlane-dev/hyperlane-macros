use crate::*;

fn expand_send_macro(
    input: TokenStream,
    send_fn: impl FnOnce(&Ident) -> TokenStream2,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => {
            let send_expr: TokenStream2 = send_fn(context);
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    #(#stmts)*
                    let _ = #send_expr;
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn send_macro(item: TokenStream) -> TokenStream {
    expand_send_macro(item, |context| {
        quote! {
            #context.send().await
        }
    })
}

pub(crate) fn send_body_macro(item: TokenStream) -> TokenStream {
    expand_send_macro(item, |context| {
        quote! {
            #context.send_body().await
        }
    })
}

pub(crate) fn send_once_macro(item: TokenStream) -> TokenStream {
    expand_send_macro(item, |context| {
        quote! {
            #context.send_once().await
        }
    })
}

pub(crate) fn send_once_body_macro(item: TokenStream) -> TokenStream {
    expand_send_macro(item, |context| {
        quote! {
            #context.send_once_body().await
        }
    })
}
