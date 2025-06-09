use crate::*;

pub(crate) fn expand_method(
    method_name: &str,
    method_ident: &Ident,
    item: TokenStream,
) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input.vis;
    let sig: &Signature = &input.sig;
    let block: &Box<Block> = &input.block;
    let attrs: &Vec<Attribute> = &input.attrs;
    let context: &Ident = match sig.inputs.first() {
        Some(FnArg::Typed(pat_type)) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            Pat::Wild(wild) => {
                return syn::Error::new_spanned(
                    wild,
                    "The argument cannot be anonymous `_`, please use a named identifier",
                )
                .to_compile_error()
                .into();
            }
            _ => {
                return syn::Error::new_spanned(
                    &pat_type.pat,
                    "expected identifier as first argument",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&sig.inputs, "expected at least one argument")
                .to_compile_error()
                .into();
        }
    };
    let check_method: Ident = Ident::new(&format!("is_{}", method_name), method_ident.span());
    let gen_code: proc_macro2::TokenStream = quote! {
        #(#attrs)*
        #vis #sig {
            if !#context.get_request().await.#check_method() {
                return;
            }
            #block
        }
    };
    gen_code.into()
}

pub(crate) fn get_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "get",
        &Ident::new("get", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn post_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "post",
        &Ident::new("post", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn put_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "put",
        &Ident::new("put", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn delete_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "delete",
        &Ident::new("delete", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn patch_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "patch",
        &Ident::new("patch", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn head_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "head",
        &Ident::new("head", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn options_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "options",
        &Ident::new("options", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn connect_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "connect",
        &Ident::new("connect", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn trace_macro(item: TokenStream) -> TokenStream {
    expand_method(
        "trace",
        &Ident::new("trace", proc_macro2::Span::call_site()),
        item,
    )
}

pub(crate) fn methods_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let methods: RequestMethods = parse_macro_input!(attr as RequestMethods);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Box<Block> = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    let context: &Ident = match sig.inputs.first() {
        Some(FnArg::Typed(pat_type)) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            Pat::Wild(wild) => {
                return syn::Error::new_spanned(
                    wild,
                    "The argument cannot be anonymous `_`, please use a named identifier",
                )
                .to_compile_error()
                .into();
            }
            _ => {
                return syn::Error::new_spanned(
                    &pat_type.pat,
                    "Expected identifier as first argument",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&sig.inputs, "Expected at least one argument")
                .to_compile_error()
                .into();
        }
    };
    let method_checks = methods.methods.iter().map(|method| {
        let check_fn = Ident::new(&format!("is_{}", method), method.span());
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
