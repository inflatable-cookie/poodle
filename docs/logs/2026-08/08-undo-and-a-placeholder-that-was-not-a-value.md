---
title: Undo, and a placeholder that was being edited as if it were text
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, poodle-headless, poodle-node, node-backend, text-input, undo]
---

## Undo

`accel+Z` and `accel+shift+Z` on every text field, with no host change required.

**Where the state lives.** History is ephemeral UI state — it belongs to the
field while it is on screen and means nothing afterwards, exactly like the blink
phase and the scroll offset already kept beside it. So the backend owns the
stack, keyed by the value node's id. The alternative was pushing another
controlled prop onto every host, for something no host cares about.

**What counts as one step is shared.** `poodle_headless::text_input::coalesces`
decides whether an edit continues the previous one. A continuous run of typing
collapses into a single step; a deletion, a paste, or a caret that jumped starts
a new one. That rule is a text-editing decision, and a rule duplicated per
backend is the drift the headless layer exists to prevent — so the node backend
now depends on `poodle-headless` (pure, no gpui) for that one function, with the
reason recorded in its manifest.

**The subtle part was where a run begins.** The first cut coalesced the opening
keystroke into the entry holding the *pre-edit* state and overwrote it, so a
whole run collapsed to one entry and there was nothing left to undo *to*. Typing
"hello" produced a history of length 1. The first change of a run must push;
only later ones replace. `History::run_open` tracks that, and stepping through
history closes the run so typing after an undo starts a fresh entry.

Editing after an undo truncates the redo tail, as every editor does.

## The Bug Underneath

Undo surfaced something worse that had been sitting there since the caret work:
after undoing an empty field, typing produced `"xFind component..."` — the
**placeholder had been recorded as the value and restored as real text**.

The cause was in my own refactor. When the value node became a text node, it was
given `value or placeholder` as its content, and the backend received that one
string as *both* the display and the value. One layer down the two are
indistinguishable: selection indices counted into the prompt, and history
recorded it.

### Two Fixes, And Why The First Was Wrong

The first attempt rendered the placeholder as its own absolutely-positioned
child so the value node could hold the true (possibly empty) value. It looked
right in GPUI and **the Jetstream gate caught it overlapping and doubling** —
`field`, `form-dialog` and `form-layout` all showed prompt text drawn over
value text.

That is the second time today the Jetstream gate has caught something GPUI could
not, and the reason is structural: two engines interpreting the same nodes
disagree about anything the vocabulary leaves implicit.

The fix is to stop leaving it implicit. `NodeCaret::showing_placeholder` says
which of the two the text is, so the backend can measure the caret against an
empty value and record nothing. A flag rather than a layout trick — the
vocabulary describes intent, and an absolutely-positioned sibling is one
backend's quirk.

## Verification

Driven live, reading history state each frame:

- Type `hello` → **2** entries, not 6: the initial state, and one coalesced run.
- `cmd+Z` → `""`, the true empty value rather than the placeholder.
- `cmd+shift+Z` → `hello` restored.
- Undo then type `xy` → new run at cursor 1, redo tail gone.

Green: `poodle-render` 129, `poodle-headless` 34, node backend 8, `poodle-node`
2, `poodle-jetstream` 161, `ci:native` 0, `docs:lint` 0,
`docs:contract-drift` 0.

Both pixel gates: GPUI `text-input`/`field` unchanged; Jetstream **137 compared,
0 failing**.

## A Note On Method

An early reading of this work said redo was broken. It was not — my probe
deduplicated identical lines, so a return to a previous state printed nothing.
A probe that hides repeats cannot observe a system whose whole job is returning
to previous states.

## Contract

`text-input.md` gains an **Undo (Rust targets)** section: the chords, the
one-step-per-run rule, what ends a run, and where the history lives.
