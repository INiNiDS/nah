use std::env;

/// The core engine evaluating Clean Code gates during compilation.
///
/// Takes a macro name, attribute token stream, and item token stream (all as `proc_macro2::TokenStream`),
/// and returns the original item (or a compile error in deploy mode).
pub fn process_gate(
    macro_name: &str,
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if env::var("PROFILE").unwrap_or_default() == "release" {
        return item;
    }

    let input: syn::Item = syn::parse2(item).expect("failed to parse item");

    let custom_message = if !attr.is_empty() {
        if let Ok(lit) = syn::parse2::<syn::LitStr>(attr) {
            format!(" -> Reason: {}", lit.value())
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let ident_str = match &input {
        syn::Item::Fn(f) => f.sig.ident.to_string(),
        syn::Item::Struct(s) => s.ident.to_string(),
        syn::Item::Enum(e) => e.ident.to_string(),
        syn::Item::Trait(t) => t.ident.to_string(),
        syn::Item::Impl(i) => match &*i.self_ty {
            syn::Type::Path(tp) => tp.path.segments.last().unwrap().ident.to_string(),
            _ => "unknown_impl".to_string(),
        },
        _ => "unknown_item".to_string(),
    };

    if env::var("DEPLOY").unwrap_or_default() == "1" {
        let error_msg = format!(
            "🚨 [NAH! DEPLOYMENT BLOCKED] Code quality gate triggered!\n\
             Element `{}` is violating Clean Code standards.\n\
             Marker: `#[{}]`{}.\n\
             Fix this violation before merging to production.",
            ident_str, macro_name, custom_message
        );
        syn::Error::new_spanned(&input, error_msg).to_compile_error()
    } else {
        println!(
            "cargo:warning=[nah] Clean Code violation on `{}`: `#[{}]`{}.",
            ident_str, macro_name, custom_message
        );
        quote::quote! { #input }
    }
}

/// Macro generator to avoid duplicating token-parsing boilerplates.
///
/// Creates a `#[proc_macro_attribute]` function for each identifier,
/// delegating to [`process_gate`].
///
/// # Example
///
/// ```ignore
/// define_gates!(too_many_arguments, spaghetti_code);
/// ```
///
/// This generates:
/// ```ignore
/// #[proc_macro_attribute]
/// pub fn too_many_arguments(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
/// #[proc_macro_attribute]
/// pub fn spaghetti_code(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
/// ```
#[macro_export]
macro_rules! define_gates {
    ($($name:ident),* $(,)?) => {
        $(
            #[proc_macro_attribute]
            pub fn $name(attr: ::proc_macro::TokenStream, item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                let attr = ::proc_macro2::TokenStream::from(attr);
                let item = ::proc_macro2::TokenStream::from(item);
                $crate::process_gate(stringify!($name), attr, item).into()
            }
        )*
    };
}
