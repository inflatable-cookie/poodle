# g15.035 — Solid Tone Surfaces (August batch log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/035-solid-status-surfaces.md`
Handoff: `docs/handoffs/20260819-143940-g15-035-solid-tone-surfaces.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-5f45fbf7`
Branch: `t3code/solid-tone-surfaces`

## Summary

`Callout`, `Pill`, and `RemediationBanner` now share one opt-in `ToneFill` axis
across Svelte, React, renderer-neutral Rust, and GPUI. `fill="tint"` remains
the default and keeps the existing tint and Pill appearance recipes. The new
`fill="solid"` treatment uses one measured 45/55 sRGB surface rule, inverse
foregrounds, neutral handling, readable local action recipes, and current-tone
pending spinners.

## Package change class

- **Change class:** additive public API plus a behavioral parity correction on
  the pre-1.0 preview channel
- **Packages:** `@inflatable-cookie/poodle-core`,
  `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-react`,
  `poodle-specs`, `poodle-render`, and preview-only consumers
- **Public entry points:** shared `ToneFill = "tint" | "solid"`; `fill` on
  Callout, Pill, and RemediationBanner; corresponding Rust spec fields and
  builders
- **Behavioral correction:** `CallOutSpec::default()` now matches the web and
  contract default (`Neutral` + `Tint`) instead of the incorrect implicit
  `Info` tone
- **Migration:** none for web callers; tint remains the default. Rust callers
  that relied on the incorrect implicit Info tone must set `StatusTone::Info`
  explicitly.

## Implementation evidence

- Shared contract/type authority added to
  `docs/contracts/004-shared-control-types.md`, both TypeScript type surfaces,
  and `poodle-specs`.
- All three contracts describe tint preservation, solid precedence, neutral
  and pending behavior, inverse foregrounds, local action recipes, and focus
  behavior.
- `poodle-render` resolves non-neutral solid backgrounds as opaque 45% tone
  base + 55% `color.text.primary` in sRGB; neutral uses primary text directly;
  borders are raw tone base or `color.border.strong`; foreground is
  `color.text.inverse`.
- Pill solid fill takes precedence over `appearance` color/opacity recipes,
  retains badge typography, keeps muted opacity, uses custom accent as the tone
  base, and paints dot/remove affordances with inverse foreground.
- Rust tests assert the shared surface formula in Callout, Pill precedence and
  custom accent behavior, neutral handling, inverse dot/remove affordances,
  pending spinner color, and local secondary action recipes.

## Contrast evidence

The renderer contrast test covers all twelve current themes, neutral, info,
success, warning, danger, and accent bases. The minimum measured normal-text
contrast for inverse foreground was **5.024:1 in Clay**, above the 4.5:1 floor
and the card's 5:1 target.

## Generated artifacts

`effigy ir:build` regenerated the authored Callout and Pill display-specimen
fixture plus Svelte, React, GPUI, and Jetstream generated specimen artifacts.
The docs gate also regenerated the expected React
`packages/react/preview/artifacts/component-docs.json` prop surface.

## Validation

| Command | Outcome |
|---------|---------|
| focused paired Callout/Pill/RemediationBanner Vitest run | 6 files, 20 tests pass |
| `cargo test --manifest-path packages/render/Cargo.toml --lib --quiet` | 333 tests pass |
| `cargo check --manifest-path packages/codegen/Cargo.toml` | pass |
| `cargo check --manifest-path packages/contracts/components/Cargo.toml` | pass |
| `effigy ir:build` / `effigy ir:check` | pass; generated artifacts current |
| `effigy test:components` | 347 files, 2826 tests pass |
| `effigy check:svelte` | 0 errors; baseline warnings only |
| `effigy react:build` | pass |
| `effigy test:parity` | 6 files, 365 tests pass |
| `effigy check:gpui` | pass; render 333 and node backend 19 tests pass |
| `effigy regressions:native` | 49 tests pass |
| `effigy test:web-pack-install` | 5 files, 11 tests pass |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | clean (post-commit) |

## Live preview checkpoint

Svelte and React preview servers are left running for operator review:

- Svelte: `http://localhost:4174/?theme=eclipse&density=compact&controlSize=sm#components/remediation-banner`
- React: `http://localhost:4180/?theme=eclipse&density=compact&controlSize=sm#components/remediation-banner`

Both pages expose the solid groups and matching `data-fill` values. Browser
inspection confirmed matching computed solid surface, border, foreground, and
secondary-action values for the paired pages. The operator's visual acceptance
of Callout, Pill, and RemediationBanner remains open; this worker does not claim
that checkpoint passed.

## Unresolved / out of scope

- Existing Svelte accessibility/deprecation warnings and Happy DOM async-task
  cleanup diagnostics remain in broad checks; all named selectors exit green.
- `effigy bootstrap:deps` hit the known sibling-worktree Cargo lock collision;
  the Bun dependencies were installed and direct manifest-based Rust checks and
  required selectors passed.
- Windowed, native-visual, conformance, Jetstream, and release selectors were
  not run by scope.
- PR review, operator preview acceptance, and merge remain with the orchestrator.
