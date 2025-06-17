use crate::*;

pub(crate) fn create_method_check(
    method_name: &str,
    span: proc_macro2::Span,
) -> impl FnOnce(&Ident) -> proc_macro2::TokenStream {
    let check_method = Ident::new(&format!("is_{}", method_name), span);
    move |context| {
        quote! {
            if !#context.get_request().await.#check_method() {
                return;
            }
        }
    }
}

pub(crate) fn expand_check_macro(
    input: TokenStream,
    check_fn: impl FnOnce(&Ident) -> proc_macro2::TokenStream,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Box<Block> = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => {
            let check_expr: proc_macro2::TokenStream = check_fn(context);
            let gen_code: proc_macro2::TokenStream = quote! {
                #(#attrs)*
                #vis #sig {
                    #check_expr
                    #block
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn parse_context_from_fn(sig: &Signature) -> syn::Result<&Ident> {
    match sig.inputs.first() {
        Some(FnArg::Typed(pat_type)) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => Ok(&pat_ident.ident),
            Pat::Wild(wild) => Err(syn::Error::new_spanned(
                wild,
                "The argument cannot be anonymous `_`, please use a named identifier",
            )),
            _ => Err(syn::Error::new_spanned(
                &pat_type.pat,
                "expected identifier as first argument",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            &sig.inputs,
            "expected at least one argument",
        )),
    }
}

pub(crate) fn methods_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let methods: RequestMethods = parse_macro_input!(attr as RequestMethods);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Box<Block> = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => {
            let method_checks = methods.methods.iter().map(|method| {
                let check_fn: Ident = Ident::new(&format!("is_{}", method), method.span());
                quote! {
                    #context.get_request().await.#check_fn()
                }
            });
            let check_expr: proc_macro2::TokenStream = quote! {
                if !(#(#method_checks)||*) {
                    return;
                }
            };
            let gen_code: proc_macro2::TokenStream = quote! {
                #(#attrs)*
                #vis #sig {
                    #check_expr
                    #block
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn ws_macro(item: TokenStream) -> TokenStream {
    expand_check_macro(item, |context| {
        quote! {
            if !#context.get_request().await.is_ws() {
                return;
            }
        }
    })
}

pub(crate) fn http_macro(item: TokenStream) -> TokenStream {
    expand_check_macro(item, |context| {
        quote! {
            if !#context.get_request().await.is_http() {
                return;
            }
        }
    })
}

fn parse_literal_str(input: TokenStream) -> syn::Result<String> {
    let lit = syn::parse::<LitStr>(input)?;
    Ok(lit.value())
}

fn parse_literal_int(input: TokenStream) -> syn::Result<u16> {
    let lit = syn::parse::<LitInt>(input)?;
    lit.base10_parse()
}

fn expand_response_setting_macro<T>(
    attr: TokenStream,
    item: TokenStream,
    parse_attr: impl FnOnce(TokenStream) -> syn::Result<T>,
    generate_setter: impl FnOnce(&Ident, T) -> proc_macro2::TokenStream,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Box<Block> = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => match parse_attr(attr) {
            Ok(value) => {
                let setter: proc_macro2::TokenStream = generate_setter(context, value);
                let gen_code: proc_macro2::TokenStream = quote! {
                    #(#attrs)*
                    #vis #sig {
                        #setter
                        #block
                    }
                };
                gen_code.into()
            }
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn code_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_response_setting_macro(attr, item, parse_literal_int, |context, code| {
        quote! {
            #context.set_response_status_code(hyperlane::ResponseStatusCode::from(#code)).await;
        }
    })
}

pub(crate) fn reason_phrase_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_response_setting_macro(attr, item, parse_literal_str, |context, phrase| {
        quote! {
            #context.set_response_reason_phrase(#phrase).await;
        }
    })
}

fn expand_send_macro(
    input: TokenStream,
    send_fn: impl FnOnce(&Ident) -> proc_macro2::TokenStream,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Box<Block> = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => {
            let send_expr: proc_macro2::TokenStream = send_fn(context);
            let gen_code: proc_macro2::TokenStream = quote! {
                #(#attrs)*
                #vis #sig {
                    #block
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

pub(crate) fn filter_unknown_macro(item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Box<Block> = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => {
            let gen_code: proc_macro2::TokenStream = quote! {
                #(#attrs)*
                #vis #sig {
                    if !#context.get_request().await.is_unknown_method() {
                        return;
                    }
                    if !#context.get_request().await.is_unknown_upgrade() {
                        return;
                    }
                    if !#context.get_request().await.is_unknown_version() {
                        return;
                    }
                    #block
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
