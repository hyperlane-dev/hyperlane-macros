use crate::*;

/// Represents the attribute for the Hyperlane macro.
///
/// It consists of a variable name and a type name, separated by `:`.
pub(crate) struct HyperlaneAttr {
    /// The variable name to assign the initialized component to.
    pub(crate) var_name: Ident,
    /// The colon token `:` separating the variable and type names.
    pub(crate) _colon: Token![:],
    /// The type name of the component to initialize.
    pub(crate) type_name: Ident,
}

/// Represents multiple attributes for the Hyperlane macro.
///
/// Used to store parsed multiple variable-type pairs from macro input.
pub(crate) struct MultiHyperlaneAttr {
    /// Vector of variable-type pairs.
    pub(crate) params: Vec<(Ident, Ident)>,
}
