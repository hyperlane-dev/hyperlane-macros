use crate::*;

impl Parse for RequestMethods {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RequestMethods {
            methods: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

impl Parse for BodyParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let type_name: Type = input.parse()?;
        Ok(BodyParams {
            variable,
            type_name,
        })
    }
}
