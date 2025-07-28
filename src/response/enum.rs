/// Defines operations that can be performed on response headers.
pub(crate) enum HeaderOperation {
    /// Sets a new header value, overwriting any existing value.
    Set,
    /// Replaces an existing header value, keeping the original if not present.
    Replace,
}
