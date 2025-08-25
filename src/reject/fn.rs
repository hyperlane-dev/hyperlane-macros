use crate::*;

/// Rejects requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `false`.
///
/// # Arguments
///
/// - `attr`: A code block that returns a boolean value.
/// - `item`: The function to which the attribute is applied.
///
/// # Returns
///
/// - `TokenStream` - The modified function wrapped with a conditional check;
///   if the condition evaluates to `true`, the function returns early,
///   otherwise the original function body is executed.
pub(crate) fn reject_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let condition: Expr = parse_macro_input!(attr as Expr);
    inject(position, item, |_| {
        quote! {
            if #condition {
                return;
            }
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "reject",
        handler: Handler::WithAttrPosition(reject_macro),
    }
}
