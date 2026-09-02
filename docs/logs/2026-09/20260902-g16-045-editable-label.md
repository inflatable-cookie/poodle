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
beside the committed label. `machine_name` and `seat_row` also forward
EditableLabel's Enter/Escape restore: the composite handlers fire, and the
host's next view-mode paint sets `machine_label_request_focus` or
`request_focus_machine_id`.

## Evidence

- Paired `edit` machines and vectors: NEL/BOM plant, ZWSP interior, scalar
  clamp, teardown, unchanged commit, restoreFocus on Enter vs blur.
- Svelte/React focused tests: methods, activation, scalar clamp, portable
  trim, teardown, window blur, name resolution, external value replacement,
  disablement during edit, and real Enter/Escape restoring `document.activeElement`
  on the display control (Tab blur does not).
- Render unit tests: live draft paint, scalar insert, trimmed commit previous,
  plus `machine_label` / seat-row restore-channel counts (Enter/Escape fire;
  blur commit does not).
- Mounted GPUI: Tab-via-blur routing; live draft oracle; Enter/Escape restore
  the real display focus handle on standalone EditableLabel, LicenceActivation
  `machine_name`, and LicenceSeats `seat_row`. Those three tests also require
  the restore callback in the event log — stable IDs can keep focus after a
  remount without it. Tab still advances and must not emit restore.
- Ledger: EditableLabel GPUI mounted 56/119 → 57/118. LicenceActivation and
  LicenceSeats keep mounted status and name the new restore tests. Updated
  date stays `2026-08-26`.

## Native notes

GPUI `on_activate` is click-equivalent, so `doubleClick` uses
`on_double_activate` plus display `on_edit_key` for enter/space. Programmatic
mode attaches neither. Focus restore is renderer `on_restore_display_focus`
plus spec `request_focus` on the next view paint. Composites were the missing
route: standalone preview already forwarded it; `machine_name` and `seat_row`
now project `request_focus` and forward restore, and LicenceSeats gained
`on_rename_cancel` so Escape can close the row. Already-editing
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

Restore-focus composition planted after `d3b020413`, then restored with
`git checkout HEAD --` on the three files. Clean tree after restore.

| Invariant | Plant | Actual failure |
| --- | --- | --- |
| Paired display focus | Svelte `emitCommit`/`emitCancel` skip `displayElement?.focus()` | `restores display focus on Enter and Escape, not on Tab blur`: `document.activeElement` is `<body>`, not the display button (`EditableLabel.test.ts:292`) |
| Composite restore (`machine_name`) | `machine_name` sets `on_restore_display_focus: None` | render: `left: 0` `right: 1` "Enter restores display focus". Mounted: `left: ["machine/commit:Studio Mac2"]` `right: [..., "machine/restore"]` |
| Composite restore (`seat_row`) | `seat_row` sets `on_restore_display_focus: None` | render: `left: []` `right: ["id-a", "id-a"]`. Mounted: `left: ["seat/commit:Studio rig2"]` `right: [..., "seat/restore"]` |

Stable-ID remount can leave `focus_state_for` true without the restore
callback. The mounted composite proofs therefore also require `machine/restore`
and `seat/restore` on Enter/Escape; Tab must not emit them.

## Validation

- `bunx vitest run` EditableLabel Svelte/React — 36 pass
- `cargo test --manifest-path packages/render/Cargo.toml restore_display_focus` — 2 pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions restore_display_focus` — 2 pass
- `editable_label_commits_on_enter_and_once_through_the_blur_tab_causes` — pass
- `a_machine_name_escape_restores_the_original_in_a_mounted_window` — pass
- `bun scripts/parity-evidence-ledger.ts --write` + ledger test — pass
- `docs:contract-drift`, `docs:callback-drift`, `docs:spec-drift`,
  `drift:handlers` — pass
- `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check` —
  pass
- `effigy qa` — pass
- `git diff --check origin/main...HEAD` — pass
- Never ran `*-windowed` or native-visual selectors
