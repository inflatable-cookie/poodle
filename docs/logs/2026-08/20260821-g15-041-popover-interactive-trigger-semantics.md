# g15.041 — Popover interactive trigger semantics

Date: 2026-08-21
Card: `docs/roadmaps/g15/041-popover-interactive-trigger-semantics.md`
Handoff: `docs/handoffs/20260821-151745-g15-041-popover-interactive-trigger-semantics.md`
Parent: `docs/roadmaps/g15/032-review-composition-navigation-overlays.md`
PR: #59 (`g15-041-popover-interactive-trigger-semantics`)

## Outcome

Popover's interactive trigger composition is now semantic in server output and
hydrated DOM. Core authors one framework-neutral `PopoverTriggerState`
(`expanded` / `controls` / `disabled`) and returns it from `popoverParts`
beside the part attributes. In interactive mode the Svelte snippet and React
render prop receive that payload, and the caller applies it to the real
control: exactly one operable trigger owns `aria-expanded` (`"false"` closed,
`"true"` open), `aria-controls` matching the rendered surface id while open,
and the effective disabled state, while the wrapper stays a roleless,
untabbable layout/event host. The default wrapper mode is unchanged.

Button gains `controls` beside `ariaExpanded` in both web packages
(`aria-controls` when non-null) and `ButtonSpec::controls` / `with_controls`
in `poodle-specs`, projected to the existing `NodeA11y.controls` field on the
shared render path — the same seam IconButton already used. All six production
composites (HistoryCenter, MessageCenter, UpdateCenter in Svelte and React)
thread the payload into their actual Button/IconButton/native-button triggers;
badges and progress decoration stay non-interactive.

## Change class

**Breaking, pre-1.0, operator-approved** (2026-08-21) public API migration.
No alias, overload, deprecated twin, runtime detection, or silent fallback
remains.

- **Packages changed:** `@inflatable-cookie/poodle-core`,
  `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-react`,
  `poodle-specs`, `poodle-render`; structural evidence in `poodle-gpui-preview`;
  internal preview/test fixtures.
- **Public-intent entry points:** core adds `PopoverTriggerState` and the
  `popoverParts` `triggerState` payload (additive); both web roots re-export
  the type and change the interactive `trigger` signature (breaking) and add
  Button `controls` (additive); `ButtonSpec` gains `controls` (breaking for
  direct struct literals, additive for builder callers); `poodle-render`
  projects it (behavioral); `poodle-gpui` consumes the general node field
  structurally (no new API, no platform-AT claim).
- **Compatibility:** React rejects the old static-node interactive trigger at
  compile time. Svelte's discriminated snippet typing rejects a wrongly-typed
  payload and wrong-branch usage, but TypeScript function assignability makes
  a zero-argument snippet assignable to `Snippet<[PopoverTriggerState]>`, so
  the old Svelte shape still compiles and silently drops the disclosure
  semantics — Svelte migration is enforced by search and review. The operator
  accepted this on 2026-08-21 (card stop condition 1) and the contract §3
  migration note records it. Direct `ButtonSpec` struct literals must
  initialize `controls`; none exist in this repository.
- **Downstream re-check:** every out-of-repo interactive Popover trigger must
  become state-aware and apply all three payload fields to the real control.
  Read-only `~/Dev/projects` search on 2026-08-21 found two source consumers,
  both Svelte, both the old zero-arg shape:
  `soundcheck-library` (`packages/library-svelte/src/PluginInspector.svelte`,
  an IconButton info trigger) and `bovine-accelerator-desktop`
  (`src/components/ProjectPopover.svelte`, an IconButton trigger that already
  passes `expanded={open}` — replaced by the payload). `figmatic`'s only hit
  is a built `dist/` bundle, not source. External repositories were not
  edited; their migrations are `trigger(state)` plus
  `expanded`/`controls`/`disabled` on the IconButton.

## Implementation

- Contracts first: `popover.md` §3 (trigger modes, `PopoverTriggerState`,
  migration), §4 part output (payload beside parts), §6 accessibility and
  focus; `button.md` §3 props and portable-spec table, §6 semantics.
- Core: `PopoverTriggerState` exported from the package root; `popoverParts`
  computes it in both modes (`controls` is `null` closed, surface id open,
  preserving the conditional `aria-controls` contract).
- Svelte `Popover` props are a discriminated union (LicenceActivation
  precedent): default mode keeps the zero-arg snippet; interactive mode
  requires `Snippet<[PopoverTriggerState]>`. React mirrors with
  `trigger?: ReactNode` vs `trigger: (state) => ReactNode`. Neither adapter
  clones children, walks the DOM, mutates attributes, or depends on an effect
  for semantics. React's surface id moved to framework-native `useId()` and
  Svelte's to framework-native `$props.id()`; both carry the server identity
  through hydration without a shared counter or post-mount repair. No
  repository-wide id change was needed. Focus restoration keeps the existing real-descendant
  lookup, which is permitted for focus only.
- Svelte `defaultOpen` seeding moved from `$effect.pre` to `$state(untrack(...))`:
  Svelte's server runtime strips effects, so effect-seeded initial state made
  `defaultOpen` server output impossible. Client behavior is identical (the
  effect applied the same value before first render).
- Rust: `ButtonSpec.controls` + `with_controls` + `Default` None;
  `poodle-render` projects `el.a11y.controls = spec.controls.clone()` beside
  the `aria_expanded` projection.
- Preview drift checkers parsed only `interface Props`; a shared
  `unionPropsBody()` fallback now extracts members from the discriminated
  union shape (also closing a latent gap for LicenceActivation).
- Generated docs artifacts regenerated only through the canonical exporters
  (`bun scripts/export-component-docs.ts` per preview, `effigy react:docs`).

## Evidence

- Core (bun): `triggerState` follows open/closed/disabled in both modes;
  retained machine and part tests unchanged. 767 tests pass (`test:core`).
- Svelte (vitest, `svelte-components`): interactive wrapper roleless and
  untabbable with the relationship on the real control; click toggling
  repeated; outside-`mousedown` and Escape close with focus returned to the
  real control; controlled and uncontrolled; disabled reaches the real control
  and blocks open; default-mode regression (wrapper role/tabindex/ARIA +
  Enter); real server markup hydrates without changing the advertised control
  or surface id. Package suite: 169 files / 1151 tests pass.
- Svelte SSR (new bounded `svelte-components-ssr` vitest project,
  `svelte/server` `render`): closed interactive output has
  `aria-expanded="false"` and no `aria-controls` on the real control;
  `defaultOpen` output has `aria-expanded="true"` with `aria-controls`
  string-equal to the rendered surface id in the same HTML — no post-mount
  repair; independent server renders reuse the framework identity instead of
  advancing shared process state; disabled and both default-mode states covered.
- React (vitest): same client matrix plus `renderToString` server evidence
  and a `hydrateRoot` check that the server-advertised `aria-controls` id
  survives hydration unchanged with zero console errors. Package suite:
  162 files / 1105 tests pass.
- Button `controls`: present/absent `aria-controls` tests in both web
  packages; render-crate test proves set/None projection onto
  `node.a11y.controls` (365 tests pass); mounted headless GPUI regression
  proves a mounted Button carries its controls target through the real
  backend — structural only, no platform-AT claim (54 pass).
- Packed roots (`test:web-pack-install`): both packed packages export
  `PopoverTriggerState`, mount the state-aware Popover, and project Button
  `controls`; open trigger `aria-controls` equals the surface id.
- MessageCenter tests in both runtimes assert `aria-controls` equals the
  dialog surface id on the actual IconButton.

## Audit

`specimen-catalogue-audit.md` revision 14: Popover returns to `A / A / A`
with disposition `keep`. Totals recounted mechanically: Svelte A 89 / C 44;
React A 102 / C 47; worst-of-three A 66 / C 52; `keep` 56;
`contract/runtime-blocker` 0. The operator authorized the reviewed fixes and
merge on 2026-08-21 without requesting another paired live-route pass; this is
recorded as an explicit gate disposition, not as new visual evidence.

## Changed routes for operator review

- Svelte preview: `http://127.0.0.1:4175/#components/popover` — both examples
  now compose real Poodle Button triggers driven by the state payload.
- React preview: `http://127.0.0.1:4181/#components/popover` — the paired page.

The operator authorized merge on 2026-08-21 after the orchestrator review and
final identity/API fixes. No separate renewed live-route pass is claimed.

## Validation

Headless only. No `*-windowed`, `test:native-visual`, Jetstream,
visual-conformance, or release selector ran.

- `effigy test:core` — 767 passed
- `effigy test:components` — 355 files / 3049 passed (includes the SSR project)
- `effigy check:svelte` — 0 errors
- `effigy react:build` — passed
- `effigy ci:rust` — exit 0
- `effigy check:gpui` — exit 0
- `effigy regressions:native` — 54 passed
- `effigy test:web-pack-install` — passed (packs, installs, runs fixture suite)
- `effigy catalogue:check` — passed
- `effigy docs:check` — passed
- `effigy qa` — worker branch passed before orchestrator review fixes. Two
  final clean-tree reruns passed every preceding component, package, type, and
  docs step, then hit the recorded `gate-tree-guard --compare` missing-snapshot
  infrastructure fault; focused final gates and CI carry the final-SHA evidence.
- `git diff --check origin/main...HEAD` — clean
