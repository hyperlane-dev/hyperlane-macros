use crate::*;

impl Parse for PreUpgradeHookAttr {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(PreUpgradeHookAttr)
    }
}
