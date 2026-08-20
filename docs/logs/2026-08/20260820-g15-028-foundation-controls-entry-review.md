# g15.028 — Screen-clear review: foundation controls and entry

Date: 2026-08-20
Card: `docs/roadmaps/g15/028-review-foundation-controls-entry.md`
Handoff: `docs/handoffs/20260820-143005-g15-028-review-foundation-controls-entry.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: pending

## Outcome

First of the six serial screen-clear review children. All 14 owned pages
received the human teaching review against the carried rubric — live Svelte
and React source and routes, GPUI specimen source, and the `g15.026` headless
construction/axis evidence. **Eleven pages keep unchanged. Three pages needed
bounded GPUI-only specimen repairs. No Svelte or React page changed, no stop
condition fired, and no contract, public API, component, shared-CSS, generated
catalogue, or infrastructure file moved.**

The 14 human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; the screening `keep` /
"no named defect" text was replaced, not extended with a second table.

## Verdict inventory

### Unchanged (11)

| Page | Verdict |
| --- | --- |
| `Checkbox` | keep — live default trio teaches normal use; Sv/Rc paired verbatim; Gp mirrors all three sections live |
| `CollapseToggle` | keep — four live directions with state readout plus Disabled; all runtimes agree |
| `Radio` | keep — live three-option group plus States; Gp matches (custom-color hex is fixture data, not copy) |
| `RadioGroup` | keep — vertical/horizontal/disabled/custom color all live; Gp adds a bounded disabled-option visual |
| `SegmentedControl` | keep — live default, disabled option, content fit, icon-only; Gp omits icon-only (node-tier options carry no icon) and adds equal width, both bounded |
| `Switch` | keep — live trio, States, custom colors, dual labels/tones; Gp mirrors all four sections |
| `ColorPicker` | keep — basic/swatches/alpha/default-open/preview-only/disabled; Gp mirrors all six with live open and value state |
| `NumberInput` | keep — numeric, steppers, string-form binding, disabled, invalid; string-form binding is web-only, Gp adds prefix/suffix/precision within the section budget |
| `Rating` | keep — live default, 10-star, half-step, clearable, disabled; Gp mirrors and adds readonly |
| `Slider` | keep — live volume/step/disabled/embedded with paired axis panes; Gp adds two static fill-evidence sections within budget |
| `ThemeSelect` | keep — standalone live selector, compact trigger, columns, disabled; Gp teaches interactive open/select plus Disabled (compact trigger and columns stay web-shown) |

### Repaired (3, GPUI specimen only)

- **`ToggleGroup`** — the GPUI "Multiple selection" section was inert: the
  node tier attaches `on_activate` only when a change handler exists
  (`packages/render/src/toggle_group.rs`), so clicks did nothing and the
  "Selected: design, docs" readout was hardcoded. The web pages teach this
  mode live with a real readout. The specimen now wires the group through the
  specimen event queue, toggling membership and rendering the live set. The
  static "Allow deactivation" section was removed: deactivation is a purely
  behavioral prop with no visible evidence, no runtime teaches it, and an
  inert section demonstrated nothing.
- **`EditableLabel`** — the GPUI page carried ten sections, past the specimen
  plan's curation-defect threshold. "Display mode (value + edit icon)" was a
  static duplicate of "Click to edit with icon"; "Flush variant (editing)"
  duplicated the editing state reachable through the interactive flush
  example. Both removed; the page now leads with the web page's default
  example and keeps the renderer-owned live editing composition second.
- **`CodeInput`** — the GPUI page carried nine sections; "Partial (3 of 6)",
  "Complete", and "Numbers only" were static fill states the web page does
  not need and the live default example already demonstrates (entry,
  completion, and the pass/fail check). Removed, and the remaining sections
  reordered to the web order (default, masked, grouped, alphanumeric, error,
  disabled) — six sections, matching the shared outline.

No page needed a contract, public API, or component-semantic change; no
underlying component control was found broken. The defects were dead specimen
wiring and section-budget drift, both specimen-owned.

## Changed routes for review

Changed GPUI routes: `toggle-group`, `editable-label`, `code-input`.
**No Svelte or React route changed**, so the operator live web checkpoint has
no pages to open for this child; GPUI evidence stays headless per the card.
Operator sign-off is pending and this card is not claimed complete.

## Validation

- `effigy probe:gpui-specimens` — 7/7 (174/174 routes construct, all
  advertised axis panes open) on the final specimen code
- `effigy check:gpui` — cargo check plus `poodle-render` and
  `poodle-gpui-node-backend` test suites pass
- `effigy check:svelte-preview` — 0 errors
- `effigy react:build` — pass
- `effigy catalogue:check` — pass (TS and Rust catalogue targets verified)
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — clean

No new focused test was added: the only interaction change (ToggleGroup
multi-select) rides the same `NodeSpecimenEvent` queue mechanism every other
live section on these pages already uses, and asserting it headlessly would
require new `debug_selector` markers — specimen infrastructure this card may
not change. The construction probe covers all three changed routes.

## Execution note

Mid-run, something outside this session deleted the entire
`AGENTS_WORKTREE_CONTAINER_DIR` (`/Users/tom/.t3/worktrees/poodle`),
including this worker's uncommitted work and every other registered worktree
directory. The worktree was recreated per the handoff's manual fallback, the
repairs reapplied, and work committed immediately. The other worktrees'
registrations were pruned by `git worktree prune`; their branches are
untouched. If the deletion was intentional operator cleanup, no harm done —
but it cost one probe run and could recur.
