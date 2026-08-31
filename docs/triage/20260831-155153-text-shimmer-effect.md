# Text Shimmer Effect

Status: open
Captured: 2026-08-31
Source: https://codepen.io/editor/devongovett/pen/01a0439c-4b7f-7f44-bf84-205c514ad139
Operator intent: research the referenced GPU-optimized text shimmer and
implement a Poodle-owned version if the evidence supports it.

## Why Keep This

A text shimmer could communicate loading, streaming, or progressive activity
without replacing meaningful text. It also risks needless motion, weak
contrast, expensive paint work, and unclear semantics. Poodle needs measured
rendering evidence and a bounded product role before choosing whether this is a
component, an appearance, or a utility.

Do not copy the reference implementation or establish an API until its
technique, performance claims, and licence have been inspected.

## Later Research Questions

- What rendering technique does the reference use, and which work stays on the
  compositor rather than triggering layout, paint, or large texture uploads?
- Do browser profiling and representative content support the GPU-optimization
  claim across current engines, device scales, and low-power hardware?
- What browser-support, fallback, SSR, and hydration behavior is required?
- How should selection, copying, screen-reader semantics, forced colors,
  contrast, reduced motion, paused/offscreen state, and animation cancellation
  behave?
- Does the effect handle wrapping, multiline content, variable fonts, RTL,
  localization, long strings, and responsive resizing without artifacts?
- Which themes, tones, speeds, widths, directions, and disabled/loading states
  are legitimate contract axes rather than decorative customization?
- Is this best represented as a semantic loading component, an opt-in text
  appearance, or a reusable motion utility?
- Can shared Rust composition and GPUI express equivalent semantics and an
  acceptable visual treatment? Jetstream remains deferred under the current
  working rules.
- What does the reference's licence require before code or technique is reused?

## Promotion Route

1. inspect the reference implementation, rendering pipeline, licence, and
   stated performance rationale;
2. audit existing Poodle loading/progress semantics and candidate consumers;
3. prototype representative single-line, multiline, theme, reduced-motion,
   and offscreen cases and capture performance evidence;
4. review the product role and API shape with the operator;
5. promote the accepted contract and runtime strategy before compiling an
   implementation card.

Keep this note open until the research is promoted, rejected, or superseded.
