use crate::*;

/// Implementation of Parse trait for HostData.
///
/// Parses host value expression from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<HostData>` - Parsed HostData or error.
impl Parse for HostData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let host_value: Expr = input.parse()?;
        Ok(HostData { host_value })
    }
}
