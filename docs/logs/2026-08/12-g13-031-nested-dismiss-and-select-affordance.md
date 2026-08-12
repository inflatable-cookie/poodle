# 12 — g13.031 Nested Dismiss Layers, And The Ghost Select Affordance (batch log)

Branch: `thread/g13-031-nested-dismiss-and-select-affordance`
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/031-nested-dismiss-and-ghost-select-affordance.md`
Status: **COMPLETED** — dismiss-layer ancestry (Defect 1) and the ghost Select
chevron (Defect 2) implemented Svelte-first then React-mirrored, committed as
two separate reverts, every step-6 command exits 0, no baseline refreshed.

## 1. Base verification (step 1)

- Branch at `202ac00e` (dispatch commit) on
  `thread/g13-031-nested-dismiss-and-select-affordance`. ✓
- `effigy test:core` → 487 pass / 0 fail (44 files). ✓
- `effigy test:components` → 982 pass / 0 fail (69 files). ✓
- `effigy check:svelte` → 0 errors (1 + 449 files, 4 pre-existing a11y
  warnings only). ✓
- `effigy docs:lint` → exit 0 (171 component contracts). ✓
- `git diff --check` → clean. ✓

## 2. Defect 1 — `packages/core/src/dom/dismiss.ts`

- `DismissLayer` gains `parent?: DismissLayer | null`. `registerDismissLayer`
  records the layer on top of the stack at registration time — the layer the
  new one opened inside. Registration order is the ancestry, not the DOM, so
  portalling cannot break it. Optional on the interface because callers
  construct layers before registration and the pure tests set it directly.
- `resolveDismiss` (outside path) now spares every layer that contains the
  target **plus every ancestor of such a layer** (walking the recorded parent
  chain), instead of only the containing layer. Peers — layers with no parent
  link to the hit layer — still all dismiss, so the peer behaviour is
  unchanged; the four named peer tests pass unedited
  (`packages/core/test/dismiss.test.ts`).
- `handlePointerDown` / `handleKeydown` / `syncListeners` untouched: the
  snapshot-then-dispatch loop already tolerates each `onDismiss`
  unregistering its own layer.
- Why not widen `Popover.contains` (the card's ruling): a host would have to
  know every component that might portal inside it. Ancestry belongs to the
  layer stack, which already sees every registration. `Popover.contains` is
  unchanged.

### New tests (4, all in `packages/core/test/dismiss.test.ts`, named tests untouched)

- child layer's portalled surface clicked → child spared, its ancestor
  spared, the ancestor's peer dismisses
- ancestry survives portalling — parent is the layer on top at registration
  (exercised through the real `registerDismissLayer`), not a DOM ancestor
- three levels deep: clicking the innermost spares all three
- a true outside click still dismisses the whole chain in one interaction

`effigy test:core` after: 491 pass / 0 fail (487 + 4).

## 3. Defect 2 — ghost Select keeps its chevron

- `Select.svelte` and `Select.tsx`: removed the `variant !== "ghost"` gate on
  the non-searchable indicator button. Ghost drops the border and the fill,
  not the signal that the control is a select. Native and searchable modes
  were already unconditional.
- `packages/core/src/styles/select.css`: the ghost trigger previously reset
  `padding: 0`, which also stripped the indicator's decoration lane (the
  non-ghost trigger reserves `end-decoration-width + end-gap`). The ghost
  trigger now keeps that right padding, so the value ellipsizes before the
  chevron instead of running under it (the indicator button is absolutely
  positioned, so without the lane it overlaps the text).
- Workaround chevrons deleted in both runtimes: `OrderBy.svelte:318`,
  `OrderBy.tsx`, `FilterBuilder.svelte:346`, `FilterBuilder.tsx` (the
  hardcoded `▾` spans), plus the `.poodle-order-by__chevron` and
  `.poodle-filter-builder__chevron` rules in
  `packages/core/src/styles/{order-by,filter-builder}.css`, and the stale
  "label + optional summary + chevron" comment in filter-builder.css.
- Contracts updated: `docs/contracts/components/select.md` (indicator always
  rendered, ghost keeps it, decoration lane reserved), `order-by.md` and
  `filter-builder.md` (Chevron part deleted from anatomy, part tables, CSS
  sections, and checklists).
- New tests (6): ghost renders the indicator button on the non-searchable
  trigger (Svelte + React `Select` suites); OrderBy and FilterBuilder render
  no hardcoded chevron (Svelte + React each).

`effigy test:components` after: 988 pass / 0 fail (982 + 6).

## 4. Visual enumeration (report mode; no baseline refreshed)

The card names seven in-repo ghost-Select users. Ground truth from source:
only `HistoryCenter` and `BlockEditor` pass `variant="ghost"` to `Select`
in-repo; the `Select`s in `LogList`, `DataTable`, `RelationPicker`, and the
internal add-field/operator/value Selects of `OrderBy`/`FilterBuilder` are
default variant (already chevroned). Per-component classification:

| Component | In-repo change | Class |
|-----------|----------------|-------|
| `HistoryCenter` (Svelte + React) | fork-picker `Select` (`variant="ghost"`, `size="xs"`, custom trigger snippet) gains the indicator button next to the picker value | **affordance gained** — the fix working |
| `BlockEditor` (Svelte + React) | type-select (`variant="ghost"`, no snippet) and add-select (`variant="ghost"`, plus-icon trigger snippet) both gain the indicator button | **affordance gained** |
| `OrderBy` (Svelte + React) | summary trigger loses the trailing `▾`; internal "+ Add field" Select (default variant) unchanged | **workaround removed** |
| `FilterBuilder` (Svelte + React) | opener trigger loses the trailing `▾`; internal operator/value/add Selects (default variant) unchanged | **workaround removed** |
| `LogList` (Svelte + React) | filter `Select` is default variant | **no in-repo change** |
| `RelationPicker` (Svelte + React) | filter `Select` is default variant | **no in-repo change** |
| `DataTable` (Svelte + React) | column-filter and page-size `Select`s are default variant — both already render a chevron | **no in-repo change** |

Notes on the card's list: the card says "DataTable draws no workaround chevron
today, so it gains an affordance it was missing" — that presumes a ghost
`Select` in DataTable; in-repo DataTable's Selects are `variant="default"` and
already show the chevron, so the expected gain applies only to hosts that do
pass ghost. The seven names match the HistoryCentre (out-of-repo) composition;
the in-repo effect of this card is exactly the four ghost instances above plus
the two trigger-button changes. The history-centre app picks up the change via
its own ghost usages, unchanged code required. No baseline refreshed anywhere.

## 5. Step-6 validation

| Command | Exit | Result |
|---------|------|--------|
| `effigy test:core` | 0 | 491 pass / 0 fail |
| `effigy test:components` | 0 | 988 pass / 0 fail (69 files) |
| `effigy test:parity` | 0 | Svelte ↔ React anatomy parity green |
| `effigy check:svelte` | 0 | 0 errors (1 + 449 files) |
| `effigy docs:lint` | 0 | 171 component contracts (see race note) |
| `effigy docs:contract-drift` | 0 | contract prop tables ↔ Svelte `$props()`, both directions |
| `effigy svelte:surface-audit` | 0 | no surface gap |
| `git diff --check` | 0 | clean |

Race note: one `docs:lint` run exited 1 because it ran concurrently with
`check:svelte`'s `bun install --cwd packages/svelte/install-smoke`; lint-docs
scans install-smoke's `node_modules` and hit the install's transient
`.old-*` swap directory (`ENOENT ... .old-F41420ADC22D692B`). Re-run alone:
exit 0. Not a content failure; the card's commands were also run serially at
the end and all passed.

## 6. Commits

```
acb1acf4 g13-031: record dismiss-layer ancestry; spare a host when its child is clicked
691b4c93 g13-031: ghost Select keeps its chevron; drop the ▾ workarounds
```

Two separate commits, either revertible alone. Pushed with
`git push -u origin thread/g13-031-nested-dismiss-and-select-affordance`. Not
merged.

## 7. Notes

- `packages/contracts/components/src/select.rs` (Rust contract crate, outside
  this card's writable paths) still says ghost "hides the chevron indicator".
  No gate compares `docs/contracts/` against the Rust contract crates, so
  nothing fails; a follow-up card with `packages/contracts` writable should
  align the Rust doc comment.
- `packages/svelte/preview/artifacts/recipe-inventory.json` still lists
  `--poodle-recipe-order-by-chevron-text` (the recipe token for the deleted
  rule). Not gate-validated against CSS usage and outside writable paths;
  left as-is. The token definition itself lives in the tokens package
  (also not writable) and is now unused by order-by.css.
- The out-of-scope focus-trap papercut (portalled listbox escaping `Popover`'s
  Tab trap, `PAPERCUTS.md` 2026-08-12 entry) is untouched; the ancestry fix
  does not alter Tab handling.
