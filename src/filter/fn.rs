use crate::*;

/// Filters requests with unknown method, upgrade and version.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with filter checks.
pub(crate) fn filter_unknown_macro(item: TokenStream) -> TokenStream {
    expand_macro_with_before_insertion(item, |context| {
        quote! {
            if !#context.get_request().await.is_unknown_method() {
                return;
            }
            if !#context.get_request().await.is_unknown_upgrade() {
                return;
            }
            if !#context.get_request().await.is_unknown_version() {
                return;
            }
        }
    })
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
