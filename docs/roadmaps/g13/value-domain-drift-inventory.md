# Value-Domain Drift Inventory

Status: complete (report only — nothing fixed)
Milestone: `g13.001` (drift-gate work)
Branch: `thread/g13-007-value-domain-drift`
Card: `docs/roadmaps/g13/batch-cards/007-value-domain-drift-inventory.md`
Governing refs: `docs/contracts/004-shared-control-types.md` (`T1`–`T3`),
`docs/contracts/001-working-rules.md`

## What This Is

The drift gates (`contract-prop-drift.ts`, `contract-spec-drift.ts`) check that a
prop **exists** on both sides but never that its **permitted values** agree. This
inventory compares, per component, the permitted value set of every enumerated
prop across three sides:

- **Contract** — the union in the component contract's §3 "Public Props" table,
  inline (`"a" | "b"`) or named (`ButtonTone`, `StatusTone`, `ControlSize`, …),
  resolved from `docs/` (004 first, then the component's own contract, then
  other contracts, then guides).
- **TypeScript** — the prop's type in the component's `Props`, resolved through
  `packages/svelte/components/src/types.ts`, component-local aliases, and
  `@inflatable-cookie/poodle-core`.
- **Rust** — the corresponding enum in `packages/contracts/components/src`
  (types.rs or the spec module), reached through the matching `<Name>Spec`
  field.

Run: `effigy docs:value-domain-drift` (report-only; `VALUE_DOMAIN_ENFORCE=1`
turns it into a gate). The check is **not** wired into `docs:check`.

## Counts by Classification

| Classification | Count |
|---|---|
| `contract-wider` — contract permits a value the implementation lacks | 11 |
| `impl-wider` — implementation permits a value the contract does not | 10 |
| `unresolved-type` — named type with no resolvable definition in `docs/` | 8 |
| **Total** | **29** |

Across **16 components** (21 value-domain disagreements) plus **8
unresolved-type findings** (7 components). 128 components / 447 enumerated props
were compared; 47 object-typed named props were skipped as non-enumerated (out
of scope), 33 props have no TS side, 76 have no Rust side, 5 have
non-comparable TS types (DOM types such as `HTMLButtonElement["type"]`).

## Findings

Sets are sorted; `null`/`undefined` absence markers are stripped. Rust enum
variant names project to literals via kebab-case (`TopStart` → `top-start`);
cases where that projection differs from the web literal are **spelling
divergences** — reported, not forgiven, for orchestrator triage (see "Method").

| # | Component | Prop | Side | Class | Contract set | Impl set | Contract-only | Impl-only |
|---|-----------|------|------|-------|--------------|----------|---------------|-----------|
| 1 | `box` | `overflow` | ts | contract-wider | `auto, hidden, scroll, visible` | `clip, hidden, visible` | `auto, scroll` | `clip` |
| 2 | `box` | `overflow` | rust | impl-wider | `auto, hidden, scroll, visible` | `auto, clip, hidden, scroll, visible` | — | `clip` |
| 3 | `confirm-action` | `tone` | rust | impl-wider | `danger, warning` | `danger, info, neutral, pending, success, warning` | — | `info, neutral, pending, success` |
| 4 | `dialog` | `role` | rust | contract-wider | `alertdialog, dialog` | `alert-dialog, dialog` | `alertdialog` | `alert-dialog` |
| 5 | `editable-label` | `activationMode` | rust | contract-wider | `doubleClick, enterOrSpace, programmatic` | `double-click, enter-or-space, programmatic` | `doubleClick, enterOrSpace` | `double-click, enter-or-space` |
| 6 | `empty-state` | `variant` | rust | contract-wider | `firstRun, neutral, search` | `first-run, neutral, search` | `firstRun` | `first-run` |
| 7 | `icon` | `size` | rust | contract-wider | `lg, md, sm, xl, xs` | `lg, md, sm` | `xl, xs` | — |
| 8 | `icon-button` | `tooltipPlacement` | ts | impl-wider | `bottom, left, right, top` | `bottom, bottom-end, bottom-start, left, left-end, left-start, right, right-end, right-start, top, top-end, top-start` | — | `bottom-end, bottom-start, left-end, left-start, right-end, right-start, top-end, top-start` |
| 9 | `icon-button` | `tooltipPlacement` | rust | impl-wider | `bottom, left, right, top` | `bottom, bottom-end, bottom-start, left, left-end, left-start, right, right-end, right-start, top, top-end, top-start` | — | `bottom-end, bottom-start, left-end, left-start, right-end, right-start, top-end, top-start` |
| 10 | `list-container` | `emptyVariant` | rust | contract-wider | `firstRun, neutral, search` | `first-run, neutral, search` | `firstRun` | `first-run` |
| 11 | `menu` | `placement` | ts | impl-wider | `bottom-end, bottom-start, top-end, top-start` | `bottom, bottom-end, bottom-start, left, left-end, left-start, right, right-end, right-start, top, top-end, top-start` | — | `bottom, left, left-end, left-start, right, right-end, right-start, top` |
| 12 | `menu` | `placement` | rust | impl-wider | `bottom-end, bottom-start, top-end, top-start` | `bottom, bottom-end, bottom-start, left, left-end, left-start, right, right-end, right-start, top, top-end, top-start` | — | `bottom, left, left-end, left-start, right, right-end, right-start, top` |
| 13 | `page-header` | `bannerTone` | rust | impl-wider | `danger, info, neutral, success, warning` | `danger, info, neutral, pending, success, warning` | — | `pending` |
| 14 | `pill` | `typography` | rust | contract-wider | `inherit, label` | `default, inherit` | `label` | `default` |
| 15 | `scroll-shell` | `padding` | rust | impl-wider | `md, none, sm` | `lg, md, none, sm` | — | `lg` |
| 16 | `stack` | `justify` | rust | contract-wider | `between, center, end, start` | `center, end, space-between, start` | `between` | `space-between` |
| 17 | `stack` | `overflow` | rust | impl-wider | `clip, hidden, visible` | `auto, clip, hidden, scroll, visible` | — | `auto, scroll` |
| 18 | `status-indicator` | `typography` | rust | contract-wider | `inherit, label` | `default, inherit` | `label` | `default` |
| 19 | `tabs` | `variant` | ts | impl-wider | `block, card, pill, strip, text` | `block, card, pill, strip, text, underline` | — | `underline` |
| 20 | `tabs` | `variant` | rust | contract-wider | `block, card, pill, strip, text` | `block, card, pill, underline` | `strip, text` | `underline` |
| 21 | `time-ago` | `typography` | rust | contract-wider | `body, inherit` | `default, inherit` | `body` | `default` |

### Reading the spellings

Findings where the two sets differ only in **spelling** of the same concept
(the Rust enum serialization vs the web literal) are still reported because a
native renderer would emit the Rust spelling:

- #4 `alertdialog` vs `alert-dialog` — `DialogKind::AlertDialog` kebab-projects
  to `alert-dialog`; the contract/TS literal is the ARIA value `alertdialog`.
- #5 `doubleClick`/`enterOrSpace` vs `double-click`/`enter-or-space` —
  `EditableLabelActivationMode` variants kebab-project; the contract/TS
  literals are camelCase.
- #6, #10 `firstRun` vs `first-run` — `EmptyStateVariant::FirstRun`
  kebab-projects; the contract/TS literal is `firstRun`.
- #16 `between` vs `space-between` — `LayoutJustify::SpaceBetween` in
  `stack.rs`; the contract/TS literal is `between`.
- #14, #18, #21 `label`/`body` vs `default` — `InlineTypographyMode::Default`
  is the shared generic enum's baseline member; the contracts name the concrete
  mode (`label` for Pill/StatusIndicator, `body` for TimeAgo).

Whether each is a convention or a defect is a triage decision, not something
this report guesses.

## Per-Component Summary

| Component | Findings | Skew |
|-----------|----------|------|
| `box` | 2 | `overflow` — box.md restates `OverflowMode` with `auto \| scroll`; TS/Rust use `clip` (see Cross-Checks) |
| `confirm-action` | 1 | `tone` — Spec types the field as full `StatusTone` |
| `dialog` | 1 | `role` — `AlertDialog` spelling |
| `editable-label` | 1 | `activationMode` — camelCase vs kebab |
| `empty-state` | 1 | `variant` — `firstRun` spelling |
| `icon` | 1 | `size` — `IconSpec.size: IconSize` lacks `xs`/`xl` |
| `icon-button` | 2 | `tooltipPlacement` — contract restates `OverlayPlacement` with 4 members |
| `list-container` | 1 | `emptyVariant` — `firstRun` spelling |
| `menu` | 2 | `placement` — contract restates `OverlayPlacement` with 4 members |
| `page-header` | 1 | `bannerTone` — Spec types the field as full `StatusTone` |
| `pill` | 1 | `typography` — `InlineTypographyMode::Default` vs `label` |
| `scroll-shell` | 1 | `padding` — Spec field is `SpaceScale` (adds `lg`) |
| `stack` | 2 | `justify` (spelling), `overflow` (Rust `Overflow` adds `auto`/`scroll`) |
| `status-indicator` | 1 | `typography` — `InlineTypographyMode::Default` vs `label` |
| `tabs` | 2 | `variant` — TS adds `underline`; Rust drops `text`, adds `underline` |
| `time-ago` | 1 | `typography` — `InlineTypographyMode::Default` vs `body` |

## Button Family

After `282ce489` (Unify ButtonTone across the button family), the amendment
**holds**: `tone` is clean on all three sides for `button`, `icon-button`, and
`split-button` — contract (004, `default | danger | success | warning`),
`types.ts` `ButtonTone`, and `ButtonTone` in types.rs all agree. No button
`tone` finding exists.

`variant` is clean on the web (contract inline union = `ButtonVariant` in
types.ts = `ButtonVariant` in types.rs, after dropping the documented legacy
variant — see below).

### Documented exception applied

`ButtonVariant::Danger` (types.rs) is retained for backward compatibility only
and is **not** part of the authored vocabulary
(`docs/contracts/004-shared-control-types.md`). It is filtered from the Rust
`ButtonVariant` set before comparison, so `button.variant`,
`icon-button.variant`, and `split-button.variant` compare clean. This is the
only such exception; it is cited in the script.

## Unresolved-Type Findings

Named types referenced by contracts with **no resolvable definition anywhere in
`docs/`** (no `type X = …` code block, no `` `X = …` `` / `` `X: …` `` inline
form, no `` - `X`: `…` `` bullet). These are shared types the docs fail to
define — the same class of gap `004` was created to close for `ButtonTone`.

| Component | Prop | Type | Also defined in |
|-----------|------|------|-----------------|
| `color-picker` | `defaultMode` | `ColorInputMode` | `types.ts` (`"hex" \| "rgb" \| "hsl"`) |
| `dock-region` | `edge` | `DockEdge` | `types.ts`, types.rs |
| `dock-region` | `sizing` | `DockSizing` | `types.ts`, types.rs |
| `dock-region` | `collapsedPosture` | `DockCollapsedPosture` | `types.ts`, types.rs |
| `dock-region` | `emphasis` | `DockEmphasis` | `types.ts`, types.rs |
| `fader` | `automation` | `AudioAutomationState` | `packages/core` (`"none" \| "touched" \| "latched" \| "writing" \| "read"`) |
| `knob` | `automation` | `AudioAutomationState` | same |
| `xy-pad` | `automation` | `AudioAutomationState` | same |

The dock-region contracts state these unions only in the props-table **notes**
column (prose), which this check deliberately does not parse as a definition
(no guessing). `ColorInputMode` and `AudioAutomationState` have no docs
definition at all. All eight are enumerated types the docs should define —
candidates for a 004-style shared-type amendment.

## Cross-Checks Surfaced

Two shared types are restated **differently across contracts**, which the
per-component resolution surfaces as impl-wider findings:

- **`OverlayPlacement`** — three conflicting docs restatements: `hover-card.md`
  (12 members, matches impl), `icon-button.md` (`top | bottom | left | right`),
  `menu.md` (`top-end | top-start | bottom-end | bottom-start`). Impl
  (`OverlayPlacement` in types.ts / types.rs) has all 12. `menu.placement` and
  `icon-button.tooltipPlacement` therefore read impl-wider against each
  component's own restatement. This is the `ButtonTone` fragment pattern,
  still open for a second type.
- **`OverflowMode`** — `box.md` restates it as
  `visible | hidden | auto | scroll`; `stack.md` as `visible | hidden | clip`;
  `types.ts` as `visible | hidden | clip`; the Rust `Overflow` enum has all
  five. `box.overflow` (findings #1–2) and `stack.overflow` (#17) follow.
- **`ButtonTone` in `docs/guides/svelte-developer-guide.md`** is stale
  (`default | danger`). It does not affect results — 004 outranks guides in
  resolution — but the guide's type block contradicts 004 and should be
  refreshed. Recorded in `PAPERCUTS.md`.

## Method

1. `bun install`, `effigy docs:lint`, `git diff --check` — all exit 0.
2. Contract §3 props tables parsed with the same helpers as the sibling drift
   scripts (escaped-pipe aware); type cells classified as enumerated only when
   every member is a string/number literal or a named type.
3. Named types resolved per component: 004 → own contract → other contracts
   (deterministic by file) → guides. A named type that docs cannot resolve is
   `unresolved-type` **unless** it resolves to a function/object/interface on
   the TS side, in which case the prop is non-enumerated and out of scope
   (callbacks/objects). `Snippet`-typed props are excluded as framework idiom
   (same exclusion as `contract-prop-drift.ts`).
4. Rust side resolved through `<Name>Spec` fields; unit-variant enums project
   via kebab-case; `ButtonVariant::Danger` dropped per 004 (documented).
5. `null`/`undefined` stripped on all sides. Array-wrapped literal unions
   (`("icon" | "count")[]`) compare their inner set.
6. One finding per (component, prop, side) whose sets differ; the primary
   classification is `contract-wider` when the contract has any impl-lacking
   member, else `impl-wider`. Both difference vectors are always listed.
7. Findings are report-only: the script exits 0 by default, 1 only under
   `VALUE_DOMAIN_ENFORCE=1`. The Effigy selector `docs:value-domain-drift` runs
   it; `docs:check` is unchanged.

## Out of Scope (per card)

- No violation fixed; no contract, component source, CSS, or Rust edited.
- CSS delivery (button family) is `g13-b006`.
- Non-enumerated props (strings, numbers, booleans, callbacks, objects).
- Enforcement / CI wiring / `docs:check` inclusion.
