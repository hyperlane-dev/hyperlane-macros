use crate::*;
use syn::Ident;

/// Generates stream processing loop based on context and data.
///
/// This function abstracts the common logic between HTTP and WebSocket stream macros.
/// It creates a token stream that wraps function body with a loop that reads from
/// a specified stream method.
///
/// # Arguments
///
/// - `&Ident` - The context identifier to use for stream access
/// - `&str` - The stream method to call (e.g., "http_from_stream" or "ws_from_stream")
/// - `&FromStreamData` - The FromStreamData containing variable name
/// - `&[Stmt]` - The statements to execute when data is successfully read
///
/// # Returns
///
/// - `TokenStream2` - The generated loop code as a token stream
pub(crate) fn generate_stream(
    context: &Ident,
    stream_method: &str,
    data: &FromStreamData,
    stmts: &[Stmt],
) -> TokenStream2 {
    let method_ident: Ident = Ident::new(stream_method, proc_macro2::Span::call_site());
    match data.variable_name.clone() {
        Some(variable_name) => {
            quote! {
                while let Ok(#variable_name) = #context.#method_ident().await {
                    #(#stmts)*
                }
            }
        }
        None => {
            quote! {
                while #context.#method_ident().await.is_ok() {
                    #(#stmts)*
                }
            }
        }
    }
}

/// Wraps function body with HTTP stream processing.
///
/// This macro generates code that wraps the function body with a check to see if
/// data can be read from an HTTP stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `TokenStream` - The attribute containing the variable name.
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
    match parse_context_from_signature(sig) {
        Ok(context) => {
            let stmts: &Vec<Stmt> = &block.stmts;
            let loop_stream: TokenStream2 =
                generate_stream(context, "http_from_stream", &data, stmts);
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

/// Wraps function body with WebSocket stream processing.
///
/// This macro generates code that wraps the function body with a check to see if
/// data can be read from a WebSocket stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `attr` - The attribute containing the variable name.
/// - `item` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with WebSocket stream processing.
pub(crate) fn ws_from_stream_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let data: FromStreamData = parse_macro_input!(attr as FromStreamData);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_signature(sig) {
        Ok(context) => {
            let stmts: &Vec<Stmt> = &block.stmts;
            let loop_stream: TokenStream2 =
                generate_stream(context, "ws_from_stream", &data, stmts);
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
        handler: Handler::WithAttr(ws_from_stream_macro),
    }
}
