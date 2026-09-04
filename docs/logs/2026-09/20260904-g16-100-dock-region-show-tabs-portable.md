# g16.100 — DockRegion `showTabs` Portable Spec And Baseline Closeout

Status: complete
Date: 2026-09-04
Card: `docs/roadmaps/g16/100-dock-region-show-tabs-portable.md`
Handoff: `docs/handoffs/20260904-170000-g16-100-dock-region-show-tabs-portable.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`packages/contracts/components/src/dock_region.rs`,
`packages/render/src/dock_region.rs`,
`packages/svelte/preview/scripts/react-prop-drift.ts`
Branch: `feature/g16-100-dock-region-show-tabs-portable`
Base: `origin/main` at `ce60eb7cbc8222ff10f2852454ae4664266544b5`

## Outcome

`showTabs` is now a portable DockRegion capability: the contract tables it,
`DockRegionSpec` carries `show_tabs`, `poodle-render` honours it on every
strip-drawing mode, the React shell matches the Svelte reference, and the
React drift baseline closes with zero `needs-decision` entries.

### Contract

- `showTabs` (`boolean`, default `true`) moved from the web-only tranche note
  into the §Public Props table with the card's semantics: when `false`, the
  strip omits panel tabs but keeps the collapse toggle when
  `showCollapseToggle` is true; the body still renders the active panel;
  keyboard tab switching is unavailable because there are no tabs.
- The tranche note now cites only `tabVariant`; the "moved, not duplicated"
  invariant holds — the prop appears in exactly one place.
- `packages/svelte/preview/scripts/contract-prop-drift.ts`: `showTabs` left
  the `dock-region` `svelteOnly` register with the move (the register has no
  stale-entry ratchet, but a svelteOnly entry for a tabled prop would be a
  lie). `tabVariant` and the five pass-throughs stay until g13.014.

### Rust spec and render

- `DockRegionSpec` gains `pub show_tabs: bool` (default `true`) with
  `with_show_tabs`. All construction sites in the repo use `::new()` plus
  builders, so the addition breaks nothing (Jetstream included).
- `poodle-render` honours the field on every mode that draws a strip:
  - Expanded flexible: with `show_tabs=false` the strip is not built at all —
    no tab nodes, no `on_tab_change` activations, no `poodle.dock-panel` drag
    sources or drop targets. The collapse toggle stands alone in the strip's
    place (matching the Svelte `__edge-toggle` posture), the body still
    renders, and the region no longer carries the `TabList` a11y role.
  - Collapsed icon-strip (side and top/bottom): the strip keeps its toggle
    and emits no tab children.
  - Static and hidden postures draw no tabs today and are unchanged.
  - Gating uses `is_collapsible`, consistent with every existing toggle site
    in the file. `show_collapse_toggle` is declared on the spec but not yet
    consumed anywhere in render — a pre-existing gap outside this card's
    boundary, unchanged here.
- Node-inventory test `show_tabs_false_emits_no_tab_nodes_but_keeps_toggle_
  and_body` covers expanded, side icon-strip, and top icon-strip: zero
  `dock-tab-*` nodes, zero interaction registrations, toggle present, body
  presence asserted, no `TabList` role.
- GPUI specimen: one new "Collapse and edge placement" row renders
  `show_tabs=false` with `collapsible=true` (instance `show-tabs-off`);
  existing rows keep the default `true`.

### React

- `DockRegion.tsx` ports `showTabs` with the Svelte default (`true`) and
  behavior: the strip (tabs + toggle) renders only when `showTabs`, an
  `{:else}` edge-toggle replaces it when `collapsible && showCollapseToggle`,
  the body always renders, the collapsed vertical/horizontal icon-strips
  keep their strip and toggle while dropping tabs, and the root carries
  `data-show-tabs="false"` when false. `showTabs` joined the compact
  ResizeObserver effect deps so unmounting the tabs div disconnects it.
- Focused tests in `packages/react/components/test/DockRegionShowTabs.test.tsx`:
  default renders tabs; `showTabs={false}` renders no tabs, no strip, the
  `data-show-tabs` marker, the collapse toggle, and the body; the collapsed
  icon-strip keeps its toggle.

### Baseline closeout

- Deleted the `dock-region` `needs-decision` entry from `BASELINE` in
  `react-prop-drift.ts`.
- Re-kinded `tree` and `order-by` to `framework-idiom` with the card's exact
  reason: "React change callback paired with the Svelte `$bindable` state
  (working rules, Runtime Parity Authority); zero consumer use 2026-09-04".
- The register now holds 15 `framework-idiom` entries and zero
  `needs-decision` entries; accepted-delta components went 16 → 15.

### Generated evidence repin (mechanical)

Touching `packages/render` and `packages/contracts` invalidates the Nucleus
receipt cohort's source pinning (`SOURCE_PATHS` in
`scripts/nucleus-parity-receipts.ts`), so `docs:check` failed on the stale
pin. Following the g16.092 precedent, from committed `fe3762696`:
`effigy regressions:native` re-emitted all 29 receipts
(`POODLE_NUCLEUS_RECEIPT_DIR={repo}/target/nucleus-receipts`); the cohort was
copied over `docs/roadmaps/g16/nucleus-parity-receipts/` and the manifest
`resolution.source_commit` advanced. Receipts differ from the prior cohort
only in `source_commit` — no mounted evidence semantics changed — and the
generated ledger was rewritten by `bun scripts/parity-evidence-ledger.ts
--write` (one DockRegion cell: the React focused test now names
`DockRegionShowTabs.test.tsx`).

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Portable field is honoured | `show_tabs=false` still emits tab nodes | `cargo test -p poodle-render` `show_tabs_false_emits_no_tab_nodes_but_keeps_toggle_and_body`: zero tab nodes, toggle present — passed |
| Web parity | React `showTabs={false}` still renders tabs | `DockRegionShowTabs.test.tsx`: `queryByRole("tab")` null, strip null, toggle and body present — 3/3 passed |
| Contract moved, not duplicated | prop appears in both the table and the web-only note | single table row; note cites only `tabVariant`; `docs:contract-drift` and `docs:spec-drift` green |
| Baseline is clean | any `needs-decision` entry remains | `docs:react-prop-drift` OK, 176 checked, 15 accepted-delta components, zero `needs-decision` kinds |
| Ratchet still bites | re-add `showTabs` as `svelteOnly` | temporarily restored the old `dock-region` entry: gate FAIL "baselined Svelte prop(s) no longer drift (delete from BASELINE): showTabs"; reverted |

## Validation

- `effigy docs:react-prop-drift` — OK (176 checked, 0 skipped).
- `effigy docs:check` — exit 0 (includes contract/spec/prop/callback drift,
  ledger validation, docs lint and build).
- `effigy test:contracts` — exit 0.
- `cargo test -p poodle-render` — 637 passed, 0 failed (includes the new
  inventory test).
- `effigy regressions:native` — 203 passed, 0 failed; 29 receipts re-emitted
  at `fe3762696`.
- Focused React: `bun run vitest run
  packages/react/components/test/DockRegionShowTabs.test.tsx` — 3 passed.
- `effigy ci:web` — exit 0 on committed `b2f7aa7b0` (23-step board: 1265
  core tests, 3684 component tests across 382 files, package installs,
  surface audits, writer gates with a clean tree-guard compare).
- `git diff --check origin/main...HEAD` — clean.

## Limits

- No Svelte component was edited; Svelte remains the reference
  implementation and already carried `showTabs`.
- Jetstream is the deferred backend: the spec field is additive and nothing
  there honours `show_tabs` yet (no compile break — no exhaustive struct
  literals exist).
- `show_collapse_toggle` render honouring remains a pre-existing declared
  gap, unchanged by this card.
- Tab drag, DockRegion sizing, and the Tabs component are untouched.
- Reserved coordinator paths (`docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`) were
  untouched.
