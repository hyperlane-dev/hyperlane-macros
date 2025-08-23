use crate::*;

/// Parses the attributes for the `PathAttr` macro.
///
/// This implementation of the `Parse` trait allows `syn` to parse
/// a `path` expression from the macro's attribute tokens.
impl Parse for PathAttr {
    /// Parses the input stream into a `PathAttr` struct.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `PathAttr` or an error.
    fn parse(input: ParseStream) -> Result<Self> {
        let path: Expr = input.parse()?;
        Ok(PathAttr { path })
    }
}

/// Parses the attributes for the `OrderAttr` macro.
///
/// This implementation of the `Parse` trait allows `syn` to parse
/// an optional `order` from the macro's attribute tokens.
/// If no order is provided, it defaults to `0`.
impl Parse for OrderAttr {
    /// Parses the input stream into an `OrderAttr` struct.
    ///
    /// # Arguments
    ///
    /// - `input` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `OrderAttr` or an error.
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(OrderAttr { order: None });
        }
        let expr: Expr = input.parse()?;
        Ok(OrderAttr { order: Some(expr) })
    }
}
