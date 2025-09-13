use crate::*;

/// Implementation of Parse trait for WsFromStreamData.
///
/// Parses buffer and variable name from input stream for WebSocket stream processing.
impl Parse for WsFromStreamData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut buffer: Option<Expr> = None;
        let mut variable_name: Option<Expr> = None;
        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "at least one parameter (buffer size or variable name) must be provided",
            ));
        }
        let first_expr: Expr = input.parse()?;
        if input.is_empty() {
            if is_integer_literal(&first_expr) {
                buffer = Some(first_expr);
            } else {
                variable_name = Some(first_expr);
            }
        } else {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                return Err(syn::Error::new(
                    input.span(),
                    "expected second parameter after comma",
                ));
            }
            let second_expr: Expr = input.parse()?;
            let first_is_int: bool = is_integer_literal(&first_expr);
            let second_is_int: bool = is_integer_literal(&second_expr);
            match (first_is_int, second_is_int) {
                (true, true) => {
                    return Err(syn::Error::new_spanned(
                        &second_expr,
                        "cannot have two buffer size parameters",
                    ));
                }
                (false, false) => {
                    return Err(syn::Error::new_spanned(
                        &second_expr,
                        "cannot have two variable name parameters",
                    ));
                }
                (true, false) => {
                    buffer = Some(first_expr);
                    variable_name = Some(second_expr);
                }
                (false, true) => {
                    variable_name = Some(first_expr);
                    buffer = Some(second_expr);
                }
            }
        }
        if !input.is_empty() {
            return Err(syn::Error::new_spanned(
                input.parse::<TokenStream2>()?,
                "unexpected additional tokens in attribute",
            ));
        }
        Ok(WsFromStreamData {
            buffer,
            variable_name,
        })
    }
}
