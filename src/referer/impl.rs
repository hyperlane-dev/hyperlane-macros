use crate::*;

/// Implementation of Parse trait for RefererData.
///
/// Parses referer value expression from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RefererData>` - Parsed RefererData or error.
impl Parse for RefererData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let referer_value: Expr = input.parse()?;
        Ok(RefererData { referer_value })
    }
}
