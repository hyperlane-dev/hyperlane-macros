use crate::*;

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

impl_http_method_macro!(get_handler, "get");
impl_http_method_macro!(post_handler, "post");
impl_http_method_macro!(put_handler, "put");
impl_http_method_macro!(delete_handler, "delete");
impl_http_method_macro!(patch_handler, "patch");
impl_http_method_macro!(head_handler, "head");
impl_http_method_macro!(options_handler, "options");
impl_http_method_macro!(connect_handler, "connect");
impl_http_method_macro!(trace_handler, "trace");

/// Creates method check function for HTTP request validation.
///
/// # Arguments
///
/// - `&str` - The HTTP method name to check.
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
                return;
            }
        }
    }
}

/// Handles HTTP requests for multiple method types.
///
/// # Arguments
///
/// - `TokenStream` - The allowed methods token stream.
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
