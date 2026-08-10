# God-File Signal Audit

Poodle remains `strict-ready`. The 26 god-file warnings contained no high or
critical findings. Twenty-five are cohesive units; one is a real structural
candidate.

## Findings

### Accepted cohesive units

- Seven React and one Svelte component shells keep component-local state,
  markup, and accessibility wiring together.
- Seven shared Rust renderer modules are single-component composition units;
  their behavior lives in `poodle-specs` or `poodle-headless`.
- Four renderer-neutral spec modules keep public shape, derived helpers, and
  their contract tests together.
- The headless text-input module and cross-runtime conformance suite each own
  one behavior boundary.
- Three Jetstream direct-adapter modules are flat backend mapping catalogues.
- React's `types.ts` is the same type-aggregation case already accepted for
  Svelte and Rust contract crates.

These paths are now explicit scanner exclusions with the ownership reason next
to each group. The exclusions are narrow; new large component or renderer files
still surface automatically.

### Remaining structural candidate

`packages/gpui/node-backend/src/lib.rs` has 1,177 code lines and 1,554 total
lines. It is one node interpreter, but it currently owns leaf conversion,
style projection, interaction routing, drag/drop, focus tracking, and animation
projection in the same module. Its IME, text-input, and tests are already split.
The next batch should extract a real style or interaction boundary and preserve
the existing backend API.

## Current State

- god-file findings: 1 warning, down from 26
- high: 0
- critical: 0
- remaining path: `packages/gpui/node-backend/src/lib.rs`

## Validated

- `effigy scan god-files`
- `effigy doctor`
- `effigy docs:check`
- `git diff --check`
