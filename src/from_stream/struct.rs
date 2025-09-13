use crate::*;

/// Represents data for stream processing.
///
/// This struct holds the buffer and variable name for stream processing.
pub(crate) struct FromStreamData {
    /// The buffer to read from the stream.
    pub(crate) buffer: Option<Expr>,
    /// The variable name to store the read data.
    pub(crate) variable_name: Option<Expr>,
}
