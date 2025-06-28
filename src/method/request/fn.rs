use crate::*;

pub(crate) fn body_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let body_params: BodyParams = parse_macro_input!(attr as BodyParams);
    let variable: Ident = body_params.variable;
    let type_name: Type = body_params.type_name;
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            let #variable: Result<#type_name, JsonError> = #context.get_request_body_json::<#type_name>().await;
        }
    })
}
