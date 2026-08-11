# 11-g13-020-active-edge-strip-consolidation

Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/020-tabs-active-edge-and-strip-consolidation.md`
Branch: `thread/g13-020-active-edge-strip-consolidation`
Status: committed, unpushed at write time; pushed with `git push -u origin …`

## What shipped

R1 — `activeOutline: boolean` replaced by a shared `ActiveEdge` type
(`"none" | "outline" | "underline"`, default `"none"`), defined once in
`docs/contracts/004-shared-control-types.md` and consumed by both `tabs.md`
and `navigation-menu.md`. Delivered in contract, both web runtimes, both
Specs (`TabsSpec`, `NavigationMenuSpec`), and `poodle-render`.

R2 — the `strip` variant is deleted everywhere (union is `card | pill |
block`). Block absorbed the listed behaviours: list inline padding
(`--poodle-tabs-strip-inline-padding` → `--poodle-tabs-block-inline-padding`,
values kept), item hover background (`surface-hover` 50%), close-button
margin-end (`--poodle-tabs-strip-close-margin-end` →
`--poodle-tabs-block-close-margin-end`), and strip's vertical-orientation
handling (list border-right + `overflow: visible`, icon-only tab sizing,
first/last-child 0.75rem padding). The strip-specific
`activeOutline`/`activeFill` suppression rules are deleted — the underline
indicator now lives on `[data-active-edge="underline"]`, so the edge axis
cannot self-conflict. Block retains separators (per-side border color
overrides in the Rust renderer so the outline does not clobber them),
full-width behaviour, and rounded-corner handling.

R3 — `bordered` defaults to `false` in Svelte, React, and `TabsSpec`
(`is_bordered: false`); the spec test asserts the flip.

## Diff table

| File | Change |
|---|---|
| `docs/contracts/004-shared-control-types.md` | `ActiveEdge` type added alongside `ActiveFill` |
| `docs/contracts/components/tabs.md` | union `card\|pill\|block`, `activeEdge` prop, `bordered` default false, edge-axis §8 section, strip tables → block absorption, specimens |
| `docs/contracts/components/navigation-menu.md` | `activeEdge` prop, outline/underline §8 tables, specimens |
| `packages/core/src/styles/tabs.css` | strip rules deleted; underline edge rules; outline keyed on `[data-active-edge="outline"]`; block absorbs inline padding / hover / close margin-end / vertical handling; strip custom props renamed to `--poodle-tabs-block-*` |
| `packages/core/src/styles/navigation-menu.css` | `[data-active-outline]` → `[data-active-edge="outline"]`; underline edge rules |
| `packages/svelte/components/src/{Tabs.svelte,NavigationMenu.svelte,types.ts}` | `activeEdge` prop (default `"none"`), `bordered=false`, `data-active-edge`, `ActiveEdge` type, union without `strip` |
| `packages/react/components/src/{Tabs.tsx,NavigationMenu.tsx,types.ts}` | same as Svelte |
| `packages/contracts/components/src/{tabs.rs,navigation_menu.rs,lib.rs,types.rs}` | `ActiveEdge` enum (default `None`), `active_edge` fields + `with_active_edge` builders, re-export, `is_bordered: false`, `TabVariant` docs |
| `packages/render/src/{tabs.rs,navigation_menu.rs}` | `apply_active_edge` helper (outline + underline, orientation-aware) on card/pill/block; nav-menu outline/underline; block separators via per-side color overrides; tests |
| `packages/gpui/preview/src/specimens/{tabs.rs,navigation_menu.rs}` | block + outline/underline specimens, vertical underline, nav-menu underline; TabStrip groups on the Tabs page replaced by Tabs block+underline |
| `packages/jetstream/preview/src/specimens/{tabs.rs,navigation_menu.rs}` | same mirror; block + each edge value groups |
| `packages/{svelte,react}/preview/src/**/{TabsSpecimen.*,NavigationMenuSpecimen.*}` | block + each `activeEdge` value, panel specimens pass `bordered` explicitly, no strip specimens; Svelte/React labels identical |
| `packages/svelte/preview/src/component-docs.ts` | `tabs`/`navigation-menu` entries: `activeEdge` row, `bordered` row |
| `packages/{svelte,react}/components/test/interactions.test.*` | defaults asserted (`data-active-edge="none"`, `data-bordered="false"`), block+underline emission |

## Compile-breaking callers edited (beyond the writable list)

Recorded per the worker rules; all in-repo, none are consumer repositories:

- `packages/svelte/preview/src/specimens/AppHeaderSpecimen.svelte` +
  `packages/react/preview/src/gallery/specimens/AppHeaderSpecimen.tsx` —
  `variant="strip" activeOutline` (2 sites each) → `variant="block"
  activeEdge="outline"` (faithful port of strip+outline).
- `packages/svelte/components/src/DockRegion.svelte` +
  `packages/react/components/src/DockRegion.tsx` — `tabVariant = "strip"`
  default (type error against the new union) → `"block"`; the Tabs renders
  hardwire `activeEdge="underline"` so the dock tabs keep the strip indicator.
- `packages/core/src/styles/dock-region.css` — the dock strip's
  `[data-variant="strip"]` selectors (horizontal list border suppression,
  right-edge vertical indicator flip) → `[data-variant="block"]` /
  `[data-active-edge="underline"]`. Without this, the right-edge dock's
  vertical indicator would silently flip sides.

`TabStrip` (component, `docs/contracts/components/tab-strip.md`,
`TabStripSpec`, `render/src/tab_strip.rs`, and both `tab_strip.rs` specimen
pages) is untouched — verified by `git status` (no `tab_strip` paths in the
diff).

## Findings / deviations (card assumptions vs repository reality)

1. **Block list width in the contract was stale.** `tabs.md` §8 said `width:
   fit-content; max-width: 100%`; CSS and `render_block` are `width: 100%`.
   Fixed the contract table to `width: 100%` (block retains full-width
   behaviour).
2. **`docs:check` regenerates `packages/react/preview/artifacts/component-docs.json`**
   and it carries unrelated pre-existing drift (the stopped b009 dialog
   `initialFocus` rows). Restored with `git checkout` — same handling as
   g13-013/016/014/017. The writable source (`component-docs.ts`) carries the
   new props; the artifact refreshes on the next `docs:check`.
3. **Jetstream preview crate cannot build in this environment** — its
   `jetstream-poodle` consumer dependency path-points `poodle-node` at
   `/Users/tom/Dev/projects/poodle-wt/poodle/…`, which does not exist, and the
   consumer repo is out of bounds. Pre-existing (g13-017 recorded the same);
   the preview crates are not in the step-8 validation list. The jetstream
   specimen edits are type-level mirrors of the gpui ones, which compile clean
   (`cargo check -p poodle-gpui-preview`).
4. **Visual sweep: 56 failing pairs vs the g13-016 set of 54.** `tabs`,
   `navigation-menu`, and `app-header` pairs are clean (both runtimes changed
   identically). The only new pair is `message-center` (1.735% / 3.405%
   pixels), which no touched file can affect (zero message-center files in the
   diff; Svelte/React specimens structurally identical; timestamps are
   `Date.now()`-relative). Classified pre-existing, recorded, no stop.
   `dock-region` remains in its pre-existing failing class with a moved
   magnitude (0.443%/0.436% → 0.473%/0.644%): DockRegion consumes Tabs, and
   the strip→block+underline change (R2) applies identically to both runtimes.
5. **Mutual exclusion is by construction, not by test.** `outline` and
   `underline` are members of one enum; the web tests assert the root emits a
   single `data-active-edge` value and the CSS keys off distinct values — no
   suppression rules exist anywhere.

## Validation (step 8)

| Command | Exit | Notes |
|---|---|---|
| `effigy test:components` | 0 | 44 files / 870 tests (867 + 3 new) |
| `effigy test:parity` | 0 | 2 files / 163 tests |
| `effigy docs:lint` | 0 | — |
| `effigy docs:contract-drift` | 0 | every documented public prop implemented in Svelte |
| `effigy docs:spec-drift` | 0 | every documented prop reaches poodle-specs |
| `effigy docs:value-domain-drift` | 0 | no tabs/navigation-menu/activeEdge/bordered finding (the former tabs `variant` strip gap is resolved by the union change) |
| `cargo test -p poodle-render` | 0 | 164 tests (160 + 4 new: underline edges, vertical underline, separators-under-outline) |
| `cargo test -p poodle-specs` | 0 | 241 tests |
| `cargo check -p poodle-gpui-preview` | 0 | clean |
| `effigy docs:check` | 0 | incl. `vite build` |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | rewritten by docs:check → restored, not committed |
| `git diff --check` | 0 | clean |
| `git status --porcelain` | — | only the writable paths + the caller edits above + this log |

Visual enumeration (`effigy visual:report`, tier=sweep): 308 pairs compared,
56 failing — 54 pre-existing (byte-identical slug set to g13-013/016) +
`message-center` (pre-existing source asymmetry, see Finding 4). No baseline
refreshed.

## Acceptance criteria

- [x] `ActiveEdge` defined once in `004`, consumed by both contracts; no
  `activeOutline` remains anywhere (contracts, CSS, web runtimes, Specs,
  renderer, specimens, tests — verified by repo-wide grep).
- [x] Variant union is `card | pill | block` in contract, both web runtimes
  and Rust; no `strip` remains in Tabs.
- [x] `block` + `activeEdge="underline"` reproduces strip's indicator; block
  retains separators, full-width, and correct rounded-corner handling.
- [x] Strip-specific suppression rules are gone.
- [x] `bordered` defaults to `false`, asserted in all three surfaces (Svelte,
  React, `TabsSpec`).
- [x] `TabStrip` untouched.
- [x] Specimens cover every `activeEdge` value in all four runtimes.
- [x] No consumer repo edited; no baseline refreshed.
- [x] All step-8 commands exit 0.
- [x] Batch log records commands, exit states, the diff table, and the
  compile-breaking callers edited.

## Stop conditions

None triggered. `block` reproduces the indicator via variant-agnostic
`[data-active-edge="underline"]` rules (no strip-specific rule reintroduced);
no non-Tabs component broke (DockRegion/AppHeader consumed Tabs and were
ported in-repo); the three runtimes agree on every default; the only new
visual pair (`message-center`) is provably independent of this diff.
