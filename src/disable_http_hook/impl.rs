use crate::*;

impl Parse for DisableHttpHookAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let path_expr: Expr = input.parse()?;
        Ok(DisableHttpHookAttr { path: path_expr })
    }
}
