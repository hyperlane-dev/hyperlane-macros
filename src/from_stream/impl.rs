use crate::*;

/// Implementation of Parse trait for FromStreamData.
///
/// This implementation handles parsing of macro attributes that specify stream processing parameters.
/// It supports various parameter combinations including buffer size, variable name, or both.
/// The parser validates input syntax and semantic correctness according to the macro's requirements.
///
/// # Arguments
/// - `input`: The parse stream containing the token stream to be parsed
///
/// # Returns
/// Returns a `syn::Result<Self>` containing the parsed FromStreamData on success,
/// or a syn::Error with appropriate error message on failure
///
/// # Errors
/// This function returns an error when:
/// - No parameters are provided
/// - Two buffer size parameters are provided
/// - Two variable name parameters are provided
/// - Additional unexpected tokens are present after valid parameters
/// - A comma is present without a second parameter following it
impl Parse for FromStreamData {
    /// Parses the input token stream into a FromStreamData structure.
    ///
    /// This method implements the core parsing logic for the FromStream macro attribute.
    /// It handles three possible parameter configurations:
    /// 1. Single parameter: interpreted as buffer size if integer literal, otherwise as variable name
    /// 2. Two parameters: first as buffer size, second as variable name (order independent)
    /// 3. No parameters: results in an error
    ///
    /// The method performs comprehensive validation of the input syntax and semantics,
    /// ensuring that the resulting FromStreamData structure is valid and consistent.
    ///
    /// # Arguments
    /// - `ParseStream`: The ParseStream containing the token stream to be parsed
    ///
    /// # Returns
    /// Returns `syn::Result<Self>` where:
    /// - Ok(FromStreamData) contains the successfully parsed data with buffer and variable name
    /// - Err(syn::Error) contains an appropriate error message for invalid input
    ///
    /// # Errors
    /// The function returns errors in the following cases:
    /// - Empty input: when no parameters are provided
    /// - Two integer literals: when both parameters are buffer sizes
    /// - Two non-integer expressions: when both parameters are variable names
    /// - Malformed syntax: when comma is present without a second parameter
    /// - Extra tokens: when additional tokens are present after valid parameters
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut buffer: Option<Expr> = None;
        let mut variable_name: Option<Expr> = None;
        if input.is_empty() {
            return Ok(FromStreamData {
                buffer,
                variable_name,
            });
        }
        let first_expr: Expr = input.parse()?;
        if input.is_empty() {
            if is_integer_literal(&first_expr) {
                buffer = Some(first_expr);
            } else {
                variable_name = Some(first_expr);
            }
        } else {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                return Err(syn::Error::new(
                    input.span(),
                    "expected second parameter after comma",
                ));
            }
            let second_expr: Expr = input.parse()?;
            let first_is_int: bool = is_integer_literal(&first_expr);
            let second_is_int: bool = is_integer_literal(&second_expr);
            match (first_is_int, second_is_int) {
                (true, true) => {
                    return Err(syn::Error::new_spanned(
                        &second_expr,
                        "cannot have two buffer size parameters",
                    ));
                }
                (false, false) => {
                    return Err(syn::Error::new_spanned(
                        &second_expr,
                        "cannot have two variable name parameters",
                    ));
                }
                (true, false) => {
                    buffer = Some(first_expr);
                    variable_name = Some(second_expr);
                }
                (false, true) => {
                    variable_name = Some(first_expr);
                    buffer = Some(second_expr);
                }
            }
        }
        if !input.is_empty() {
            return Err(syn::Error::new_spanned(
                input.parse::<TokenStream2>()?,
                "unexpected additional tokens in attribute",
            ));
        }
        Ok(FromStreamData {
            buffer,
            variable_name,
        })
    }
}
