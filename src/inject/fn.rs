use crate::*;

fn apply_macro(macro_meta: &Meta, item_stream: TokenStream) -> TokenStream {
    let (macro_name, macro_attr) = match macro_meta {
        Meta::Path(path) => (
            path.get_ident()
                .expect("Macro path should have an identifier")
                .to_string(),
            TokenStream::new(),
        ),
        Meta::List(meta_list) => (
            meta_list
                .path
                .get_ident()
                .expect("Macro path should have an identifier")
                .to_string(),
            meta_list.tokens.clone().into(),
        ),
        _ => panic!("Unsupported macro format in inject macro"),
    };
    for injectable_macro in inventory::iter::<InjectableMacro>() {
        if injectable_macro.name == macro_name {
            return match injectable_macro.handler {
                Handler::Simple(handler) => {
                    if !macro_attr.is_empty() {
                        panic!("Macro {} does not take attributes", macro_name);
                    }
                    handler(item_stream)
                }
                Handler::WithAttr(handler) => handler(macro_attr, item_stream),
            };
        }
    }
    panic!("Unsupported macro: {}", macro_name);
}

pub(crate) fn prologue_hooks_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let metas: Punctuated<Meta, Comma> = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse(attr.into())
        .expect("Failed to parse macro attributes");
    let mut current_stream: TokenStream = item;
    for meta in metas.iter().rev() {
        current_stream = apply_macro(meta, current_stream);
    }
    current_stream
}

pub(crate) fn epilogue_hooks_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let metas: Punctuated<Meta, Comma> = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse(attr.into())
        .expect("Failed to parse macro attributes");
    let mut current_stream: TokenStream = item;
    for meta in metas.iter() {
        current_stream = apply_macro(meta, current_stream);
    }
    current_stream
}
