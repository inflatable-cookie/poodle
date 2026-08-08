---
title: Slot carets, and a gate that hid two components taking no keys at all
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, poodle-headless, poodle-render, node-backend, code-input, duration-input]
---

## First, The Whole Surface

Before adding anything: a **full GPUI sweep — 134 compared, 0 failing**. Many
render changes had landed since the last full run (the value node's kind, the
placeholder flag, accessible names, seven new icons) and only ~9 slugs had been
spot-checked. Nothing was hiding.

## The Bug The New Work Uncovered

`code-input` and `duration-input` were wired for keys yesterday, with component
tests proving the handlers do the right thing. Driving the preview, **not one
keystroke arrived**.

`on_edit_key` was registered inside `if let NodeKind::Input`. Both components
put their key handler on a plain container — CodeInput on the slot *row*,
DurationInput on each segment — so the backend never wired either. `drift:events`
passed, because the components genuinely accept handlers. The component tests
passed, because they call the handler directly.

Two gates and a test suite, all green, over a control that could not be typed
into. Registration is now keyed on the channels rather than the node's kind.

The preview could not have caught it either: both compat builders had
`on_change` stubs that took a handler and dropped it (`Code entry wiring remains
host-owned`), and the specimens passed gpui listeners the node path cannot use.
Both are now wired to the specimen event queue, so the preview exercises what
the gates assert.

## Slot Carets

The contract says clicking a filled slot selects that character so typing
replaces it in place. That needed a caret, which `CodeInputSpec` did not have.

- `selection_start`/`selection_end`, **Rust targets only** and documented as
  such: the web target hides a real `<input>` behind the slots and lets the
  browser own the caret, and there is no such input here.
- `code_slot_selection` and `code_insert_replacement` ported verbatim from
  `packages/core/src/code-input.ts` rather than reimplemented from the prose.
- The active slot follows the caret rather than the value length, so clicking
  slot 1 of a filled code highlights slot 1.

### The Port Contradicted My Own Test

`codeInsertReplacement` caps the caret at `length - 1`. So once a code is full
the caret sits on the last slot and **a further digit replaces it**: typing
1,2,3,4 into a three-slot code gives `124`, not `123`.

My existing Rust rule ignored the extra digit, and my test asserted that. The
web target is the parity authority, so the test was wrong, not the port — it now
records the surprising behaviour and why it is followed.

## Verified Live

Asserted through `--print-state`, not pixels — the driver's own docs warn that a
screenshot with the display link stopped shows the last drawn frame. I read one
such stale frame as "the value was cleared" before checking the state, which
said `"123"` all along.

- Type `123`, click slot 1, type `9` → `code-input-code="193"`. In place.
- Minutes segment, Up → `duration-full="01:31:00"`.
- Type `59` into minutes, Up → `"02:00:00"` — the carry, end to end, through the
  ported rules.

Green: `poodle-render` 131, `poodle-headless` 41, `poodle-specs` 231, node
backend 8, `poodle-node` 2, `ci:native` 0, and all six drift/doc gates. Visual:
GPUI 4 touched slugs unchanged, Jetstream 137 compared 0 failing.

## Still Missing

Click-to-position inside a *text* field remains the pointer path; this is
slot-granularity only. IME and dead keys are unchanged — the one genuine
architectural limit.
