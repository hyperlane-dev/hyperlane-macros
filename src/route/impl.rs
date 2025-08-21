use crate::*;

impl Parse for RouteAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RouteAttr {
            path: input.parse()?,
        })
    }
}
