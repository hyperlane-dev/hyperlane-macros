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

/// Implementation of Parse trait for MultiRefererData.
///
/// Parses multiple referer value expressions from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRefererData>` - Parsed MultiRefererData or error.
impl Parse for MultiRefererData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut referer_values: Vec<Expr> = Vec::new();
        loop {
            let referer_value: Expr = input.parse()?;
            referer_values.push(referer_value);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRefererData { referer_values })
    }
}
