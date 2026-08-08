---
title: TextInput is actually editable — caret, selection, and a shared edit model
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, poodle-headless, poodle-render, text-input, editing]
---

## Before

The GPUI backend implemented "lightweight typing": append a character, or
backspace the last one. No caret, no selection, no cursor at all — typing in the
middle of a value was impossible, and a clicked field looked identical to an
idle one. The gap was recorded as "a pure `Node -> element` backend cannot
create a native GPUI Editor entity", which is true and also not the whole story:
most of what was missing needed no native editor.

## The Model Is Shared, Not GPUI's

`poodle_headless::text_input` now owns the editing rules:
`edit_transition(value, state, key, shift, accel)`, pure, returning the new
value (when it changed) and the new cursor. It implements the contract's
§Keyboard table: insert at the caret, backspace/delete around it, Arrow
Left/Right, Home/End, Shift+Arrow to extend, accel+A select-all, and correct
fall-through for Enter/Tab/Escape so submit/cancel/focus still work.

It started inside the GPUI backend. Putting it there would have meant Jetstream
reimplementing the same rules and drifting from them, so it moved to the shared
headless layer where every target drives one copy. **12 tests**, covering
insert-mid-string, selection replacement, cursor clamping when the host rewrites
the value underneath, inert-but-consumed edge keys, and multibyte text editing
by character rather than byte.

## The Caret Is Spec State

`<input>` owns its caret, selection and focus. GPUI and Jetstream have no native
editor to own them, so the host does — the same shape as
`TreeSpec::focused_value`:

- `TextInputSpec` gained `selection_start`, `selection_end` and `is_focused`,
  with `selection_range()` clamping to the value.
- `TextInputHandlers { on_change, on_selection_change, on_focus_change }`
  reports changes back, behind an additive `text_input_with_handlers`.
- The contract documents all three as **Rust targets only**, with a
  "Caret Ownership" section explaining why the web target has no equivalent.

## Rendering Without Measuring Text

The value is no longer one text node. It renders as runs around the cursor —
`[before][caret][after]`, or a tinted run when a selection exists — so the caret
lands in the right place with **no text measurement at all**: the layout engine
positions the runs and the caret is a 1px element between two of them. Unfocused
fields still render a single run, which is why no existing capture moved.

## Who Decides What A Key Means

`Interaction::on_edit_key` is a new vocabulary channel carrying the key name and
modifiers. Editing depends on the caret, which lives in the spec, so the
*component* decides what a keystroke means and reports the resulting value and
selection; the backend only forwards which key arrived. That is what lets one
edit model serve every backend.

The earlier backend-side attempt is gone, along with its thread-local cursor
storage — it was dead code for real inputs anyway, because
`poodle_render::text_input` renders the value as its own child rather than
relying on the backend's intrinsic input text.

## A Real Bug Found On The Way

`text_input` gave every field without a spec id the id `poodle-input`. Backends
key element state by id — gpui keeps **focus and the editing cursor** there — so
every unnamed input on a page shared one caret and stole each other's focus.
Unnamed fields now derive a distinct id from their own aria-label, placeholder
or name.

## Gate Change

`docs:contract-drift` compares documented props against the Svelte component, so
the three Rust-only props read as contract-only drift. Rather than park them in
the accepted-drift baseline — whose own comment says entries exist to be closed,
and these are permanent by design — the gate now skips rows the contract marks
`**… targets only**`. State the DOM owns natively has to be a controlled prop
where there is no DOM, and that is not drift.

## Verification

- Header search: click, type — the caret renders after the typed text and moves
  with it.
- `text-input` and `field` captures differ only in the header band from this
  round's chrome changes, plus `field`'s animated pending spinner. Unfocused
  inputs are unchanged, as intended.
- Green: `poodle-headless` 12, `poodle-render` 112, `poodle-specs` 227, node
  backend 4, both preview builds, `effigy drift:handlers`,
  `effigy docs:spec-drift`, `effigy docs:contract-drift`, `git diff --check`.

## Still Missing

- **Click-to-position the caret.** Needs text measurement to map an x offset to
  a character index — the one part that genuinely belongs to the backend.
  Clicking focuses; the caret stays where it was.
- **Blur is not reported.** `on_focus_change` fires on activation only, so a
  field keeps its caret after focus moves elsewhere. Needs a focus-out channel;
  gpui exposes focus listeners on `Context`/`Window` against a `FocusHandle` the
  backend never receives, so this wants thought rather than a quick hook.
- **IME and clipboard.** Both need a platform input handler
  (`EntityInputHandler`), which is the part the original "no native editor" note
  was really about.
- Jetstream renders the caret from the same nodes, but its shell does not yet
  own selection state for its own fields.
