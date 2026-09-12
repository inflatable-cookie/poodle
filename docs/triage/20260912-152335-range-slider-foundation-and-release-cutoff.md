# RangeSlider Foundation and `0.4.0` Cutoff

Status: open — operator architecture and runway decision required
Captured: 2026-09-12
Owner: Chatterbox (planning)
Source: operator review after g18.022–g18.025 and direct inspection of the
current Slider family

## Problem

The recent Slider fixes did not land consistently in RangeSlider. The public
components expose parallel configuration, but Svelte, React and CSS each carry
separate rendering and interaction implementations. Core already shares value,
formatting and block-layout helpers while retaining distinct one-thumb and
two-thumb state machines. The duplicated renderer layer is now the main drift
source.

This is holding Bovine Desktop behind unpublished `0.4.0`. The release should
include every deliberate pre-v1 breaking change, unblock Desktop, then move
compatible defects found by a bounded sweep into `0.4.1`.

## Recommendation

Keep `Slider` and `RangeSlider` as separate public components and contracts.
Do not replace them with a `number | [number, number]` union API or make one
public component inherit from the other.

Consolidate their private implementation instead:

- one shared slider foundation for size tokens, axis geometry, pointer
  coordinates, track/capsule, fill segments, center marker, handles, text
  clipping and effective hit targets;
- one-thumb and two-thumb machines remain distinct;
- thin Svelte and React public shells own their different value types,
  callbacks, focus stops and ARIA structures;
- one canonical style foundation carries block/embedded, horizontal/vertical,
  unipolar/bipolar and size behavior; component CSS contains only genuine
  single/range modifiers;
- RangeSlider composes the shared handle and fill primitives twice rather than
  copying Slider's rendering.

This preserves the clear consumer API while making a Slider visual repair land
once for both controls.

## Known `0.4.0` migration inventory

- Slider and RangeSlider now default to `variant="block"`; `embedded` is the
  retained compact form. `appearance`, `standard`, `track`,
  `SliderAppearance`, `RangeSliderAppearance`, the combined RangeSlider
  visible-range formatter and dead fallback helpers were removed without
  aliases.
- Markdown rendering now defaults to the safe HTML policy. Consumers relying
  on unsanitized rendered HTML must opt into `htmlPolicy="trusted"`.
- Tabs card inactive fill is a behavioral/visual change, not an API removal.
- CodeEditor, RichTextEditor/Renderer, MarkdownRenderer, editor language
  adapters, syntax palettes and rich-text heading configuration are additive.

This is not claimed as the exhaustive release inventory. A final exported API,
default and package-entry comparison against immutable `v0.3.0` must run after
the Slider foundation lands and before candidate preparation.

## Proposed runway

1. Promote a Slider-foundation and RangeSlider-parity task. Port every accepted
   Slider behavior, specimen and proof to RangeSlider while extracting the
   shared private rendering seam.
2. Promote a final `v0.3.0`→main breaking-change and package-surface audit. It
   may start discovery in parallel, but its accepted inventory must bind the
   post-foundation main head.
3. Make retained g18.006 depend on both tasks, then prepare and merge the exact
   `0.4.0` candidate.
4. Run g18.009 to tag, publish and return package evidence so Bovine Desktop
   can resume immediately.
5. Run a bounded consumer and specimen sweep after publication. Only compatible
   repairs enter `0.4.1`; any newly desired break waits for `0.5.0`.

## Decision needed

Confirm the separate-public/shared-private architecture and the proposed
pre-release order. On confirmation, promote the two pre-release tasks, update
retained g18.006 dependencies in place, repair the stale g18.025 frontier
wording, and delete this note.
