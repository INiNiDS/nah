extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, LitStr};
use std::env;

/// The core engine evaluating Clean Code gates during compilation.
fn process_gate(macro_name: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    if env::var("PROFILE").unwrap_or_default() == "release" {
        return item;
    }

    let input = parse_macro_input!(item as Item);

    // Parse custom reason message if provided, e.g., #[kiss_violation("Too many abstract factories")]
    let custom_message = if !attr.is_empty() {
        if let Ok(lit) = syn::parse::<LitStr>(attr) {
            format!(" -> Reason: {}", lit.value())
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    // Extract the name of the entity for logging
    let ident_str = match &input {
        Item::Fn(f) => f.sig.ident.to_string(),
        Item::Struct(s) => s.ident.to_string(),
        Item::Enum(e) => e.ident.to_string(),
        Item::Trait(t) => t.ident.to_string(),
        Item::Impl(i) => match &*i.self_ty {
            syn::Type::Path(tp) => tp.path.segments.last().unwrap().ident.to_string(),
            _ => "unknown_impl".to_string(),
        },
        _ => "unknown_item".to_string(),
    };

    // Quality gate enforcement logic
    if env::var("DEPLOY").unwrap_or_default() == "1" {
        // DEPLOYMENT MODE: Throw a hard compiler error and break the build
        let error_msg = format!(
            "🚨 [NAH! DEPLOYMENT BLOCKED] Code quality gate triggered!\nElement `{}` is violating Clean Code standards.\nMarker: `#[{}]`{}.\nFix this violation before merging to production.",
            ident_str, macro_name, custom_message
        );
        return syn::Error::new_spanned(&input, error_msg)
            .to_compile_error()
            .into();
    } else {
        // DEVELOPMENT MODE: Let it compile, but spam the Cargo terminal with warnings
        println!(
            "cargo:warning=[nah] Clean Code violation on `{}`: `#[{}]`{}.",
            ident_str, macro_name, custom_message
        );
    }

    quote! { #input }.into()
}

// Macro generator to avoid duplicating token-parsing boilerplates
macro_rules! define_gates {
    ($($name:ident),* $(,)?) => {
        $(
            #[proc_macro_attribute]
            pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
                process_gate(stringify!($name), attr, item)
            }
        )*
    };
}

// =========================================================================
// CLEAN CODE VIOLATION REGISTRY (The Ultimate `nah` Marker List)
// =========================================================================
define_gates!(
    // 1. Complexity & Metrics
    high_complexity,        // Cyclomatic complexity is through the roof
    arrow_anti_pattern,     // Arrow anti-pattern (deeply nested if/match blocks)
    too_many_arguments,     // Function accepts more than 3-4 arguments
    huge_item,              // Bloated function or struct (> 100 lines of code)

    // 2. Duplication (Don't Repeat Yourself - DRY)
    duplicate,              // Explicit duplicate of another system entity
    have_duplicate_code,    // Function contains copy-pasted blocks of logic
    copy_paste_programming, // Code written via Ctrl+C -> Ctrl+V engineering

    // 3. Code Smells & Architecture
    spaghetti_code,         // Control flow is tangled and unreadable
    god_object,             // Struct/Module knows or does too much
    dead_code_candidate,    // Unused code kept around because "we are afraid to delete it"
    no_needing_function,    // Dead code left "for future use"
    premature_optimization, // Readability sacrificed for unmeasured performance gains
    magic_numbers,          // Hardcoded primitives used instead of named constants
    temporary_field,        // Struct fields used only under specific, rare conditions
    kiss_violation,         // Overengineered solution (Keep It Simple, Stupid violation)

    // 4. Side Effects & Clean Functions
    side_effects,           // Function mutates global state or triggers hidden behavior
    output_arguments,       // Modifying arguments via mutable references instead of returning values
    flag_argument,          // Passing a boolean flag that splits the function into two separate flows

    // 5. SOLID Violations
    srp_violation,          // Single Responsibility Principle violation
    ocp_violation,          // Open/Closed Principle violation
    lsp_violation,          // Liskov Substitution Principle violation
    isp_violation,          // Interface Segregation Principle violation
    dip_violation,          // Dependency Inversion Principle violation

    // 6. Psychological Technical Debt
    crutch,                 // A deliberate, temporary workaround (hack/quick fix)
    broken_window,          // Code left dirty because the surrounding file is already a mess
    boy_scout_fail,         // Left the codebase messier than it was before your commit
);