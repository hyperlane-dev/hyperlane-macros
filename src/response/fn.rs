use crate::*;

fn parse_literal_str(input: TokenStream) -> syn::Result<String> {
    let lit: LitStr = syn::parse::<LitStr>(input)?;
    Ok(lit.value())
}

fn parse_literal_int(input: TokenStream) -> syn::Result<usize> {
    let lit: LitInt = syn::parse::<LitInt>(input)?;
    lit.base10_parse()
}

macro_rules! impl_response_setting_macro {
    ($name:ident, $parse_fn:ident, $setter_fn:ident) => {
        pub(crate) fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            expand_macro_with_attr_and_before_insertion(attr, item, $parse_fn, |context, value| {
                let setter = Ident::new(stringify!($setter_fn), proc_macro2::Span::call_site());
                quote! {
                    #context.#setter(#value).await;
                }
            })
        }
    };
    ($name:ident, $parse_fn:ident, $setter_fn:ident, $value_type:ident) => {
        pub(crate) fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            expand_macro_with_attr_and_before_insertion(attr, item, $parse_fn, |context, value| {
                let setter = Ident::new(stringify!($setter_fn), proc_macro2::Span::call_site());
                let value_type =
                    Ident::new(stringify!($value_type), proc_macro2::Span::call_site());
                quote! {
                    #context.#setter(hyperlane::#value_type::from(#value)).await;
                }
            })
        }
    };
}

impl_response_setting_macro!(
    response_status_code_macro,
    parse_literal_int,
    set_response_status_code,
    ResponseStatusCode
);

impl_response_setting_macro!(
    response_reason_phrase_macro,
    parse_literal_str,
    set_response_reason_phrase
);
