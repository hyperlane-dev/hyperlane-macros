use crate::*;

/// Wraps function body with WebSocket stream processing.
///
/// This macro generates code that wraps the function body with a check to see if
/// data can be read from a WebSocket stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `attr` - The attribute containing the buffer and variable name.
/// - `item` - The input token stream to process.
/// - `position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with WebSocket stream processing.
pub(crate) fn ws_from_stream_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ws_data: WsFromStreamData = parse_macro_input!(attr as WsFromStreamData);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let stmts: &Vec<Stmt> = &block.stmts;
            let loop_stream: TokenStream2 = match (ws_data.buffer, ws_data.variable_name) {
                (Some(buffer), Some(variable_name)) => {
                    quote! {
                        while let Ok(#variable_name) = #context.ws_from_stream(#buffer).await {
                            #(#stmts)*
                        }
                    }
                }
                (Some(buffer), None) => {
                    quote! {
                        while #context.ws_from_stream(#buffer).await.is_ok() {
                            #(#stmts)*
                        }
                    }
                }
                (None, Some(variable_name)) => {
                    quote! {
                        while let Ok(#variable_name) = #context.ws_from_stream(hyperlane::DEFAULT_BUFFER_SIZE).await {
                            #(#stmts)*
                        }
                    }
                }
                (None, None) => {
                    return syn::Error::new_spanned(
                        TokenStream2::new(),
                        "Internal error: both buffer and variable_name are None",
                    )
                    .to_compile_error()
                    .into();
                }
            };
            quote! {
                #(#attrs)*
                #vis #sig {
                    #loop_stream
                }
            }
            .into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

inventory::submit! {
    InjectableMacro {
        name: "ws_from_stream",
        handler: Handler::AttrPosition(ws_from_stream_macro),
    }
}
