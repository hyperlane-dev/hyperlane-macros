use crate::*;

/// Represents data for stream processing.
///
/// This struct holds the request_config and variable name for stream processing.
pub(crate) struct FromStreamData {
    /// The request config to read from the stream.
    pub(crate) request_config: Option<Expr>,
    /// The variable name to store the read data.
    pub(crate) variable_name: Option<Expr>,
}
