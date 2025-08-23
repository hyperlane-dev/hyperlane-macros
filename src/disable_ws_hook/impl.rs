use crate::*;

impl Parse for DisableWsHookAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let path_expr: Expr = input.parse()?;
        Ok(DisableWsHookAttr { path: path_expr })
    }
}
