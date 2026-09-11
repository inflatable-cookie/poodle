# Rich-text controlled echo resets the caret

Status: open
Date: 2026-09-11
Owner: Poodle

## Issue

Typing one character in the controlled `RichTextEditor` moves the caret to the
end of the document. Tom reproduced this in the rich-text specimen during the
g18 web-component testing sweep.

This breaks ordinary editing anywhere except the document end and violates the
contract requirement that prop updates do not steal focus or selection.

## Diagnosis

The Svelte and React specimens immediately echo every `onChange` document back
through `value`. Each engine update emits a fresh ProseMirror JSON object.

Both wrappers classify that fresh object identity as a new host push and call
`engine.update({ value })`. The engine sees JSON different from its last host
value and calls TipTap `setContent(..., { emitUpdate: false })`. Replacing the
whole document discards the live ProseMirror selection and places the caret at
the document end.

The existing host-echo regression proves that content is retained and no
second callback fires, but it does not assert that the caret or selection is
preserved. The implementation therefore satisfies its current test while
still resetting selection on every accepted controlled echo.

The defect is shared by:

- `packages/svelte/components/src/RichTextEditor.svelte`
- `packages/react/components/src/RichTextEditor.tsx`
- their mirrored private `rich-text-engine.ts` implementations

## Required outcome

- An immediate host echo of the exact document emitted by the editor is a true
  no-op: no `setContent`, remount, callback echo, history change, focus change,
  or selection change.
- A genuinely different host document remains authoritative and replaces the
  editor document without emitting `onChange`.
- A host rejection that restores the prior accepted value still rejects the
  local edit without a callback loop.
- Svelte and React carry mounted regressions that type in the middle of a
  multi-block document across repeated controlled echoes and assert the exact
  caret/selection after every character.
- Cover a non-collapsed selection replacement and IME composition so the repair
  does not preserve only the simplest collapsed-caret case.

## Scope guard

Keep ProseMirror JSON as the controlled public value and TipTap/ProseMirror as
the private engine. Do not expose transactions or selections to consumers and
do not weaken genuine host-authoritative replacement or rejection semantics.

## Next check

Promote this into the current g18 editor testing sweep before release preflight
resumes. Bind the failing mounted law in both frameworks before changing the
controlled-sync bookkeeping.
