use crate::*;

fn parse_expr(input: TokenStream) -> syn::Result<Expr> {
    syn::parse::<Expr>(input)
}

pub(crate) fn response_status_code_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_macro_with_attr_and_before_insertion(attr, item, parse_expr, |context, value| {
        quote! {
            #context.set_response_status_code(hyperlane::ResponseStatusCode::from(#value as usize)).await;
        }
    })
}

pub(crate) fn response_reason_phrase_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_macro_with_attr_and_before_insertion(attr, item, parse_expr, |context, value| {
        quote! {
            #context.set_response_reason_phrase(#value).await;
        }
    })
}

pub(crate) fn response_header_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let header_data: ResponseHeaderData = parse_macro_input!(attr as ResponseHeaderData);
    let key: Expr = header_data.key;
    let value: Expr = header_data.value;
    let operation: HeaderOperation = header_data.operation;
    expand_macro_with_before_insertion(item, |context| match operation {
        HeaderOperation::Set => {
            quote! {
                #context.set_response_header(#key, #value).await;
            }
        }
        HeaderOperation::Replace => {
            quote! {
                #context.replace_response_header(#key, #value).await;
            }
        }
    })
}

pub(crate) fn response_body_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_data: ResponseBodyData = parse_macro_input!(attr as ResponseBodyData);
    let body: Expr = body_data.body;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            #context.set_response_body(#body).await;
        }
    })
}

pub(crate) fn response_version_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_macro_with_attr_and_before_insertion(attr, item, parse_expr, |context, value| {
        quote! {
            #context.set_response_version(#value).await;
        }
    })
}
