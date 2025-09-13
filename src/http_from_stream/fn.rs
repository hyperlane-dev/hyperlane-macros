use crate::*;

/// Wraps function body with HTTP stream processing.
///
/// This macro generates code that wraps the function body with a check to see if
/// data can be read from an HTTP stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `TokenStream` - The attribute containing the buffer and variable name.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with HTTP stream processing.
pub(crate) fn http_from_stream_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let data: FromStreamData = parse_macro_input!(attr as FromStreamData);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let stmts: &Vec<Stmt> = &block.stmts;
            let loop_stream: TokenStream2 = match (data.buffer, data.variable_name) {
                (Some(buffer), Some(variable_name)) => {
                    quote! {
                        while let Ok(#variable_name) = #context.http_from_stream(#buffer).await {
                            #(#stmts)*
                        }
                    }
                }
                (Some(buffer), None) => {
                    quote! {
                        while #context.http_from_stream(#buffer).await.is_ok() {
                            #(#stmts)*
                        }
                    }
                }
                (None, Some(variable_name)) => {
                    quote! {
                        while let Ok(#variable_name) = #context.http_from_stream(hyperlane::DEFAULT_BUFFER_SIZE).await {
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
        name: "http_from_stream",
        handler: Handler::WithAttr(http_from_stream_macro),
    }
}
