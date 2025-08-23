use crate::*;

/// Implements an HTTP method macro.
///
/// This macro generates a handler function for a specific HTTP method (e.g., GET, POST).
/// It expands to a check that aborts the request if the HTTP method does not match.
///
/// # Arguments
///
/// - `$name`: The name of the generated handler function.
/// - `$method`: The HTTP method as a string literal (e.g., "get", "post").
macro_rules! impl_http_method_macro {
    ($name:ident, $method:expr) => {
        pub(crate) fn $name(item: TokenStream) -> TokenStream {
            expand_check_macro(
                item,
                create_method_check($method, proc_macro2::Span::call_site()),
            )
        }
    };
}

// This macro expands to a check that aborts the request if the HTTP method is not GET.
impl_http_method_macro!(get_handler, "get");

// This macro expands to a check that aborts the request if the HTTP method is not POST.
impl_http_method_macro!(post_handler, "post");

// This macro expands to a check that aborts the request if the HTTP method is not PUT.
impl_http_method_macro!(put_handler, "put");

// This macro expands to a check that aborts the request if the HTTP method is not DELETE.
impl_http_method_macro!(delete_handler, "delete");

// This macro expands to a check that aborts the request if the HTTP method is not PATCH.
impl_http_method_macro!(patch_handler, "patch");

// This macro expands to a check that aborts the request if the HTTP method is not HEAD.
impl_http_method_macro!(head_handler, "head");

// This macro expands to a check that aborts the request if the HTTP method is not OPTIONS.
impl_http_method_macro!(options_handler, "options");

// This macro expands to a check that aborts the request if the HTTP method is not CONNECT.
impl_http_method_macro!(connect_handler, "connect");

// This macro expands to a check that aborts the request if the HTTP method is not TRACE.
impl_http_method_macro!(trace_handler, "trace");

/// Creates method check function for HTTP request validation.
///
/// # Arguments
///
/// - `&str` - The method name string.
/// - `proc_macro2::Span` - The span for error reporting.
///
/// # Returns
///
/// - `impl FnOnce(&Ident) -> TokenStream2` - The generated check function.
pub(crate) fn create_method_check(
    method_name: &str,
    span: proc_macro2::Span,
) -> impl FnOnce(&Ident) -> TokenStream2 {
    let check_method: Ident = Ident::new(&format!("is_{}", method_name), span);
    move |context| {
        quote! {
            if !#context.get_request().await.#check_method() {
                let _ = #context.aborted().await;
                return;
            }
        }
    }
}

/// Handles HTTP requests for multiple method types.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with methods check.
pub(crate) fn methods_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let methods: RequestMethods = parse_macro_input!(attr as RequestMethods);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_fn(sig) {
        Ok(context) => {
            let method_checks = methods.methods.iter().map(|method| {
                let check_fn: Ident = Ident::new(&format!("is_{}", method), method.span());
                quote! {
                    #context.get_request().await.#check_fn()
                }
            });
            let check_expr: TokenStream2 = quote! {
                if !(#(#method_checks)||*) {
                    let _ = #context.aborted().await;
                    return;
                }
            };
            let stmts: &Vec<Stmt> = &block.stmts;
            let gen_code: TokenStream2 = quote! {
                #(#attrs)*
                #vis #sig {
                    #check_expr
                    #(#stmts)*
                }
            };
            gen_code.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
