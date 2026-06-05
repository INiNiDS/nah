extern crate proc_macro;

use nah_core::define_gates;

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
