use crate::*;

/// Implements the `Parse` trait for `DisableHttpHookAttr`.
///
/// This implementation defines how to parse a `TokenStream` into a `DisableHttpHookAttr` struct,
/// extracting the path expression from the input.
impl Parse for DisableHttpHookAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let path_expr: Expr = input.parse()?;
        Ok(DisableHttpHookAttr { path: path_expr })
    }
}
