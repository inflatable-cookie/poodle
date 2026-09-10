# TipTap-backed rich-text editor

Status: open — structured-document direction confirmed; schema scope unresolved
Owner: Poodle Chatterbox
Created: 2026-09-10
Related: `../contracts/components/code-editor.md`,
`../contracts/components/markdown-editor.md`,
`../contracts/components/block-editor.md`,
`../architecture/001-poodle-system-shape.md`

## Idea

Bovine Desktop will probably need a reusable rich-text editing interface.
Assess a Poodle `RichTextEditor` wrapper over TipTap in the same posture as the
CodeMirror-backed `CodeEditor`: Poodle owns the public semantics, controls,
tokens, accessibility, package boundary, and Svelte/React alignment; the engine
and ProseMirror types stay private. GPUI/native support may remain future work
under a separately confirmed staged admission.

Operator decision 2026-09-10: the target is structured rich-text documents,
not a WYSIWYG view over authored Markdown. Markdown source and conversion stay
outside this editor contract.

## Evidence

- TipTap 3 is a headless ProseMirror-based editor whose schema is assembled
  from extensions. Its current official docs cover both Svelte and React.
- TipTap recommends JSON for persisted editor state. Its schema is strict:
  content not admitted by the configured extensions can be removed. Exact JSON
  content checking is opt-in through `enableContentCheck`.
- The open-source editor packages are MIT-licensed. Current registry versions
  observed on 2026-09-10 are `3.31.3` for core, ProseMirror packaging,
  StarterKit, React, and Markdown.
- TipTap's Markdown extension is beta. It must not become an implicit
  Markdown-authority bridge for Desktop.
- Poodle already has `MarkdownEditor`, a source-plus-preview control, and
  `BlockEditor`, a consumer-owned block-shell. Neither is a WYSIWYG structured
  rich-text editor.
- Desktop currently consumes domain-owned rich-text envelopes and separately
  treats Markdown as authored source. Poodle must not redefine those product
  formats.

Sources:

- <https://tiptap.dev/docs/editor/getting-started/install/svelte>
- <https://tiptap.dev/docs/editor/getting-started/install/react>
- <https://tiptap.dev/docs/editor/core-concepts/schema>
- <https://tiptap.dev/docs/editor/core-concepts/persistence>
- <https://tiptap.dev/docs/editor/markdown>
- <https://github.com/ueberdosis/tiptap>

## Tentative recommendation

Use TipTap 3, but do not expose TipTap JSON, ProseMirror nodes, transactions,
extensions, commands, or an editor instance as Poodle's public API.

The smallest credible Poodle package is a paired Svelte/React TypeScript web
surface behind dedicated `./rich-text` entries:

- one controlled, versioned, engine-neutral `RichTextDocument`;
- one closed initial schema and toolbar;
- exact document-change callbacks with prop-update no-echo behavior;
- read-only and disabled states, selection/focus and keyboard behavior, paste
  filtering, links, accessibility, and bounded document behavior;
- a matching read-only renderer or generation path so stored documents do not
  require a live editor to display;
- no raw extension escape hatch; later schema growth is an explicit contract
  migration.

Candidate first schema: paragraph, text, heading, bold, italic, strike, inline
code, link, blockquote, bullet list, ordered list, list item, hard break,
horizontal rule, and code block. Tables, images, embeds, mentions, custom
nodes, collaboration, comments, tracked changes, Markdown conversion, and
consumer save/recovery policy stay out of the first seam.

The consumer remains responsible for translating between the Poodle document
and any Silo/Farmyard/Bovine envelope. Unsupported input must fail closed; it
must never be normalized or stripped and then emitted as if lossless.

## Decisions needed

1. Does the first useful editor need images, tables, embeds, or domain nodes?
   If yes, name them before fixing the closed schema.
2. Should Poodle own the versioned generic document schema, or should an
   existing upstream rich-text contract become the semantic authority while
   Poodle owns only its editor projection?
3. Is the matching read-only renderer part of the first delivery? Recommendation:
   yes, because persistence without a stable display path is incomplete.

## Next check

Confirm the minimum node/mark set and schema authority with the operator. Then
reconcile the answer with the current upstream rich-text authority before
promoting a component contract or roadmap task.
