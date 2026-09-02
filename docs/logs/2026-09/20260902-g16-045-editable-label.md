# g16.045 — EditableLabel Editing Model And Mounted Parity

Status: complete — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/045-editable-label-editing-model-and-mounted-parity.md`
Handoff: `docs/handoffs/20260902-004200-g16-045-editable-label.md`
Governing refs: `docs/contracts/components/editable-label.md`,
`docs/contracts/components/text-input.md`,
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/contracts/001-working-rules.md`
Branch: `feature/g16-045-editable-label`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-045-editable-label`
Base: planning commit `7f59ae42f4917c675968819eb23a5e41dc90013c` is an ancestor.

## Outcome

One host-owned committed `value` and one session-private draft across Svelte,
React, shared machines, the Rust spec/renderer, and GPUI. `onCommit` is
`{ value, previousValue }` everywhere. Native live paint uses `draft_value`;
committed `value` does not move until commit. Web owns `isEditing` and draft
internally. No public controlled draft, `commit()` method, or compatibility
shim.

Portable trim is set **T** (Unicode White_Space plus U+FEFF). `maxLength`
counts Unicode scalar values. `doubleClick` gains Enter/Space. Teardown emits
neither commit nor cancel. Enter/Escape restore display focus; Tab and blur
commit without restoring.

Svelte instance methods and React `EditableLabelHandle` (component module and
package root) are exactly `focus()`, `startEditing()`, `cancelEditing()`.

LicenceActivation and LicenceSeats native hosts project draft and selection
beside the committed label.

## Evidence

- Paired `edit` machines and vectors: NEL/BOM plant, ZWSP interior, scalar
  clamp, teardown, unchanged commit, restoreFocus on Enter vs blur.
- Svelte/React focused tests: methods, activation, scalar clamp, portable
  trim, teardown, window blur, name resolution, external value replacement,
  disablement during edit.
- Render unit tests: live draft paint, scalar insert, trimmed commit previous.
- Mounted GPUI: existing Tab-via-blur routing plus
  `editable_label_live_draft_stays_off_the_committed_value` (paint `Kicks`,
  host value and previous stay `Kick` until Enter).
- Ledger: EditableLabel GPUI mounted 56/119 → 57/118. Updated date stays
  `2026-08-26`.

## Native notes

GPUI `on_activate` is click-equivalent, so `doubleClick` uses
`on_double_activate` plus display `on_edit_key` for enter/space. Programmatic
mode attaches neither. Focus restore is renderer `on_restore_display_focus`
plus spec `request_focus` on the next view paint. Already-editing
LicenceActivation hosts must project `machine_label_selection`; the mounted
Escape fixture seeds end-caret so the typed suffix stays unique.

## Oracle falsification

Planted after `b4a822a4b`, confirmed intended proofs failed, then restored with
`git checkout HEAD --` on the four files. Clean tree after restore.

| Invariant | Plant | Actual failure |
| --- | --- | --- |
| Distinct committed/draft | renderer paints `spec.value` instead of `live_text()` while editing | `editable_label_live_draft_stays_off_the_committed_value`: `left: "Kick"` `right: "Kicks"` |
| Portable trim (JS) | `trimEditableLabel` = `value.trim()` | `edit-code-token`: Expected `"Take"`, Received `"\u0085Take"`. Svelte commit: `onCommit` `{ value: "\u0085Take", previousValue: "Kick" }` |
| Portable trim (Rust) | `trim_editable_label` = `value.trim()` | `portable_trim_drops_nel_and_bom_that_str_trim_does_not_both_drop`: `left: "Take\u{feff}"` `right: "Take"` |
| Scalar length | Svelte `maxlength={maxLength}` | `clamps maxLength to Unicode scalar values`: `expected true to be false` at `input.hasAttribute("maxlength")` |
| Silent teardown | `onDestroy` sends `COMMIT_BLUR` | `emits neither commit nor cancel when unmounted while editing`: `onCommit` called once with `{ previousValue: "Kick", value: "Kicks" }` |

Tab-via-blur, activation, name, and handle oracles already fail on the focused
web tests without extra plants; g16.008 still owns the mounted Tab routing test.

## Validation

- `bunx vitest run` EditableLabel Svelte/React + `edit-code-token` — 34 pass
- `cargo test` headless `edit`, render `editable_label`/`licence` — pass
- `cargo test --test headless_regressions editable_label` — pass
- `cargo test --bin poodle-preview a_machine_label` — pass
- `bun scripts/parity-evidence-ledger.ts --write` + ledger test — pass
- `docs:contract-drift`, `docs:callback-drift`, `docs:spec-drift`,
  `drift:handlers` — pass
- `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`,
  `effigy qa`, and `git diff --check origin/main...HEAD` — recorded at PR time
- Never ran `*-windowed` or native-visual selectors
