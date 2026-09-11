# Rich-text heading mode select

Status: open — direction accepted; public toolbar configuration needs one
operator decision before promotion
Captured: 2026-09-11
Owner: Chatterbox (planning)
Source: operator UX review of the merged RichTextEditor toolbar

## Confirmed need

Replace the three separate heading buttons with one compact text-mode select.
The control must expose Normal text and Heading 1 through Heading 6. Its menu
options should preview the corresponding document typography instead of
rendering every row at one undifferentiated size.

This is a contract extension, not presentation-only work. The current shared
schema, command registry, Svelte engine, and React engine admit only heading
levels 1–3. Heading levels 4–6 must validate, render, edit, serialize, and
round-trip in RichTextEditor and RichTextRenderer on both web frameworks.

## Recommended interaction contract

- Render the heading cluster as one stable-height toolbar control. The trigger
  shows Normal text, the active heading level, or Mixed for a multi-block
  selection without one text mode.
- Keep the trigger typography stable so changing modes does not move the
  toolbar. Render the menu labels with the matching document heading scale.
- Normal text converts the selected block or blocks to paragraphs. A heading
  choice sets the exact level; it is not an on/off toggle.
- Preserve selection, focus, history, and controlled-value behavior. One user
  choice emits one document change.
- Treat the select trigger as one toolbar roving-focus stop. When its listbox
  is open, listbox keyboard handling owns arrows and selection; the parent
  toolbar must not intercept those keys.
- Constrain the menu so the H1 preview cannot cause excessive width or height.
- Exercise the behavior and visual contract in both Svelte and React specimen
  pages, including H4–H6 and the Mixed state.

Poodle's existing custom Select supports custom trigger and option rendering,
so this should compose that primitive rather than introduce an editor-specific
popup.

## Sequencing

`g18.013` already merged the current icon-button toolbar. The new task must
follow `g18.018`, which owns overlapping RichTextEditor engine behavior. It
then becomes a prerequisite of the `g18.011` three-component acceptance sweep
and therefore lands before the `0.4.0` candidate lane.

Provisional task number: `g18.020`.

## Decision before promotion

Choose the public configuration model:

1. Recommended: retain granular heading commands, extend them through
   `heading-6`, and project each contiguous admitted heading set as one select.
   This lets a consumer expose only the heading levels its document policy
   supports. Normal text is the selector's intrinsic reset option.
2. Replace individual heading commands with one composite `text-mode` toolbar
   item whose menu always exposes Normal text and all six levels. This is a
   larger public API change and removes per-level toolbar configuration.

After that choice, promote the note into the shared rich-text contract and one
bounded implementation card; remove this triage note in the same planning
batch.
