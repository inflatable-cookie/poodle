# TipTap-backed rich-text editor

Status: open — ProseMirror authority confirmed; configuration model unresolved
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
wrapper stays TipTap-specific, while ProseMirror's document and schema model is
the public semantic authority. GPUI/native support may remain future work under
a separately confirmed staged admission.

Operator decision 2026-09-10: the target is structured rich-text documents,
not a WYSIWYG view over authored Markdown. Markdown source and conversion stay
outside this editor contract.

Operator decision 2026-09-10: tables belong in the first useful configuration.
Images and embeds are not needed by Bovine Desktop yet, but image support varies
by project. The reusable editor must therefore support deliberate schema
configuration rather than baking one permanently closed Poodle schema.

Operator decision 2026-09-10: ProseMirror is the document/schema authority.
Poodle must not invent a parallel engine-neutral rich-text document model.

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

Use TipTap 3 as the Svelte/React integration layer over ProseMirror. Persist and
exchange ProseMirror document JSON; do not translate it into a Poodle-authored
document schema. Keep the live TipTap editor instance, transactions, and raw
commands out of the ordinary component API.

The smallest credible Poodle package is a paired Svelte/React TypeScript web
surface behind dedicated `./rich-text` entries:

- one controlled ProseMirror JSON document value;
- one explicit schema/feature configuration and matching toolbar projection;
- exact document-change callbacks with prop-update no-echo behavior;
- read-only and disabled states, selection/focus and keyboard behavior, paste
  filtering, links, accessibility, and bounded document behavior;
- a matching read-only renderer or generation path so stored documents do not
  require a live editor to display;
- a supported-extension boundary that can vary by project without silently
  accepting or stripping nodes.

Candidate baseline configuration: paragraph, text, heading, bold, italic,
strike, inline code, link, blockquote, bullet list, ordered list, list item,
hard break, horizontal rule, code block, table, table row, table header, and
table cell.
Images should be an optional supported feature rather than part of every
project's schema. Embeds, mentions, arbitrary custom nodes, collaboration,
comments, tracked changes, Markdown conversion, and consumer save/recovery
policy stay out of the first seam.

The consumer remains responsible for its Silo/Farmyard/Bovine envelope and for
choosing the admitted editor configuration. Poodle passes the ProseMirror
document through without inventing another representation. Unsupported input
must fail closed; it must never be normalized or stripped and then emitted as
if lossless.

## Decisions needed

1. Should v1 expose only Poodle-supported feature modules/profiles, or accept
   arbitrary consumer-supplied TipTap/ProseMirror extensions? Recommendation:
   start with composable supported modules, including optional images, and
   reserve arbitrary extension injection until its typing, styling, security,
   serialization, and read-only rendering contract is explicit.
2. Is the matching read-only renderer part of the first delivery? Recommendation:
   yes, because persistence without a stable display path is incomplete.

## Next check

Confirm the extension-configuration boundary and read-only renderer scope,
then promote a component contract and roadmap task.
