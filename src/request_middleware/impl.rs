use crate::*;

/// Parses the attributes for the `request_middleware` macro.
///
/// This implementation of the `Parse` trait allows `syn` to parse
/// the priority from the macro's attribute tokens. If no priority
/// is provided, it defaults to 0.
impl Parse for RequestMiddlewareAttr {
    /// Parses the input stream into a `RequestMiddlewareAttr` struct.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `RequestMiddlewareAttr` or an error.
    fn parse(input: ParseStream) -> Result<Self> {
        let priority: Expr = if input.is_empty() {
            syn::parse_str("0")?
        } else {
            input.parse()?
        };
        Ok(RequestMiddlewareAttr { priority })
    }
}
