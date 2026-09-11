# Standalone MarkdownRenderer

Status: open
Date: 2026-09-11
Owner: Poodle web components

## Idea

Add a public `MarkdownRenderer` matching the role of `RichTextRenderer`.
`MarkdownEditor` already contains a read-only preview path; consumers should not
have to mount editor chrome or duplicate that rendering just to display a
Markdown document.

Tom confirmed the intended shape is a wrapper around the existing editor
renderer, not a second Markdown model or parser. As with the other new web
editor surfaces, the component needs dedicated Svelte and React specimen pages.

## Current evidence

Both `MarkdownEditor` wrappers currently:

- parse the current string with `marked.parse()` unless `renderHtml` is supplied;
- inject the resulting HTML into `.poodle-md-editor__preview`;
- use the same shared preview CSS;
- keep the preview code inline inside the editor shell.

There is no standalone public renderer or shared private preview component.
The existing `./markdown` entry already carries `MarkdownEditor` and the
`marked` dependency boundary, so `MarkdownRenderer` belongs there.

## Recommended seam

- Extract one private Markdown rendering/content surface per framework and use
  it from both `MarkdownEditor` preview mode and public `MarkdownRenderer`.
- Keep one public source prop, preferably `value: string`, matching
  `RichTextRenderer`'s controlled-document naming.
- Preserve the current `renderHtml(markdown)` customization seam unless the
  safety decision below requires a more explicit name or contract.
- Render semantic document content only: no textarea, toolbar, editor state,
  form behavior or contenteditable surface.
- Reuse the exact preview typography, link, code, blockquote, list and overflow
  styling. Empty Markdown should have a deliberate renderer posture rather than
  inheriting editor chrome accidentally.
- Require deterministic SSR and matching Svelte/React output for the admitted
  Markdown subset.
- Add separate `MarkdownRenderer` specimen pages in both web previews and bind
  packaging through `./markdown`.

## Safety gap

The current editor preview sends `marked.parse()` output directly through
Svelte `{@html}` and React `dangerouslySetInnerHTML`. `marked` does not sanitize
HTML. Raw HTML in a Markdown value, or unsafe HTML returned by `renderHtml`, can
therefore become live DOM.

That behavior should not be copied into a general document renderer without an
explicit policy. Recommended default: the shared renderer sanitizes all parsed
HTML, including custom-renderer output, with links and media governed by a
closed safe policy. Do not add an implicit trusted-HTML escape hatch. If a
consumer genuinely needs trusted arbitrary HTML, that should require a
separately named opt-in contract rather than overloading ordinary Markdown.

The same shared path should then harden `MarkdownEditor` preview behavior so
editor and renderer cannot drift.

## Unresolved operator decision

Confirm the raw-HTML posture:

1. recommended — safe Markdown only by default; sanitize built-in and custom
   renderer output, with no trusted arbitrary-HTML escape hatch in this task;
2. retain trusted raw HTML as an explicitly named opt-in for controlled
   consumers while keeping the default safe.

## Next check

After the raw-HTML posture is confirmed, promote one bounded g18 web task for
the shared renderer extraction, paired public components, editor reuse,
security regressions, package certification and both specimen pages. Keep it
before the held editor acceptance sweep and release candidate.
