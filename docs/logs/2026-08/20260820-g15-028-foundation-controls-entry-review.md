# g15.028 — Screen-clear review: foundation controls and entry

Date: 2026-08-20
Card: `docs/roadmaps/g15/028-review-foundation-controls-entry.md`
Handoff: `docs/handoffs/20260820-143005-g15-028-review-foundation-controls-entry.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: #51

## Outcome

First of the six serial screen-clear review children. All 14 owned pages
received the human teaching review against the carried rubric — live Svelte
and React source and routes, GPUI specimen source, and the `g15.026` headless
construction/axis evidence. **Nine pages keep unchanged, four pages needed
bounded GPUI-only specimen repairs, and one page records a
contract/runtime blocker.** No Svelte or React page changed, and no contract,
public API, component, shared-CSS, generated catalogue, or infrastructure
file moved.

The 14 human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; the screening `keep` /
"no named defect" text was replaced, not extended with a second table.

## Review round 1 (orchestrator, PR #51)

The first pass claimed "no stop condition fired" and was wrong on three
counts; all three are addressed in this revision:

1. **SegmentedControl's GPUI icon omission is a contract/runtime blocker, not
   a bounded keep.** The contract's `SegmentedControlOption.icon`/`iconOnly`
   (`docs/contracts/components/segmented-control.md` §SegmentedControlOption)
   have no counterpart in Rust `ChoiceOption`/`SegmentedControlSpec`
   (`packages/contracts/components/src/segmented_control.rs`). Under the
   active-cohort working rule that is a gap to port, and the card treats it
   as a stop condition, not hidden work. It is **not** implemented here; the
   audit row now carries disposition `contract/runtime-blocker` with the
   exact blocker, the GPUI grade drops to B, and the audit totals are
   updated (keep 55, blocker 1). The orchestrator routes the follow-up.
2. **ThemeSelect's native page did not satisfy its existing specimen
   contract.** The contract's specimen definition (`theme-select.md` §14)
   requires the compact trigger (no label) and four-column cases, and both
   are already portable Rust inputs (`ThemeSelectSpec.show_label`,
   `columns`). The GPUI specimen now teaches both — compact trigger as a
   static closed example, four columns as a second interactive example whose
   open popover shows the grid — matching the web page's section set.
3. **The ToggleGroup behavioural repair had no regression that could detect
   the old defect.** The construction probe never activates specimen
   controls, so the inert pre-PR page also passed it. The multi-select
   builder and transition are now private helpers
   (`multi_select_toggle_group_node`, `multi_select_transition`) with an
   owner-local test that invokes a rendered option's `on_activate`, drains
   the event queue, and asserts add-then-remove membership. It fails if the
   handler is removed — the assertion expects `on_activate` to exist.

Also corrected per review: this log's `PR:` field, and the stale
`node_toggle_group_static` comment that still listed the removed
multiple/deactivation cases.

## Verdict inventory

### Unchanged (9)

| Page | Verdict |
| --- | --- |
| `Checkbox` | keep — live default trio teaches normal use; Sv/Rc paired verbatim; Gp mirrors all three sections live |
| `CollapseToggle` | keep — four live directions with state readout plus Disabled; all runtimes agree |
| `Radio` | keep — live three-option group plus States; Gp matches (custom-color hex is fixture data, not copy) |
| `RadioGroup` | keep — vertical/horizontal/disabled/custom color all live; Gp adds a bounded disabled-option visual |
| `Switch` | keep — live trio, States, custom colors, dual labels/tones; Gp mirrors all four sections |
| `ColorPicker` | keep — basic/swatches/alpha/default-open/preview-only/disabled; Gp mirrors all six with live open and value state |
| `NumberInput` | keep — numeric, steppers, string-form binding, disabled, invalid; string-form binding is web-only, Gp adds prefix/suffix/precision within the section budget |
| `Rating` | keep — live default, 10-star, half-step, clearable, disabled; Gp mirrors and adds readonly |
| `Slider` | keep — live volume/step/disabled/embedded with paired axis panes; Gp adds two static fill-evidence sections within budget |

### Repaired (4, GPUI specimen only)

- **`ToggleGroup`** — the GPUI "Multiple selection" section was inert: the
  node tier attaches `on_activate` only when a change handler exists
  (`packages/render/src/toggle_group.rs`), so clicks did nothing and the
  "Selected: design, docs" readout was hardcoded. The web pages teach this
  mode live with a real readout. The specimen now wires the group through the
  specimen event queue, toggling membership and rendering the live set, and
  carries the focused regression test described above. The static "Allow
  deactivation" section was removed: deactivation is a purely behavioral prop
  with no visible evidence, no runtime teaches it, and an inert section
  demonstrated nothing.
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
- **`ThemeSelect`** — the GPUI page showed only the interactive default and
  Disabled, short of the contract's specimen definition. Added the compact
  trigger (no label) and the four-column grid (interactive, deferred
  popover), giving the native page the web page's four-section set.

### Contract/runtime blocker (1)

- **`SegmentedControl`** — GPUI cannot teach the icon-only section: the Rust
  option/spec types lack the contract's `icon`/`iconOnly` fields. Recorded
  as `contract/runtime-blocker` in the audit; not implemented in this PR.

No other page needed a contract, public API, or component-semantic change;
no underlying component control was found broken. The repaired defects were
dead specimen wiring, missing contract-required sections, and section-budget
drift, all specimen-owned.

## Changed routes for review

Changed GPUI routes: `toggle-group`, `editable-label`, `code-input`,
`theme-select`. **No Svelte or React route changed**, so the operator live
web checkpoint has no pages to open for this child; GPUI evidence stays
headless per the card. Operator sign-off is pending and this card is not
claimed complete.

## Validation

- `effigy probe:gpui-specimens` — 7/7 (174/174 routes construct, all
  advertised axis panes open) on the final specimen code
- focused regression: `cargo test --bin poodle-preview multi_select` — 2/2;
  the activation test fails without the handler by construction (it expects
  `on_activate` to exist before invoking it)
- `effigy check:gpui` — cargo check plus `poodle-render` and
  `poodle-gpui-node-backend` test suites pass
- `effigy check:svelte-preview` — 0 errors
- `effigy react:build` — pass
- `effigy catalogue:check` — pass (TS and Rust catalogue targets verified)
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — clean

## Execution note

Mid-run, something outside this session deleted the entire
`AGENTS_WORKTREE_CONTAINER_DIR` (`/Users/tom/.t3/worktrees/poodle`),
including this worker's uncommitted work and every other registered worktree
directory. The worktree was recreated per the handoff's manual fallback, the
repairs reapplied, and work committed immediately. The other worktrees'
registrations were pruned by `git worktree prune`; their branches are
untouched. If the deletion was intentional operator cleanup, no harm done —
but it cost one probe run and could recur.
