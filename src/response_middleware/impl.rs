use crate::*;

impl Parse for ResponseMiddlewareAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(ResponseMiddlewareAttr { path: None })
        } else {
            let path_expr: Expr = input.parse()?;
            Ok(ResponseMiddlewareAttr {
                path: Some(path_expr),
            })
        }
    }
}
