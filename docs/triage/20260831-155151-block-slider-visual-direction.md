# Block-Style Slider Visual Direction

Status: open
Captured: 2026-08-31
Source: operator-supplied visual reference
Operator intent: move Slider and RangeSlider toward a block-style treatment
with inline labels and values, a substantially larger track, and smaller,
visually subordinate thumbs.

## Reference Direction

The supplied reference is a large horizontal rounded capsule. Its active black
block contains the inline label `Blur`; the remaining light-grey block contains
the inline value `67 px`. The track is the dominant object and no conventional
thumb is visible. Preserve this description as the durable lead; the temporary
attachment path is not part of the record.

This is a visual and interaction direction, not approval to copy a particular
implementation or silently change the current component contract.

## Later Research Questions

- How does the block treatment express a single value and a bounded range,
  including two values, overlapping thumbs, and a narrow selected interval?
- Where do the label, current value, unit, min/max context, marks, and tooltip
  belong without obscuring the selected extent or reducing legibility?
- How should horizontal and vertical orientation, step behavior, disabled,
  read-only, invalid, and indeterminate states render?
- What visible focus, hover, pressed, and active-thumb treatments preserve
  keyboard and pointer clarity when the thumb is intentionally small?
- Can the visible thumb remain small while its touch target meets accessibility
  requirements? What contrast and high-contrast-mode treatment is required?
- Which measurements and colors should become shared tokens, and which belong
  only to a Slider appearance?
- Can Svelte, React, shared Rust composition, and GPUI preserve the same
  semantics and state presentation? Jetstream remains deferred under the
  current working rules.
- What consumer breakage would a new default create? Should this replace the
  current appearance or begin as an additive appearance pending adoption?

## Promotion Route

1. audit the current Slider and RangeSlider contracts, implementations, demos,
   tests, tokens, and consumers;
2. prototype representative single-value and range cases across interaction,
   layout, accessibility, and theme axes;
3. review the visual direction and default-versus-additive decision with the
   operator;
4. promote the accepted semantics and tokens into contracts and architecture;
5. only then compile implementation and migration cards.

Keep this note open until the direction is promoted, rejected, or superseded.
