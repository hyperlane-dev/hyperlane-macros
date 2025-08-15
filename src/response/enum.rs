/// Defines operations that can be performed on response headers.
pub(crate) enum HeaderOperation {
    /// Sets an existing header value, keeping the original if not present.
    Set,
    /// Add a new header value, overwriting any existing value.
    Add,
}
