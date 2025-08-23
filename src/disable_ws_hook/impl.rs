use crate::*;

/// Implements the `Parse` trait for `DisableWsHookAttr`.
///
/// This implementation defines how to parse a `TokenStream` into a `DisableWsHookAttr` struct,
/// extracting the path expression from the input.
impl Parse for DisableWsHookAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let path_expr: Expr = input.parse()?;
        Ok(DisableWsHookAttr { path: path_expr })
    }
}
