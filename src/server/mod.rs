/// Server module containing macro implementations for server operations.
///
/// This module provides macros for configuring and managing HTTP server behavior.
mod server_macro;

/// Re-export server macros for public use.
pub(crate) use server_macro::*;
