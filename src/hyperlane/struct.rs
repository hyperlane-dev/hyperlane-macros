use crate::*;

/// Represents the attribute for the Hyperlane macro.
///
/// It consists of a type name and a variable name, separated by `=>`.
pub(crate) struct HyperlaneAttr {
    /// The type name of the component to initialize.
    pub(crate) type_name: Ident,
    /// The arrow token `=>` separating the type and variable names.
    pub(crate) _arrow: Token![=>],
    /// The variable name to assign the initialized component to.
    pub(crate) var_name: Ident,
}
