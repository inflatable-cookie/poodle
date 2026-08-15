# 14 — g14.006 TextInput Runtime-boundary Proof

Batch log, 2026-08-15. Card:
`docs/roadmaps/g14/006-text-input-runtime-boundary-proof.md`.

## What changed

The first complete input profile ran through the landed conformance kernel:
one portable TextInput interface + 18 typed cases executed headless in
Svelte, React, and GPUI. Typing, selection, IME commit, and event order go
through real editing paths. DOM and GPUI mechanisms differ; the portable
actions and observations do not.

- **Interface authority** — `packages/core/src/conformance/text-input.ts`
  (profile `input`): controlled/uncontrolled value, validation, affixes,
  adornment icons, char count, search clear, size/density, and native-caret
  plus residual native-compat fields generated into Rust but excluded from
  `PortablePropsOf`. Web-html attributes never enter the Rust spec.
- **Corpus** — `text-input-cases.ts` (18 cases): default, controlled value,
  disabled, invalid chrome, affixes, adornments, char count, type,
  controlled type, selection replace, disabled/read-only inert, focus,
  submit, cancel, type-then-submit order, search clear, IME commit.
- **Generated declaration** — `generated/text-input/mod.rs` replaces the
  hand-written struct. The extension keeps token recipes, `current_value`,
  `with_selection`, and aliases (`with_focused`, `with_input_type`). Integer
  `rustType` stays Eq-safe; `usize` is not namespaced under `crate::types`.
- **Generic vocabulary** — `insert` / `select` / `compose` actions; string
  `value` and selection observations. Portable selection uses UTF-16 offsets;
  the GPUI adapter translates to its scalar-indexed editor. The selection case
  crosses a non-BMP character so that boundary cannot drift silently. No
  TextInput identifier lives in generic runners.
  Web insert is one `input` event. GPUI insert is per-character
  `on_edit_key`; one `valueChange` is recorded per insert action. IME
  start/update must not commit; commit is the single insert.
- **Web execution** — controlled hosts, portable bind for leading/trailing
  icons, `data-has-leading` / `data-has-trailing`, generic `text-actions.ts`.
- **Native execution** — renderer stamps part ids, wires submit/cancel/clear,
  search auto-leading icon and clear (`valueChange` then `clear`). Observer
  reads `NodeKind::Input` value and caret. When root and another `self` part
  are the same node, root keeps wrapper identity (not the tab stop); the
  named part carries role/value/focusable. GPUI IME buffers in `MARKED` +
  composing string; paint splices preview; `on_edit_insert` is commit.
- **Primitive rows** — `input.value`, `input.editing`, `input.ime` on web,
  render-neutral, and GPUI join the gated owned rows.
- **Failure proof** — dropped edit, dropped selection, dropped IME commit,
  and submit-before-valueChange each fail the expected field.
- **Geometry** — default display cases compare the authored root radius across
  all active runtimes.
- **Review repair** — direct icon props are contracted; React multiline passes
  `autocorrect` like Svelte.
- **Cost** — TextInput pilot increment 1,918 LOC (746 authored authority,
  230 generated source, 942 harness and runtime deltas) and 33,538 bytes of
  TextInput fixture JSON. Mechanism total 17,003 LOC.

## Before / after runtime

| Board | Before | After |
| --- | --- | --- |
| Web TextInput corpus | none | 18 cases × Svelte + React |
| GPUI TextInput corpus | none | 18 cases headless |
| Primitive owned rows | overlay set | + `input.value` / `input.editing` / `input.ime` |
| Jetstream | deferred | still deferred |

## Stop conditions

- Typing goes through keys (GPUI) or `input` events (web), not a host-only
  state write.
- IME start/update do not emit `valueChange`; commit does, once.
- Portable TS omits native-caret, native-compat, and web-html props.
- Jetstream is not in the active cohort reports.

## Residual

- Curated TextInput specimens stay as documentation (g14.026).
- `with_input_type` / `with_focused` remain aliases on the extension.
- Native root is the input node (web root is the wrapper). The observer
  projects wrapper identity onto root when another `self` part shares the
  node; corpus does not assert `root.focusable`.
- happy-dom has no keydown→input default; web insert uses `input` events.

## Next

g14.007 HistoryCenter, after orchestrator review of this PR.
