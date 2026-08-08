---
title: The Jetstream caret, and a duplicate every field had been drawing
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, jetstream, text-input, caret, cross-repo]
---

## Result

Jetstream draws the caret. A poodle text field renders its value as a
caret-bearing text run, the caret sits at the host's position, and it appears
only in the field that has focus.

Implemented in the sibling `jetstream` repo with the user's explicit go-ahead.

## Almost None Of It Was Missing

Jetstream already had the machinery: `Widget::TextInput` carries
`cursor`/`selection_start`, `convert_text_cursor_quads` draws the bar and the
selection highlight, and `byte_cursor_offsets` measures glyph-accurate
positions. The bridge simply hardcoded `cursor: 0`.

## The Design Question That Actually Mattered

The obvious wiring — emit a `TextInput` widget for a caret-bearing text node —
works and is wrong. That widget is an editable *control*: it brings chrome, and
nesting one inside a field painted a background over the field's own border.

So the caret rides on a new `Widget::TextWithCaret`: a label that can show a
cursor and a selection, and nothing else. A **new variant rather than fields on
`Label`** — `Label` is matched in 38 places, a new variant costs only the
exhaustive matches, and the compiler names them (there were three). The same
additive lesson `poodle-node` taught when adding fields to `NodeKind::Input`
broke every backend's `match`.

Three further details:

- **Characters to bytes at the boundary.** Poodle counts characters, Jetstream's
  text layout counts bytes. The conversion lives in the bridge and nowhere else,
  the same division the GPUI backend draws.
- **Focus gates the caret, in Jetstream.** Poodle sets a caret on every enabled
  field because it cannot know which one is focused — focus belongs to the
  renderer. Without the gate, a form sprouts a caret in every field at once.
- **Placeholder text gets no caret**, since there is nothing to place inside a
  prompt.

## The Bug It Uncovered

Every composite input was drawing its value **twice**. Poodle's field root is an
`Input` carrying the value, and its child draws the value too; the GPUI backend
guards against this explicitly, the Jetstream bridge did not.

It was invisible because both drew the identical string in the identical place —
until the child became a caret-bearing run and the two stopped coinciding. It is
plainly visible in the old baselines once you look: `Hello world` reads as
`HelloworldId`.

14 specimens changed. Every changed pixel across all 14 is **darker** and none
lighter — the signature of removing doubled text, since overlapping strokes
lighten pixels on a dark ground. Nothing was added or moved anywhere. Baselines
refreshed and re-run clean.

## Verification

Two tests in `jetstream-poodle`, because the pixel gate cannot show a caret
(nothing is focused in a static snapshot):

- Focused field draws its caret at the host's byte offset; unfocused draws none.
- `héllo` with the caret after three characters resolves to byte **4**, on a
  character boundary — the case a character/byte mix-up corrupts.

Jetstream: workspace builds, `jetstream-ui` 226 tests, `jetstream-ui-element`
12. Poodle: `ci:native` 0, GPUI slugs unchanged, Jetstream 137 compared 0
failing.

## Pre-existing, Not Mine

`jetstream-poodle`'s `representative_components_render` test **stack-overflows**,
which aborts the whole test binary. Confirmed pre-existing by stashing this work
and reproducing on the clean tree. It is why the caret tests are run by name.

## A Note On How This Went

I had this working, hit the chrome problem, judged the correct fix (the `Label`
route) too large for a sibling repo, and reverted — throwing away working code
over a scope concern the user had already waived. The scope estimate was also
wrong: the additive-variant route touched three matches, not thirty-eight.

Reverting was the mistake, not the caution. When the path is known and the work
is sanctioned, finish it.
