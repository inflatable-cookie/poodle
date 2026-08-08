---
title: The caret work doubled every field in the accessibility tree
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, accessibility, poodle-render, node-backend, text-input]
---

## Found By Running The Gate That Was Already Failing

`test:jetstream-a11y` has been failing on unnamed `TextInput` nodes, recorded as
pre-existing and held under `g12.015`. The count was **151**.

It is now **325**, and the increase is mine.

The caret work gave every text field a nested value node built with
`Node::input`, so each field rendered *two* inputs: the field root, which
carries the accessible name, and the value node, which carries none. Every
field became two controls to a screen reader, one of them anonymous.

That is worse than the unnamed nodes the gate was already counting. Those are
missing names; this was inventing controls that do not exist.

Worth stating plainly: I had run `ci:native` earlier in this campaign and read
"151 unnamed, pre-existing" as a known number rather than a number to re-check
after changing every input in the repo. The gate was doing its job; I was
treating its output as a constant.

## Fix

The value node is now a **text** node that carries a caret, not an input.

`NodeCaret` was already a field on `Node` rather than on `NodeKind::Input` —
that decision was made so backends that had never heard of carets kept
compiling, and it turns out to be exactly what makes this possible. The caret
channel is what asks the backend to measure and draw; the node's *kind* is free
to be whatever the accessibility tree needs it to be.

- **Render**: the value is `Node::text(display)` with `.with_caret(..)`. The
  field root stays the only `NodeKind::Input`, and keeps the name and the focus.
- **Backend**: the `Text` branch renders the measuring `input_text` element when
  a caret is present. Multi-line content still falls through to plain wrapped
  text, as the `shape_line` panic fix requires.
- **Pointer selection is keyed on the channel, not the kind.** It had been
  registered inside `if let NodeKind::Input`, so moving the value node to text
  silently unhooked click-to-position and drag-select.

Count back to **151** — exactly the pre-existing baseline, with the regression
gone and nothing else moved.

## A False Alarm Worth Recording

Between the two changes, a driven click-type-drag showed an empty, unfocused
field, and I read it as the fix breaking selection. It was not: a lone `--click`
can land before hover registers, because gpui gates mouse-down on
`hitbox.is_hovered`, which is the *last painted frame's* state. The first click
never focused, so the typing went nowhere and the drag had nothing to select.

Two probes I added to chase it silently failed to match their anchor text, which
made a working handler look dead and cost more time than the bug. A probe that
does not assert its insertion point is not evidence.

Clicking twice — the workaround already recorded for this driver — showed the
selection painting correctly.

## Verification

- `test:jetstream-a11y`: 325 → 151 unnamed, matching the pre-existing count.
- Live: click, type, drag → `helloworld` with `oworld` washed, caret and ring
  correct.
- `test:native-visual` on `text-input`, `field`, `token-input`, `embed-input`,
  `command-palette`, `editable-label`, `time-input`: **0 failing**. The node
  kind changed; not one pixel did.
- Green: `poodle-render` 129, `poodle-headless` 30, `poodle-node` 2, node
  backend 8, `effigy drift:events`, `drift:handlers`, `check:gpui`, `docs:lint`.

The component test now asserts the shape directly: exactly one
`NodeKind::Input` in a rendered field, so a nested input cannot come back
unnoticed.

## Still Open

The original 151 remain, and they are the real `g12.015` work: specimens that
render fields without an `aria_label`. Unchanged by this, and still the last
thing between `ci:native` and green.
