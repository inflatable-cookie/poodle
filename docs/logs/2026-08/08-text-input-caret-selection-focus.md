---
title: TextInput — measured caret, real selection, and one source of truth for focus
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, poodle-node, poodle-render, node-backend, text-input, focus]
---

## Symptom

"Doesn't feel good yet. Selection doesn't work, caret doesn't blink, hover,
focus and blur states are all over the place."

All three were the same mistake, made three times: the caret had been built to
avoid ever measuring text, and focus had been inferred instead of observed.

## What Was Actually Wrong

**Hover overrode focus.** gpui refines styles in a fixed order — `in_focus`,
`focus`, `group_hover`, `hover`, `active` (`div.rs` 2484/2490/2499/2506/2554).
`text_input` sets a hover border *and* a focus ring, so hovering a focused
field silently replaced its ring. Nothing in the component could see that.

**There were two ideas of "focused".** gpui's real focus drove the ring; the
spec's `is_focused` drove the caret, and it was latched by `on_activate` — a
channel that can only ever report a *gain*. A field kept its caret after focus
moved on, and the two signals disagreed from the first click onward.

**The caret could not be positioned or hit-tested.** The value was split into
sibling text runs with a 1px caret between them, chosen precisely because it
needs no text measurement. It renders a caret, but it cannot answer "which
character did I click on?", which is the same question as "where does this
caret go?" — so click-to-position, drag-select and blink were all unreachable
from that design, not merely unimplemented.

## The Reference

gpui ships `examples/input.rs`, which is the canonical answer and was not
consulted: `shape_line` to measure, `x_for_index` to place a caret,
`closest_index_for_x` to turn a click back into an index, and the caret and
selection painted as quads rather than laid out. Measurement is the backend's
job in this architecture already — the runs trick was a way around the one
thing that genuinely belongs to the backend.

## Changes

**`poodle-node`** — `NodeCaret { selection, caret_color, selection_color }` on
`Node`, plus `Interaction::on_select_range` (pointer → character indices) and
`Interaction::on_focus_change` (both directions).

Deliberately a **new field on `Node`, not new fields on `NodeKind::Input`**:
adding fields to a struct variant breaks every `match` in every backend, which
is the opposite of additive. The first cut did exactly that and broke the
sibling Jetstream repo's interpreter, which this work may not modify. A backend
that has never heard of carets keeps compiling and keeps rendering the value.

Selection is expressed in **character** indices; `ShapedLine` counts bytes. The
conversion lives at the backend's edge and nowhere else.

**GPUI backend** — a new `input_text` element:

- Shapes the value once, paints the selection quad under the text and the caret
  over it, and caches the shaped line + bounds per node id so a *later*
  mouse-down can be resolved (mouse events carry a position and nothing else).
- Click sets a collapsed range; drag extends from an anchor; `on_mouse_up_out`
  ends a drag that finished outside the field.
- Blink: solid on any value or caret change, then a ~1.06s cycle, driven by
  `request_animation_frame` **only while focused**, so an idle window still
  parks at zero repaints.

**Focus is observed, not inferred.** The backend owns a `FocusHandle` per
focus-tracking node, created lazily in the paint pass (the first place with an
`App`) and attached on the next build — which has to request a repaint, or the
handle sits unattached forever and never sees a focus. Gains *and* losses are
reported through `on_focus_change`.

**Hover no longer eats the ring**: while a node actually holds focus, the focus
patch is re-applied on top inside the hover closure, so focus gets the last
word regardless of gpui's refine order.

**Focus is inherited down the subtree.** A field's caret sits on the value
node, several levels below the focusable root. The first attempt made every
input node focus-tracking, which was worse than useless: gpui focuses the
*innermost* focusable element under the pointer, so clicking a field focused
the value node — an element with no key listeners. Clicks focused something
that could not type. The value node now learns it is focused by inheritance.

## A Real Bug The Specimens Exposed

Typing "abc" into the text-input specimen produced **"cba"**. The specimen
stored the value but not the caret, so every keystroke inserted at index 0.
With no native editor the caret is host state exactly like the value is, and a
host that stores one without the other spells text backwards. `SpecimenState`
now carries `carets`, clamped when a value is replaced.

## Verification

Driven in the live app (field bounds read from the elements themselves, not
measured off a screenshot):

- Click at x=940 into "helloworld" → caret lands between `hellow` and `orld`.
- Drag x=920 → x=1000 → selection `(4, 10)`, wash painted over `oworld`.
- Focus a field, then another: the first reports `focused=false`, the second
  `true` — blur is real now.
- Specimen typing: `""` → `"a"` → `"ab"` → `"abc"` with the caret at 1, 2, 3.

Green: `poodle-render` 115, node backend 8, `poodle-node` 2,
`poodle-headless` 16, `poodle-specs` 230, `poodle-jetstream` 161, both preview
builds, `effigy check:gpui`, `drift:handlers`, `docs:contract-drift`.

`docs:spec-drift` and `docs:lint` fail on one unrelated in-flight change from
another thread — `stepper.md` documents `defaultCollapsed`, which `StepperSpec`
does not have yet. Nothing in this work contributes to it. (That file also
broke a build mid-run, so the repo has a concurrent editor.)

`test:native-visual --slug=text-input` differs by 0.5830%, entirely in the
header band (the old theme-button header against the current ThemeSelect and
contrast slider). The field bodies are pixel-identical, which is the useful
result: the custom element paints text exactly where the old text child did.
Baselines stay deferred, as asked.

## Closeout Fixed On The Way

`effigy docs:lint` had been aborting before it linted anything, because the
release manifest still listed `poodle-jetstream-components` (deleted in Batch
F) and tried to read its `Cargo.toml`. Recorded as a papercut on 2026-08-06 and
masking three real gaps behind it:

- `poodle-node`, `poodle-render` and `poodle-gpui-node-backend` declared release
  metadata but were in no manifest — now registered, with release-notes entries.
- The GPUI acceptance suite and parity report still claimed coverage of
  `poodle-gpui-components`; both now name the renderer-inversion crates.
- `shared-demo-app-audit.json` had a hand-maintained export count (181) that the
  generated parity report contradicts (183).

The papercut entry is removed.

## Round Two: Clipboard, Word Select, Scrolling

Clipboard turned out **not** to need an entity — `App::read_from_clipboard` and
`write_to_clipboard` are reachable from a key listener. Only IME does. The
"needs `EntityInputHandler`" note covered both and was wrong about half of it.

- **`Interaction::on_edit_insert`** — insert text at the caret, replacing the
  selection. Paste goes through it; so will an IME commit or a text drop. It is
  distinct from `on_edit_key` because it carries *content*, not a keystroke.
  Backed by `poodle_headless::text_input::insert_transition`.
- **Copy/cut/paste in the backend.** Copying an empty selection leaves the
  clipboard alone; a cut is a copy then `insert("")`; a multi-line paste
  collapses to one line, as `<input>` does.
- **`SelectGranularity`** on `on_select_range`. The backend counts the clicks —
  double for word, triple for line — and the component resolves what a word is,
  via `word_range_at` in the shared model. A run of alphanumerics or `_` is a
  word; a run of anything else is its own word, so double-clicking punctuation
  does not swallow the words either side.
- **The caret scrolls into view.** A value wider than its field shifts left to
  keep the caret visible, with a character of margin, clamped so short values
  never move. Hit-testing uses the same origin — otherwise clicks in a scrolled
  field land on the wrong character. Reset on blur.

The root node now also carries the caret. It never draws one (it has children,
so the backend does not render its intrinsic value), but key events arrive at
the focusable root, and copy/cut need to know what is selected without hunting
through the subtree. Found because cut silently did nothing.

### Driver Additions

Neither clipboard nor word-select was drivable, so the click driver gained two
things — both real gaps, not test scaffolding:

- **`--key cmd-v`** — one keystroke with modifiers, plus named keys
  (`shift-left`, `backspace`, `escape`). `--type` only ever sent bare
  characters, so no chord could be exercised at all.
- **`--click X,Y,N`** — the synthetic NSEvent hardcoded `clickCount: 1`, so a
  double click was impossible however fast it repeated.

### Verified Live

- Double-click a word → whole word selected; `cmd-x` then `cmd-v` twice →
  `helloworldhelloworld`, round-tripping through the app's own text.
- 52 characters into a 358px field → text scrolls 31.6px, tail visible, caret
  in view.

## Still Missing

- **IME and dead keys.** Still missing, but the reason given here was wrong and
  is corrected in `08-slot-carets-and-keys-that-never-arrived.md`:
  `Window::handle_input` takes `impl InputHandler`, a plain public trait with no
  entity requirement. The blocker was never architectural.
- **Undo.** No history stack anywhere in the model.
- **Jetstream draws no caret.** It reads the same `NodeCaret`, but painting it
  needs its own text measurement. Nothing regressed — its shell never owned
  selection state, so it had no caret before either.
