use crate::*;

pub type MacroHandler = fn(TokenStream) -> TokenStream;
pub type MacroHandlerWithAttr = fn(TokenStream, TokenStream) -> TokenStream;
