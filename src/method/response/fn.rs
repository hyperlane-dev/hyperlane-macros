use crate::*;

fn parse_literal_str(input: TokenStream) -> syn::Result<String> {
    let lit = syn::parse::<LitStr>(input)?;
    Ok(lit.value())
}

fn parse_literal_int(input: TokenStream) -> syn::Result<usize> {
    let lit = syn::parse::<LitInt>(input)?;
    lit.base10_parse()
}

fn expand_response_setting_macro<T>(
    attr: TokenStream,
    item: TokenStream,
    parse_attr: impl FnOnce(TokenStream) -> syn::Result<T>,
    generate_setter: impl FnOnce(&Ident, T) -> TokenStream2,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;

    match parse_context_from_fn(sig) {
        Ok(context) => match parse_attr(attr) {
            Ok(value) => {
                let setter: TokenStream2 = generate_setter(context, value);
                let stmts: &Vec<Stmt> = &block.stmts;
                let gen_code: TokenStream2 = quote! {
                    #(#attrs)*
                    #vis #sig {
                        #setter
                        #(#stmts)*
                    }
                };
                gen_code.into()
            }
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

macro_rules! impl_response_setting_macro {
    ($name:ident, $parse_fn:ident, $setter_fn:ident) => {
        pub(crate) fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            expand_response_setting_macro(attr, item, $parse_fn, |context, value| {
                let setter = Ident::new(stringify!($setter_fn), proc_macro2::Span::call_site());
                quote! {
                    #context.#setter(#value).await;
                }
            })
        }
    };
    ($name:ident, $parse_fn:ident, $setter_fn:ident, $value_type:ident) => {
        pub(crate) fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            expand_response_setting_macro(attr, item, $parse_fn, |context, value| {
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
    code_macro,
    parse_literal_int,
    set_response_status_code,
    ResponseStatusCode
);
impl_response_setting_macro!(
    reason_phrase_macro,
    parse_literal_str,
    set_response_reason_phrase
);
