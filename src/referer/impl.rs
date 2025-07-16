use crate::*;

impl Parse for RefererData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let referer_value: Expr = input.parse()?;
        Ok(RefererData { referer_value })
    }
}
