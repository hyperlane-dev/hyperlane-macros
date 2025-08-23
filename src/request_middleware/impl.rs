use crate::*;

impl Parse for RequestMiddlewareAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(RequestMiddlewareAttr { path: None })
        } else {
            let path_expr: Expr = input.parse()?;
            Ok(RequestMiddlewareAttr {
                path: Some(path_expr),
            })
        }
    }
}
