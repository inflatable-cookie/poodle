---
title: CodeInput and DurationInput take keys, and the ratchet empties
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, poodle-headless, poodle-render, code-input, duration-input, gates]
---

## The Debt

`drift:events` landed with two entries in `KNOWN_GAPS`: `code-input` rendered
slots and `duration-input` rendered segments, and neither took a keystroke.
Both were listed as debt rather than accepted, precisely so they would be
closed. This closes them.

## CodeInput

The contract's web target hides a real `<input>` behind the slots and lets the
browser own typing. There is no such input on the Rust targets, so the **slot
row** takes focus and the keys and the slots stay pure visuals — the same
division, reached differently. One focus stop, not six: the value is a single
string and the active slot is derived from its length.

Rules in `poodle_headless::text_input`, next to `edit_transition`:

- `code_transition` — a digit appends, backspace removes the last, and both
  stop at `length`.
- `code_paste` — sanitize and clamp, because a one-time code is far more often
  pasted than typed. Wired through `on_edit_insert`, the channel the clipboard
  work added.

Two rules worth stating because they are easy to get subtly wrong:

- **A key can be ours and still change nothing.** A letter typed into a
  digits-only code is *consumed*, not forwarded — otherwise it falls through
  and triggers a submit. Same for backspace on an empty code.
- **`onComplete` fires on the transition into a full code**, not on every
  keystroke while it is full. A no-op keystroke reports nothing at all.

`Escape` clears a filled code and passes through an empty one, so a dialog
containing one still closes on the second press.

## DurationInput

The contract names `@poodle/headless`' `duration.ts` as the authority for
segment semantics, and `poodle-headless` had no equivalent. `duration.rs` is a
faithful port of `packages/core/src/duration.ts` — not a reimplementation from
the prose, because the interesting behaviour is not in the prose:

> **A carry at the hour bound is swallowed, not clamped into a partial change.**
> 59 minutes stepping up at max hours stays put rather than rolling the minutes
> to 0 and silently losing an hour of entry.

Each segment is separately focusable, which is what the contract's Tab and
Shift+Tab rows describe: focus moves between segments, arrows and digits act on
whichever holds it. Digits **shift** into a two-digit segment (0 → 4 → 45 → 56)
the way a clock field behaves, rather than replacing.

`on_change` reports all three segments plus the total, after carry — the
payload the contract's callback table documents.

Keys that are not ours pass through untouched, so Tab still moves focus and
Enter still submits.

## The Ratchet Worked

With both wired, `drift:events` **failed**:

```
FAIL — code-input, duration-input no longer inert; remove from KNOWN_GAPS
       so it cannot regress.
```

That is the half of the ratchet that is easy to leave out, and the half that
stops a debt list becoming permanent furniture. `KNOWN_GAPS` is now empty, with
a comment saying both entries were closed rather than accepted.

## Verification

Nine new component tests pin the parts that only exist in the component: one
focus stop for a code and three for a duration, completion firing once, no-op
keys reporting nothing, disabled inputs taking no keys at all, and pass-through
for Tab/Enter/Escape. Fourteen new headless tests cover the shared rules,
including every carry and borrow direction and the swallowed bound.

Green: `poodle-render` 129, `poodle-headless` 30, `poodle-node` 2, node backend
8, preview builds, `effigy drift:events`, `drift:handlers`, `check:gpui`,
`docs:lint`, `docs:contract-drift`, `git diff --check`.

`test:native-visual` on `code-input`, `duration-input` and `embed-input`:
**0 failing**. Behaviour only — no pixel moved.

## Not Done

- **Click a slot to position the caret.** The contract says clicking a filled
  slot selects that character so typing replaces it in place. Typing appends
  and backspace removes from the end; there is no per-slot caret yet.
- **`onParse`** on `embed-input` stays the host's: the host does the parsing and
  the spec arrives with the result resolved.
