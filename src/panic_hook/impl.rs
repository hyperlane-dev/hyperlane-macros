use crate::*;

impl Parse for PanicHookAttr {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(PanicHookAttr)
    }
}