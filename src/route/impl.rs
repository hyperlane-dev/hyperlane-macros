use crate::*;

impl Parse for RouteAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let server: Option<Expr> = if input.peek2(Token![=>]) {
            let server_expr: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            Some(server_expr)
        } else {
            None
        };
        let path: Expr = input.parse()?;
        Ok(RouteAttr { server, path })
    }
}
