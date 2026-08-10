# Rust Publication Documentation

Poodle remains `strict-ready`. Every public-intent Rust package has a bounded
crate payload, but strict rustdoc exposed broken public documentation in two
crates.

## Findings

- All 13 public-intent Rust manifests completed `cargo package --list`.
- `poodle-specs` had four unresolved method links and two raw heading tags in
  public docs.
- `poodle-render` had seven ambiguous function/module links and one raw
  `<aside>` tag in crate documentation.
- The other 11 public-intent crates built rustdoc with warnings denied.

## Repaired

- Qualified `FormActionsSpec` method links through `Self`.
- Marked literal HTML element names as code.
- Replaced ambiguous module/function links with full function paths.

## Validated

- `cargo package --list` for all 13 public-intent Rust crates
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` for all 13 public-intent
  Rust crates
- `effigy ci:rust`
- `git diff --check`
