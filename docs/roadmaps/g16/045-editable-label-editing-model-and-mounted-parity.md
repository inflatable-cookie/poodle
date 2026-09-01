# g16.045 — EditableLabel Editing Model And Mounted Parity

Status: ready
Type: implementation
Opened: 2026-09-01
Depends on: merged `g16.008`, `g16.030`, and the accepted EditableLabel
decision packet in `../../triage/20260901-230406-editable-label-decision.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/editable-label.md`,
`../../contracts/components/text-input.md`,
`../../architecture/006-headless-core-and-machine-model.md`

## Goal

Replace the split web/native EditableLabel model with one host-owned committed
value and one session-private draft. Align activation, trim, Unicode-scalar
`maxLength`, commit/cancel/blur, selection, focus restoration, teardown, and
mounted GPUI behavior in one clean pre-1.0 migration.

## Fixed Envelope

- `value` is the committed string. Web owns `isEditing` and draft internally;
  native projects `value`, `draft_value`, and `is_editing` without storing the
  live draft in `value`.
- `onCommit` carries `{ value, previousValue }` in every active runtime.
  Unchanged commits still emit. Cancel never emits a commit.
- `doubleClick` keeps double-click pointer entry and gains Enter/Space keyboard
  entry. `enterOrSpace` uses single click or Enter/Space. `programmatic` uses
  only `startEditing()` / `cancelEditing()` or the native wrapper.
- Enter commits and restores display focus. Escape cancels and restores it.
  Tab, pointer blur, and window blur commit once without restoring. Teardown
  itself emits neither commit nor cancel.
- Commit trims the exact portable set **T** from the accepted packet. Paired
  machines must not call language-native `trim`.
- `maxLength` counts Unicode scalar values. HTML UTF-16 `maxlength` is not the
  authority. `selectOnFocus=true` selects all; false places the caret at end.
- Omitted `ariaLabel` resolves from visible value, then `emptyText`, then
  `"Edit label"`. The same resolved name reaches display and editor.
- No public live draft, validation, pending, multiline, rich-text, persistence,
  or compatibility surface is added. Jetstream remains deferred.

## Ordered Work

1. Amend the component contract first. Add paired TypeScript/Rust transition
   and trim vectors, including NEL, BOM, ZWSP, empty, astral, unchanged, and
   teardown cases.
2. Align Svelte and React adapters, methods, scalar-length clamp, focus guards,
   keyboard entry, and teardown-blur suppression.
3. Align the Rust spec, renderer, GPUI session wrapper, and LicenceActivation /
   LicenceSeats in-repo composites. Use the shared headless text-input path.
4. Add focused and mounted proof. Move only EditableLabel's GPUI mounted-
   behavior ledger cell when the full active-cohort result is proven.
5. Record one execution log. Leave g16 front doors to orchestrator closeout.

## Acceptance

- Starting a second session after cancel seeds from the committed value, not an
  abandoned draft.
- Native paint uses `draft_value` while `value` and
  `onCommit.previousValue` retain the committed snapshot.
- TypeScript and Rust produce the same set-T trim result and scalar-length
  decisions, including `maxLength=1` accepting `"𝄞"` and rejecting a later
  ASCII insert.
- Enter, Escape, Tab, pointer/window blur, external value replacement,
  disablement, and teardown follow the fixed callback and focus law exactly.
- `doubleClick`, `enterOrSpace`, and `programmatic` have the same observable
  activation boundary across Svelte, React, shared Rust, and GPUI.
- The accessible name no longer defaults over visible content. No live region
  or new focus manager appears.
- No sibling repository, release, visual-comparison, broad native AT, or
  Jetstream claim enters the diff.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Committed and draft values stay distinct | type `Kicks` over committed `Kick` | native node paints `Kicks`; spec value and callback previous value remain `Kick` |
| Trim is portable | commit `\u0085Take\uFEFF` | both machines emit `Take`; planted language-native trim fails |
| Length is scalar-based | `maxLength=1`, insert `𝄞`, then `A` | first insert succeeds, second is silent in web and native |
| Terminal callbacks are exact | Escape or Enter followed by unmount blur | cancel: one cancel/no commit; Enter: one commit; teardown adds nothing |
| Tab remains traversal | type, then Tab | commit once and focus advances; display is not refocused |
| Activation modes stay distinct | single-click default and gesture in programmatic mode | default stays view on one click; programmatic stays view for every gesture |
| Focus/name behavior is observable | omit `ariaLabel` on value `Kick`, then Enter/Escape | both nodes are named `Kick`; display focus returns after direct terminal |

Plant the pre-fix behavior for each row after committing the real proof. Restore
from that commit and rerun green.

## Writable Scope

EditableLabel contract; shared edit/text helpers and vectors; component-local
Svelte/React files and tests; EditableLabel Rust spec/headless/render paths;
the GPUI wrapper/specimen/regressions; LicenceActivation/LicenceSeats adapters;
the EditableLabel ledger row; this card, one log, and new `PAPERCUTS.md`
entries. Do not edit global roadmap front doors, releases, workflows, sibling
repositories, visual evidence cells, or Jetstream behavior.

## Validation

Use Effigy discovery. Run focused paired-machine, Svelte, React, Rust render,
and mounted GPUI checks; relevant contract/prop/callback drift selectors;
`effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, one
final headless `effigy qa`, and `git diff --check origin/main...HEAD`. Never run
`*-windowed` or native-visual selectors.

## Stop Conditions

Stop if the envelope requires a public controlled draft, async validation,
multiline/IME expansion beyond the current TextInput boundary, a new focus
architecture, a compatibility shim, or a ledger claim wider than the single
mounted EditableLabel cell.

## Continuation

After accepted merge, mark EditableLabel closed in the continuation register.
No separate web/native follow-up is planned.
