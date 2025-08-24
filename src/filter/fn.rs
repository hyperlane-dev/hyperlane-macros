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
    inject_at_start(item, |context| {
        quote! {
            {
                let request: ::hyperlane::Request = #context.get_request().await;
                if !request.is_unknown_method() || !request.is_unknown_upgrade() || !request.is_unknown_version() {
                    return;
                }
            }
        }
    })
}

/// Implements a filter macro.
///
/// This macro generates a function that filters requests based on a specific check.
/// If the check fails, the request is aborted.
///
/// # Arguments
///
/// - `$name`: The name of the generated macro function.
/// - `$check`: The name of the method to call on the request to perform the filter check (e.g., `is_unknown_method`).
macro_rules! impl_filter_macro {
    ($name:ident, $check:ident) => {
        pub(crate) fn $name(item: TokenStream) -> TokenStream {
            inject_at_start(item, |context| {
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

// Filters requests with an unknown method.
impl_filter_macro!(filter_unknown_method_macro, is_unknown_method);

// Filters requests with an unknown upgrade.
impl_filter_macro!(filter_unknown_upgrade_macro, is_unknown_upgrade);

// Filters requests with an unknown version.
impl_filter_macro!(filter_unknown_version_macro, is_unknown_version);
