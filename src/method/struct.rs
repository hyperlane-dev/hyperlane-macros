use crate::*;

pub(crate) struct RequestMethods {
    pub(super) methods: Punctuated<Ident, Token![,]>,
}
