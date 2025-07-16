use crate::*;

impl Parse for HostData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let host_value: Expr = input.parse()?;
        Ok(HostData { host_value })
    }
}
