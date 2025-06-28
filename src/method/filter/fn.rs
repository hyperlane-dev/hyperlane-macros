use crate::*;

pub(crate) fn filter_unknown_macro(item: TokenStream) -> TokenStream {
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
                    if !#context.get_request().await.is_unknown_method() {
                        return;
                    }
                    if !#context.get_request().await.is_unknown_upgrade() {
                        return;
                    }
                    if !#context.get_request().await.is_unknown_version() {
                        return;
                    }
                    #(#stmts)*
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

macro_rules! impl_filter_macro {
    ($name:ident, $check:ident) => {
        pub(crate) fn $name(item: TokenStream) -> TokenStream {
            expand_check_macro(item, |context| {
                let check_fn = Ident::new(stringify!($check), proc_macro2::Span::call_site());
                quote! {
                    if !#context.get_request().await.#check_fn() {
                        return;
                    }
                }
            })
        }
    };
}

impl_filter_macro!(filter_unknown_method_macro, is_unknown_method);
impl_filter_macro!(filter_unknown_upgrade_macro, is_unknown_upgrade);
impl_filter_macro!(filter_unknown_version_macro, is_unknown_version);
