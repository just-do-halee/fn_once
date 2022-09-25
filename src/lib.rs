use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn once(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();

    let error = |or: &str| {
        syn::Error::new_spanned(or, "Please use 'or' attribute. ex:) #[once(or = { ... })]")
            .into_compile_error()
    };

    let (command, block) = if attr.is_empty() {
        ("or", "{{}}")
    } else {
        match attr.split_once('=') {
            Some((command, block)) => (command.trim(), block.trim()),
            None => return error(&attr).into(),
        }
    };

    if command.trim() != "or" {
        return error(command).into();
    }

    let body = item.to_string();

    let (head1, body) = body.split_once("fn").expect("This is not a function");
    let (head2, body) = body.split_once('{').expect("This is not a function");

    let name = format!(
        "FN_ONCE_{}",
        syn::parse_macro_input!(item as syn::ItemFn)
            .sig
            .ident
            .to_string()
            .to_uppercase()
    );

    TokenStream::from_str(&format!(
        "
        static {name}: std::sync::Once = std::sync::Once::new();
        {head1} fn {head2} {{
            if {name}.is_completed() {{
                return {block};
            }}
            {name}.call_once(|| {{}});
        {body}"
    ))
    .unwrap()
}
