# nah

`nah` is a proc-macro crate for flagging clean-code and code-style violations at compile time.

## What it does

The crate provides a set of attribute macros that can be attached to functions, structs, enums, traits, and `impl` blocks.

- In normal mode, the macro does not break the build; it emits a Cargo warning instead.
- If the `DEPLOY=1` environment variable is set, the macro turns the violation into a hard compile error.
- If `PROFILE=release`, the check is skipped and the annotated item is returned unchanged.

## Supported attributes

### Complexity & metrics
- `#[high_complexity]`
- `#[arrow_anti_pattern]`
- `#[too_many_arguments]`
- `#[huge_item]`

### Duplication
- `#[duplicate]`
- `#[have_duplicate_code]`
- `#[copy_paste_programming]`

### Code smells & architecture
- `#[spaghetti_code]`
- `#[god_object]`
- `#[dead_code_candidate]`
- `#[no_needing_function]`
- `#[premature_optimization]`
- `#[magic_numbers]`
- `#[temporary_field]`
- `#[kiss_violation]`

### Side effects & clean functions
- `#[side_effects]`
- `#[output_arguments]`
- `#[flag_argument]`

### SOLID violations
- `#[srp_violation]`
- `#[ocp_violation]`
- `#[lsp_violation]`
- `#[isp_violation]`
- `#[dip_violation]`

### Psychological technical debt
- `#[crutch]`
- `#[broken_window]`
- `#[boy_scout_fail]`

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
nah = "0.1"
```

## Usage example

```rust
use nah::too_many_arguments;

#[too_many_arguments("Consider grouping parameters into a config struct")]
pub fn render(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
) {
    println!("{} {} {} {}", a, b, c, d);
}
```

You can also use an attribute without a message:

```rust
use nah::spaghetti_code;

#[spaghetti_code]
fn legacy_parser() {}
```

## Build behavior

### Development mode
If `DEPLOY` is not set to `1`, compilation continues, but Cargo prints a warning like this:

```text
cargo:warning=[nah] Clean Code violation on `render`: `#[too_many_arguments]` -> Reason: Consider grouping parameters into a config struct.
```

### Deployment mode
If `DEPLOY=1`, compilation stops with an error and the build fails.

```bash
DEPLOY=1 cargo build
```

### Release mode
If `PROFILE=release`, the macro returns the original item without warnings or errors.

## Notes

- The macro accepts an optional string message, for example: `#[kiss_violation("Too clever, simplify this code")]`.
- The message is only used for diagnostics and does not affect code generation.
- Supported items include `fn`, `struct`, `enum`, `trait`, and `impl`.

## Tests

This repository includes integration tests powered by `trybuild`.

Run them with:

```bash
cargo test
```

## License

MIT


