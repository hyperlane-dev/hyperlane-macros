use crate::*;

/// Creates the method check function identifier.
///
/// # Arguments
///
/// - `&str` - The HTTP method name as a string.
/// - `proc_macro2::Span` - The span for error reporting.
///
/// # Returns
///
/// Returns the check function identifier.
pub(crate) fn create_method_check_ident(method: &str, span: proc_macro2::Span) -> Ident {
    Ident::new(&format!("get_request_is_{method}_method"), span)
}

/// Creates a method check function for HTTP request validation.
///
/// # Arguments
///
/// - `&proc_macro2::Ident` - The HTTP method name as an ident.
/// - `proc_macro2::Span` - The span for error reporting.
///
/// # Returns
///
/// Returns a closure that generates the method check code.
pub(crate) fn create_method_check(
    method: &proc_macro2::Ident,
    span: proc_macro2::Span,
) -> impl FnOnce(&Ident) -> TokenStream2 {
    let check_method: Ident = create_method_check_ident(&method.to_string(), span);
    move |context| {
        quote! {
            if !#context.#check_method().await {
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
    match parse_context_from_signature(sig) {
        Ok(context) => {
            let method_checks = methods.methods.iter().map(|method| {
                let check_fn: Ident = create_method_check_ident(&method.to_string(), method.span());
                quote! {
                    #context.#check_fn().await
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

/// Implements an HTTP method macro.
///
/// This macro generates a handler function for a specific HTTP method (e.g., GET, POST).
/// It expands to a check that aborts the request if the HTTP method does not match.
///
/// # Arguments
///
/// - `$name` - The name of the generated handler function.
/// - `$submit_name` - The string name for macro registration as an ident.
/// - `$method` - The HTTP method as an ident (e.g., get, post).
///
/// # Returns
///
/// Returns a macro that generates a handler function for the specified HTTP method.
macro_rules! impl_http_method_macro {
    ($name:ident, $submit_name:ident, $method:ident) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(
                position,
                item,
                create_method_check(
                    &proc_macro2::Ident::new(stringify!($method), proc_macro2::Span::call_site()),
                    proc_macro2::Span::call_site(),
                ),
            )
        }
        inventory::submit! {
            InjectableMacro {
                name: stringify!($submit_name),
                handler: Handler::NoAttrPosition($name),
            }
        }
    };
}

impl_http_method_macro!(get_method_handler, get_method, get);
impl_http_method_macro!(post_method_handler, post_method, post);
impl_http_method_macro!(put_method_handler, put_method, put);
impl_http_method_macro!(delete_method_handler, delete_method, delete);
impl_http_method_macro!(patch_method_handler, patch_method, patch);
impl_http_method_macro!(head_method_handler, head_method, head);
impl_http_method_macro!(options_method_handler, options_method, options);
impl_http_method_macro!(connect_method_handler, connect_method, connect);
impl_http_method_macro!(trace_method_handler, trace_method, trace);
impl_http_method_macro!(unknown_method_handler, unknown_method, unknown);
