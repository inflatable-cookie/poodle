# g15.036 — Pill Appearance Semantics (August correction log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/036-pill-appearance-semantics.md`
Handoff: `docs/handoffs/20260819-154051-g15-036-pill-appearance-semantics.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/g15-036-pill-appearance-semantics`
Branch: `t3code/g15-036-pill-appearance-semantics`

## Summary

Pill's temporary `fill` axis from PR #44 (`g15.035`) is fully removed. Pill
now has one honest, mutually exclusive visual-treatment axis:

```ts
type PillAppearance = "tint" | "solid" | "subtle" | "badge";
```

`tint` is the default and preserves the pre-PR #44 ordinary tone-tinted shell
exactly. `solid` now owns the contrast-safe opaque 45/55 tone recipe its name
promised. `subtle` and `badge` are unchanged. Callout and RemediationBanner
keep the shared `ToneFill` API untouched.

## Package change class

- **Change class:** breaking public API correction on the pre-1.0 preview
  channel (approved pre-v0.2; no alias, precedence rule, or silent fallback)
- **Packages:** `@inflatable-cookie/poodle-core`,
  `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-react`,
  `poodle-specs`, `poodle-render`, and preview-only consumers
- **Public entry points removed:** Pill `fill` prop and `data-fill` output
  (web), `PillSpec::fill`, `PillSpec::with_fill`, `PillSpec::is_solid_fill`
  (Rust); Pill is no longer a named `ToneFill` consumer
- **Public entry points added/changed:** `PillAppearance` gains `"tint"` (web
  and Rust); the implicit appearance changes from `solid` to `tint` while
  default rendered output is unchanged; explicit `appearance="solid"` now
  resolves the opaque shared `solid_tone_surface` recipe;
  `PillSpec::is_solid_appearance` replaces `is_solid_fill`
- **Migration:** web callers using Pill `fill="solid"` switch to
  `appearance="solid"`; callers relying on the old default keep visuals with
  no change; Rust callers replace `with_fill(ToneFill::Solid)` with
  `with_appearance(PillAppearance::Solid)`

## Implementation evidence

- Contract authority corrected in `docs/contracts/components/pill.md`
  (four-appearance axis, exact default/solid recipes, mutually exclusive
  semantics) and `docs/contracts/004-shared-control-types.md` (`ToneFill`
  consumers reduced to Callout and RemediationBanner).
- Svelte and React shells drop `fill`/`data-fill`, default
  `appearance="tint"`, and emit only `data-appearance`.
- Shared Pill CSS resolves all treatment from `data-appearance`; the opaque
  recipe moved from `[data-fill="solid"]` to `[data-appearance="solid"]` with
  identical recipes and inverse-foreground dot handling. Tint is the
  documented base recipe, so default output is observably unchanged.
- `poodle-render::pill` branches on `PillAppearance::Solid` through the same
  shared `solid_tone_surface` resolver; the resolver itself is untouched.
- Paired web tests prove the default emits `data-appearance="tint"` with no
  `data-fill`, explicit tint renders identical markup to the default, all
  four appearances project distinctly, and dot/title anatomy is intact.
- Rust tests prove the default appearance is `Tint`, explicit tint matches
  the default, subtle halves the tint fill alpha (border/text unchanged),
  solid uses the shared opaque recipe for status tones, neutral, and custom
  accent, and solid dot/remove affordances use the inverse foreground.
- Authored display-specimen model drops the pill `fill` prop and `Solid
  fills` group, adds `tint` to the appearance domain, and teaches one compact
  `Appearances` group (tint/solid/subtle/badge); every derived Svelte, React,
  GPUI, Jetstream, and fixture artifact regenerated via `effigy ir:build`.
- GPUI authored Pill page teaches the same four-appearance group; GPUI and
  Jetstream scene renderers map the corrected domain with tint as the
  fallback.
- Generated component-docs artifacts for both web previews regenerated
  through the package scripts (`docs:export-json` / `docs:export`).

## Contrast evidence

The shared resolver's contrast test
(`solid_tone_surface_keeps_inverse_foreground_readable_across_themes`,
`poodle-render::color`) still passes across all twelve themes; this card
reassigned the recipe without changing color math.

## Generated artifacts

`effigy ir:build` regenerated `packages/codegen/fixtures/specimens-model.json`,
the Svelte/React `pill-specimen.ts` scenes, and the GPUI/Jetstream
`specimens.rs` scenes. `effigy ir:check` confirms all generated artifacts are
current. No generated file was edited by hand.

## Validation

| Command | Outcome |
|---------|---------|
| focused paired Pill Vitest run (`bunx vitest run Pill`) | 2 files, 10 tests pass |
| `cargo test --manifest-path packages/render/Cargo.toml --lib --quiet` | 335 tests pass |
| focused contrast test (`solid_tone_surface…across_themes`) | 1 test passes |
| `cargo check --manifest-path packages/contracts/components/Cargo.toml` | pass |
| `cargo check --manifest-path packages/gpui/preview/Cargo.toml` | pass |
| `effigy ir:build` / `effigy ir:check` | pass; generated artifacts current |
| `effigy test:core` | 765 tests pass (includes the Pill style evidence) |
| `effigy test:components` | 347 files, 2828 tests pass |
| `effigy check:svelte` | 0 errors; 8 baseline warnings in 5 files |
| `effigy react:build` | pass |
| `effigy test:parity` | 6 files, 365 tests pass |
| `effigy check:gpui` | pass; render 335 and node backend 19 tests pass |
| `effigy regressions:native` | 49 tests pass |
| `effigy test:web-pack-install` | 7 files, 15 tests pass (includes packed Pill consumers) |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | clean (post-commit) |

## Orchestrator review round 1 (changes requested, resolved)

1. **Web `subtle` computed fully transparent.** The pre-existing subtle rule
   defined `--poodle-pill-fill` in terms of itself — a custom-property cycle
   that invalidates the declaration. Fixed by introducing the non-recursive
   `--poodle-pill-tint-fill` base: tone and custom-accent recipes assign it,
   `--poodle-pill-fill` defaults to it, and subtle derives
   `color-mix(in srgb, var(--poodle-pill-tint-fill) 50%, transparent)` from
   it. The contract's token tables now document the derivation.
2. **Solid + neutral tone + custom accent web recipe.** While pinning the
   style evidence, the new tests surfaced that
   `[data-appearance="solid"][data-tone="neutral"]` outranked
   `[data-accent="custom"]`, so a custom-accent solid pill with the default
   neutral tone got the neutral `text.primary` background instead of the
   contract's accent-base 45/55 mix (which the Rust renderer already
   produced). Added an explicit
   `.poodle-pill[data-appearance="solid"][data-accent="custom"]` rule so the
   accent replaces the tone base as the contract states.
3. **Durable style evidence.** `packages/core/test/pill-appearance-styles.test.ts`
   models the pill.css cascade, resolves the `var()` chain with cycle
   detection, and pins tint preservation, subtle derivation, the solid 45/55
   recipe with inverse foreground for every tone and custom accent, dot
   treatment, and badge typography/recipes. Reintroducing the self-reference
   fails three of its tests; the paired data-attribute tests passed
   throughout.
4. **Packed-root Pill evidence.** New package-install fixtures mount the
   packed Svelte (`PillHarness.svelte` + `PillPackage.test.ts`) and React
   (`PillPackageReact.test.tsx`) Pill from both packed roots across all four
   appearances, asserting `data-appearance` and the absence of `data-fill`,
   with `PillAppearance` imported from each packed root.

## Unresolved / out of scope

- `cargo test --manifest-path packages/codegen/Cargo.toml` has one
  environmental failure,
  `emitted_typescript_type_checks_with_no_framework_dependency`: the test
  shells out to `bunx --no-install tsc` and no `tsc` binary is installed at
  the repo root. The same missing binary exists on the `main` checkout, so
  this is a pre-existing environment gap, not a regression from this change.
- `cargo check` on `packages/jetstream/preview` remains blocked by the known
  sibling-checkout path collision also recorded in the `g15.035` log; the
  Jetstream scene-renderer edit mirrors the GPUI one, which compiles.
  Jetstream remains program-deferred and no Jetstream selector was run.
- Existing Svelte warnings and the future-incompat `block` /
  `proc-macro-error2` notices remain baseline; no baseline cleanup was folded
  into this PR.
- Windowed, native-visual, conformance, Jetstream, and release selectors were
  not run by scope.
- PR review and merge remain with the orchestrator.
