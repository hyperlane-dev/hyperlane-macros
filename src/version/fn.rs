use crate::*;

/// Creates a version check function for HTTP version validation.
///
/// # Arguments
///
/// - `&proc_macro2::Ident` - The HTTP version identifier as an ident.
///
/// # Returns
///
/// Returns a closure that generates the version check code.
pub(crate) fn create_version_check(
    version: &proc_macro2::Ident,
) -> impl FnOnce(&Ident) -> TokenStream2 {
    let version_str: String = version.to_string();
    move |context| {
        let check_fn: proc_macro2::Ident = Ident::new(&format!("is_{version_str}"), context.span());
        quote! {
            if !#context.get_request().get_version().#check_fn() {
                return;
            }
        }
    }
}

/// Implements a version check macro.
///
/// This macro generates a handler function for a specific HTTP version check.
/// It expands to a check that aborts the request if the HTTP version does not match.
///
/// # Arguments
///
/// - `$name` - The name of the generated handler function.
/// - `$submit_name` - The string name for macro registration as an ident.
/// - `$version` - The HTTP version identifier as an ident.
macro_rules! impl_version_check_macro {
    ($name:ident, $submit_name:ident, $version:ident) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(
                position,
                item,
                create_version_check(&proc_macro2::Ident::new(
                    stringify!($version),
                    proc_macro2::Span::call_site(),
                )),
            )
        }
        inventory::submit! {
            InjectableMacro {
                name: stringify!($submit_name),
                handler: Handler::NoAttrPosition($name),
            }
        }
    };
}

impl_version_check_macro!(http0_9_version_macro, http0_9_version, http0_9);
impl_version_check_macro!(http1_0_version_macro, http1_0_version, http1_0);
impl_version_check_macro!(http1_1_version_macro, http1_1_version, http1_1);
impl_version_check_macro!(http2_version_macro, http2_version, http2);
impl_version_check_macro!(http3_version_macro, http3_version, http3);
impl_version_check_macro!(
    http1_1_or_higher_version_macro,
    http1_1_or_higher_version,
    http1_1_or_higher
);
impl_version_check_macro!(http_version_macro, http_version, http);
impl_version_check_macro!(unknown_version_macro, unknown_version, unknown);
