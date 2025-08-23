use crate::*;

impl Parse for ConnectedHookAttr {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(ConnectedHookAttr)
    }
}
