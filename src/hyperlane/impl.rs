use crate::*;

/// Implementation of the `Parse` trait for `HyperlaneAttr`.
///
/// This implementation allows parsing a `HyperlaneAttr` from a token stream,
/// expecting the format `variable_name: TypeName`.
///
/// # Arguments
///
/// - `ParseStream` - The `ParseStream` to parse from.
///
/// # Returns
///
/// A `syn::Result` containing the parsed `HyperlaneAttr` or an error.
impl Parse for HyperlaneAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(HyperlaneAttr {
            var_name: input.parse()?,
            _colon: input.parse()?,
            type_name: input.parse()?,
        })
    }
}
