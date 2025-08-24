use crate::*;

/// Filters requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `true`.
///
/// # Arguments
///
/// - `attr`: A code block that returns a boolean value.
/// - `item`: The function to which the attribute is applied.
///
/// # Returns
///
/// - `TokenStream` - The modified function wrapped with a conditional guard;
///   the original function body is executed only if the condition is `true`,
///   otherwise the function returns early without doing anything.
pub(crate) fn filter_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let condition: Expr = parse_macro_input!(attr as Expr);
    let mut item_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let stmts: &Vec<Stmt> = &item_fn.block.stmts;
    let new_stmts: TokenStream2 = quote! {
        if !(#condition) {
            return;
        }
        #(#stmts)*
    };
    let body: TokenStream2 = quote!({#new_stmts});
    item_fn.block = syn::parse2(body).unwrap();
    quote!(#item_fn).into()
}

inventory::submit! {
    InjectableMacro {
        name: "filter",
        handler: Handler::WithAttr(filter_macro),
    }
}
