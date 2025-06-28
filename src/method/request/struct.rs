use crate::*;

pub(crate) struct RequestMethods {
    pub(crate) methods: Punctuated<Ident, Token![,]>,
}

pub(crate) struct BodyParams {
    pub(crate) variable: Ident,
    pub(crate) type_name: Type,
}
