use crate::*;

pub(crate) struct RouteAttr {
    pub(crate) server: Option<Expr>,
    pub(crate) path: Expr,
}
