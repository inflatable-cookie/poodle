# g15.036 — Pill Appearance Semantics

Status: **complete** — PR #45 merged 2026-08-19; Pill's duplicate `fill` axis
was removed before v0.2.0
Depends on: `g15.035` (complete; PR #44)
Blocks: `g15.012`, `g15.013`
Parallel with: `g15.022` — no shared mutable component or specimen files
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/004-shared-control-types.md`,
`../../contracts/components/pill.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`

## Outcome

Pill has one honest visual-treatment axis:

```ts
type PillAppearance = "tint" | "solid" | "subtle" | "badge";

appearance?: PillAppearance; // default: "tint"
```

The temporary `fill` prop added by `g15.035` is removed from Pill only.
Callout and RemediationBanner retain the shared
`fill: "tint" | "solid"` API. Pill's current default appearance moves to the
new `tint` value without changing its visuals. The existing `solid` value
becomes the opaque high-contrast treatment its name promises.

This is a deliberate pre-v0.2 correction, not a compatibility migration. Do
not retain a deprecated Pill `fill` prop, alias, precedence rule, or silent
fallback.

## Decision

- `appearance="tint"` owns today's default tone-tinted shell.
- The implicit appearance changes from `solid` to `tint`; default rendered
  output remains observably unchanged.
- `appearance="solid"` owns the `g15.035` opaque recipe:
  - non-neutral background: 45% tone base plus 55% `color.text.primary` in
    sRGB;
  - neutral background: `color.text.primary`;
  - non-neutral border: raw tone base;
  - neutral border: `color.border.strong`;
  - foreground and dot/remove affordance: `color.text.inverse`;
  - a custom `accent` replaces the semantic tone base.
- `appearance="subtle"` and `appearance="badge"` keep their current visual and
  typography behavior. They do not combine with solid because appearance is
  a single mutually exclusive axis.
- `muted`, selection, disabled/removable native states, sizing, density,
  typography, title, accessible naming, and callbacks are unchanged.
- `ToneFill` remains a shared Callout/RemediationBanner type. Remove Pill from
  its named consumer list; do not delete or rename the type.

## Measured Starting Point

- PR #44 added `fill` to the Svelte and React Pill shells, `PillSpec`, shared
  CSS, contract, authored display-specimen model, generated Svelte/React/Rust
  specimens, GPUI examples, and focused tests.
- Before PR #44, `appearance="solid"` was the default but selected no dedicated
  CSS or Rust color recipe. The name described the ordinary tinted shell.
- The opaque 45/55 recipe is already implemented and contrast-tested across
  all twelve themes in `poodle-render`; this card reassigns it to Pill's
  existing appearance axis rather than inventing new color math.
- Callout and RemediationBanner genuinely need a second fill axis because
  their other props describe semantic tone and behavior, not mutually
  exclusive shell appearances. Their landed APIs are not reopened here.

## Delivery

### 1. Correct the contract and public types

- Remove Pill `fill` from the component contract, web prop surfaces,
  `PillSpec`, builders, and generated specimen schema.
- Add `Tint` to `PillAppearance` in TypeScript and Rust; make it the default.
- Document the four appearances and exact default/solid recipes.
- Update shared-control-type docs so `ToneFill` names only Callout and
  RemediationBanner as consumers.
- Record the release impact honestly: Pill `fill` is removed; `tint` is added;
  the implicit appearance value changes while default visuals do not; explicit
  `appearance="solid"` calls gain the promised saturated treatment.

### 2. Keep web and native rendering aligned

- Remove `data-fill` from both web roots. Shared Pill CSS resolves all visual
  treatment from `data-appearance`.
- Move the existing opaque CSS recipe from `[data-fill="solid"]` to
  `[data-appearance="solid"]`.
- Make tint the explicit ordinary-shell selector or a clearly documented base
  recipe. Preserve default output and current subtle/badge output.
- Update `poodle-render::pill` to branch on `PillAppearance::Solid`, not
  `ToneFill`. Preserve the same shared `solid_tone_surface` color resolver.
- Keep full Svelte/React/Rust/GPUI parity for tone, custom accent, dot, remove,
  muted, selected, and disabled states.

### 3. Teach the real axis

- Remove Pill's `fill` presentation prop and `Solid fills` generated group.
- Add `tint` to the authored appearance domain and regenerate every derived
  Svelte, React, Rust, and fixture artifact through the generator.
- Keep one compact human-facing appearances group showing tint, solid, subtle,
  and badge. Do not add an exhaustive tone-by-appearance matrix.
- Keep existing tone, size, density, typography, and realistic examples
  intact. Generated files are never edited by hand.
- Update the GPUI Pill page to teach the same four appearances.

### 4. Prove the correction

- Paired web tests prove:
  - omitting the prop emits `data-appearance="tint"`, emits no `data-fill`, and
    preserves the previous default recipe;
  - explicit tint matches the default;
  - solid uses inverse foreground for every tone and custom accent;
  - subtle and badge remain distinct and unchanged;
  - no Pill public type or DOM output contains `fill`.
- Rust spec/render tests prove the same default, four-member enum, solid color
  resolution, dot/remove treatment, and absence of a Pill fill field/builder.
- Packed-root evidence imports the corrected Pill types from both web package
  roots and the Rust public surface where existing package checks cover it.
- Add one August correction log that points to PR #44 as the temporary API
  source and records its complete removal before release.

## Acceptance

- [x] Pill exposes only `appearance` for tint/solid/subtle/badge treatment in
      Svelte, React, Rust, contracts, generated artifacts, and specimens.
- [x] Default appearance is tint and its rendered output matches the pre-PR
      #44 default treatment.
- [x] Explicit solid uses the shared contrast-safe opaque recipe in both web
      runtimes and renderer-neutral Rust.
- [x] Subtle and badge remain observably unchanged and do not participate in a
      two-axis precedence rule.
- [x] Pill has no public `fill`, `ToneFill`, `with_fill`, `data-fill`, or
      compatibility path after the correction.
- [x] Callout and RemediationBanner retain their landed `ToneFill` APIs and
      behavior unchanged.
- [x] Generated artifacts are rebuilt from the authored display model and
      `effigy ir:check` is clean.
- [x] The Pill specimen teaches the four appearances compactly in Svelte,
      React, and GPUI.
- [x] Focused component, Rust, parity, package, docs, and diff gates pass.

## Writable Scope

- `docs/contracts/004-shared-control-types.md`
- `docs/contracts/components/pill.md`
- Pill-only shells, types, styles, tests, and specimens under
  `packages/{core,svelte,react}`
- `packages/contracts/components/src/{pill,types}.rs` only where needed to
  remove Pill's dependency and add `PillAppearance::Tint`
- `packages/render/src/pill.rs` and focused tests; the existing shared solid
  color resolver may be consumed but not redesigned
- Pill authored display-specimen model entry, fixture, generated artifacts,
  and focused generator tests
- GPUI Pill specimen and generated Pill specimen artifacts
- package-facing generated component docs affected by Pill
- one August correction log
- append-only `PAPERCUTS.md` for new execution friction only

Do not edit Callout or RemediationBanner implementation, global tone values,
token schema, Button, catalogue navigation/shells, unrelated components,
Jetstream integration, conformance architecture, release automation, or
`.github/workflows/`.

## Validation

Run one coherent headless round after implementation:

- focused Svelte/React Pill component tests
- focused Rust Pill spec/render and contrast tests
- focused generated-specimen and paired parity evidence
- `effigy ir:check`
- `effigy test:components`
- `effigy check:svelte`
- `effigy react:build`
- `effigy test:parity`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy test:web-pack-install`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Never run a `*-windowed`, native-visual, conformance,
Jetstream, or release selector.

## Stop Conditions

- Removing Pill `fill` would also remove or rename Callout/RemediationBanner's
  `ToneFill` API.
- Preserving current default visuals requires a compatibility alias or silent
  fallback instead of a direct `appearance="tint"` default.
- The four appearances cannot remain mutually exclusive across web and Rust.
- The existing 45/55 solid recipe fails a current theme's 4.5:1 text contrast
  evidence.
- The generated presentation model would need component behavior or runtime
  callbacks.
