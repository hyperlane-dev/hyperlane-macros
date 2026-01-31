use crate::*;

/// Creates a protocol check function for protocol upgrade type validation.
///
/// # Arguments
///
/// - `&proc_macro2::Ident` - The protocol upgrade type identifier as an ident.
/// - `proc_macro2::Span` - The span for error reporting.
///
/// # Returns
///
/// Returns a closure that generates the protocol check code.
pub(crate) fn create_protocol_check(
    upgrade_type: &proc_macro2::Ident,
    span: proc_macro2::Span,
) -> impl FnOnce(&Ident) -> TokenStream2 {
    let check_upgrade: Ident =
        Ident::new(&format!("get_request_is_{upgrade_type}_upgrade_type"), span);
    move |context| {
        quote! {
            if !#context.#check_upgrade().await {
                return;
            }
        }
    }
}

/// Implements a protocol upgrade type check macro.
///
/// This macro generates a handler function for a specific protocol upgrade type check.
/// It expands to a check that aborts the request if the upgrade type does not match.
///
/// # Arguments
///
/// - `$name` - The name of the generated handler function.
/// - `$submit_name` - The string name for macro registration as an ident.
/// - `$upgrade_type` - The protocol upgrade type identifier as an ident.
macro_rules! impl_protocol_check_macro {
    ($name:ident, $submit_name:ident, $upgrade_type:ident) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(
                position,
                item,
                create_protocol_check(
                    &proc_macro2::Ident::new(
                        stringify!($upgrade_type),
                        proc_macro2::Span::call_site(),
                    ),
                    proc_macro2::Span::call_site(),
                ),
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

impl_protocol_check_macro!(ws_upgrade_type_macro, ws_upgrade_type, ws);
impl_protocol_check_macro!(h2c_upgrade_type_macro, h2c_upgrade_type, h2c);
impl_protocol_check_macro!(tls_upgrade_type_macro, tls_upgrade_type, tls);
impl_protocol_check_macro!(unknown_upgrade_type_macro, unknown_upgrade_type, unknown);
