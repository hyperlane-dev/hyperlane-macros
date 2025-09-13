use crate::*;

/// Represents data for WebSocket stream processing.
///
/// This struct holds the buffer and variable name for WebSocket stream processing.
pub(crate) struct WsFromStreamData {
    /// The buffer to read from the WebSocket stream.
    pub(crate) buffer: Option<Expr>,
    /// The variable name to store the read data.
    pub(crate) variable_name: Option<Expr>,
}
