# Release Quality Scanner Sweep

Poodle remains `strict-ready`. The repository-quality scanners now distinguish
actionable release risks from deliberate, self-reporting Rust lint
expectations.

## Findings

- Attention markers: no findings across package sources.
- Generated files in source trees: no unexpected generated inputs.
- Comment ratio: no comment-heavy files.
- Stale suppressions: the default scan reported 18 Rust `#[expect]`
  attributes and no silent lint bypasses.

The Rust expectations carry reasons and cover deliberate API-shape or preview
compatibility cases. Rustc reports `unfulfilled_lint_expectations` when an
expected lint stops firing, so these attributes already expose their own stale
state. The two `dead_code` expectations retain optional preview compatibility
builders whose values are still forwarded by the renderer.

## Current State

`stale-suppressions` now scans silent and tool-specific bypasses, including
TypeScript, ESLint, Rust `allow`, RuboCop, SwiftLint, Prettier, Stylelint,
ShellCheck, and generic `nolint` markers. It runs as part of `effigy doctor`.
Rust `#[expect]` is intentionally outside that marker set.

## Validated

- `effigy scan attention-markers`
- `effigy scan generated-in-src`
- `effigy scan comment-ratio`
- `effigy scan stale-suppressions`
- `effigy doctor`
- `effigy docs:check`
- `git diff --check`
