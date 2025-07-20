use crate::*;

pub(crate) struct ResponseHeaderData {
    pub(crate) key: Expr,
    pub(crate) value: Expr,
    pub(crate) operation: HeaderOperation,
}

pub(crate) struct ResponseBodyData {
    pub(crate) body: Expr,
}
