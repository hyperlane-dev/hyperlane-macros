use crate::*;

/// Implements an HTTP method macro.
///
/// This macro generates a handler function for a specific HTTP method (e.g., GET, POST).
/// It expands to a check that aborts the request if the HTTP method does not match.
///
/// # Arguments
///
/// - `$name` - The name of the generated handler function.
/// - `$method` - The HTTP method as a string literal (e.g., "get", "post").
///
/// # Returns
///
/// Returns a macro that generates a handler function for the specified HTTP method.
macro_rules! impl_http_method_macro {
    ($name:ident, $method:expr) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(
                position,
                item,
                create_method_check($method, proc_macro2::Span::call_site()),
            )
        }
        inventory::submit! {
            InjectableMacro {
                name: $method,
                handler: Handler::NoAttrPosition($name),
            }
        }
    };
}

// Generates a handler that checks if the HTTP method is GET.
impl_http_method_macro!(get_handler, "get");

// Generates a handler that checks if the HTTP method is POST.
impl_http_method_macro!(epilogue_handler, "post");

// Generates a handler that checks if the HTTP method is PUT.
impl_http_method_macro!(put_handler, "put");

// Generates a handler that checks if the HTTP method is DELETE.
impl_http_method_macro!(delete_handler, "delete");

// Generates a handler that checks if the HTTP method is PATCH.
impl_http_method_macro!(patch_handler, "patch");

// Generates a handler that checks if the HTTP method is HEAD.
impl_http_method_macro!(head_handler, "head");

// Generates a handler that checks if the HTTP method is OPTIONS.
impl_http_method_macro!(options_handler, "options");

// Generates a handler that checks if the HTTP method is CONNECT.
impl_http_method_macro!(connect_handler, "connect");

// Generates a handler that checks if the HTTP method is TRACE.
impl_http_method_macro!(trace_handler, "trace");

/// Creates a method check function for HTTP request validation.
///
/// # Arguments
///
/// - `method_name` - The HTTP method name as a string.
/// - `span` - The span for error reporting.
///
/// # Returns
///
/// Returns a closure that generates the method check code.
pub(crate) fn create_method_check(
    method_name: &str,
    span: proc_macro2::Span,
) -> impl FnOnce(&Ident) -> TokenStream2 {
    let check_method: Ident = Ident::new(&format!("is_{}", method_name), span);
    move |context| {
        quote! {
            if !#context.get_request().await.#check_method() {
                return;
            }
        }
    }
}

/// Handles HTTP requests for multiple method types.
///
/// This macro allows a handler to respond to multiple HTTP methods.
/// It generates code that checks if the request method matches any of the specified methods.
///
/// # Arguments
///
/// - `TokenStream` - The attribute `TokenStream` containing the list of allowed HTTP methods.
/// - `TokenStream` - The input `TokenStream` representing the handler function.
/// - `Position` - The position at which to inject the method check code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the methods check code injected.
pub(crate) fn methods_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let item_clone_1: TokenStream = item.clone();
    let methods: RequestMethods = parse_macro_input!(attr as RequestMethods);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let sig: &Signature = &input_fn.sig;
    match parse_self_from_method(sig) {
        Ok(context) => {
            let method_checks = methods.methods.iter().map(|method| {
                let check_fn: Ident = Ident::new(&format!("is_{}", method), method.span());
                quote! {
                    #context.get_request().await.#check_fn()
                }
            });
            inject(position, item_clone_1, |_| {
                quote! {
                    if !(#(#method_checks)||*) {
                        return;
                    }
                }
            })
        }
        Err(err) => err.to_compile_error().into(),
    }
}

inventory::submit! {
    InjectableMacro {
        name: "methods",
        handler: Handler::WithAttrPosition(methods_macro),
    }
}
