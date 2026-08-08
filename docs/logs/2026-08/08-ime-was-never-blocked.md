---
title: IME was never architecturally blocked — I had just read the API wrong
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, node-backend, text-input, ime, correction]
---

## The Claim I Kept Repeating

Through this campaign I described IME and dead keys as the one *genuine
architectural limit*: they need `EntityInputHandler` via `window.handle_input`,
which needs an entity, and a `&Node -> AnyElement` backend has none.

I wrote it in a log. Then in `text-input.md` — a **contract**, which this repo
treats as source of truth. Then repeated it in summaries as settled fact.

It is wrong.

`Window::handle_input` takes `impl InputHandler`. `InputHandler` is a plain
public trait with no entity requirement; `EntityInputHandler` and
`ElementInputHandler` are just the entity-backed convenience gpui ships for
views. Any `'static` struct can implement it.

I checked this by compiling a bare struct against gpui 0.2.2 with no entity
anywhere — it builds. The mistake was reading the *example* (`examples/input.rs`,
which uses an entity because it is a view) as the *API*.

Corrected in the contract, the log, and the roadmap.

## What Landed

`ime.rs`: `NodeInputHandler`, a direct `InputHandler` implementation, registered
in paint against whichever handle currently holds focus.

- **Three encodings meet at this boundary.** The vocabulary counts characters,
  the text system counts bytes, and the platform counts UTF-16 code units. Every
  conversion is in this one file, so nothing above it knows UTF-16 exists.
  Three tests cover the surrogate-pair cases: `🎈` is one char, four bytes and
  two UTF-16 units, and an offset landing inside the pair must resolve to a
  whole character rather than splitting it.
- **Marked text** (composition in progress) is backend-owned per field, the same
  class as the blink phase and the undo history.
- **`bounds_for_range`** answers from the measured line, so the candidate window
  lands under the text being composed.
- Focus registration needed one addition: the value node draws the caret but the
  *root* holds focus, so the backend now records which field is focused and
  registers the handler against that handle.

## What Is And Is Not Proven

**Proven:** the handler registers for the focused field (observed directly), the
encoding conversions are unit-tested, and ordinary typing, selection, undo and
the visual gates are all unaffected.

**Not proven:** composition itself. Dead keys and IME need the OS input source
to process the keystroke, and this repo's driver posts synthetic `NSEvent`s that
bypass that path. I can show the handler is installed and correct by
construction; I cannot show `option+e, e` producing `é` without a human at the
keyboard. Stated plainly rather than implied by the green tests.

## A Method Note

Midway through, a driven run put text in the *wrong field* — `workspace` instead
of `name`. I treated it as a regression from the IME change. It was not: that
run was the one where `cargo run` also compiled, so the clicks landed while the
window was still starting. Two clean re-runs put the text where it belonged.

Do not measure on the run that also builds.

## Verification

`poodle-render` 131, `poodle-headless` 41, node backend 11, `ci:native` 0,
`docs:lint` 0, `docs:contract-drift` 0. Visual: GPUI 4 touched slugs unchanged,
Jetstream 137 compared, 0 failing.
