use crate::*;

/// Parses the attributes for the hook macro.
///
/// This implementation of the `Parse` trait allows `syn` to parse
/// the order from the macro's attribute tokens. If no order
/// is provided, it defaults to 0.
impl Parse for HookAttr {
    /// Parses the input stream into a `HookAttr` struct.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `HookAttr` or an error.
    fn parse(input: ParseStream) -> Result<Self> {
        let order: Expr = if input.is_empty() {
            syn::parse_str("0")?
        } else {
            input.parse()?
        };
        Ok(HookAttr { order })
    }
}

/// Parses the attributes for the `PathAndOrderAttr` macro.
///
/// This implementation of the `Parse` trait allows `syn` to parse
/// a `path` expression and an optional `order` from the macro's
/// attribute tokens. If no order is provided, it defaults to `0`.
impl Parse for PathAndOrderAttr {
    /// Parses the input stream into a `PathAndOrderAttr` struct.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `PathAndOrderAttr` or an error.
    fn parse(input: ParseStream) -> Result<Self> {
        let path: Expr = input.parse()?;
        let order: Expr = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse()?
        } else {
            syn::parse_str("0")?
        };
        Ok(PathAndOrderAttr { path, order })
    }
}
