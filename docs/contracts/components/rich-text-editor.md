# RichTextEditor

Status: detailed contract
Updated: 2026-09-10

## 1. Purpose

- Component name: `RichTextEditor` (paired with `RichTextRenderer` in the same
  staged package contract)
- Layer: `composites`
- Summary: configurable web rich-text editing and read-only rendering over one
  ProseMirror document and schema authority
- In scope: controlled ProseMirror JSON, curated feature configuration,
  formatting, links, lists, code, tables, optional images, toolbar projection,
  read-only display, accessibility, paste filtering, and bounded documents
- Out of scope: Markdown conversion, HTML as stored authority, arbitrary
  TipTap/ProseMirror extensions, raw editor handles or transactions, uploads,
  asset storage, collaboration, comments, tracked changes, persistence, drafts,
  saves, review, recovery, and native parity in the first admission

ProseMirror owns document and schema semantics. Poodle must not translate the
document into a second Poodle-authored rich-text model. TipTap is the private
Svelte/React integration engine. Product repositories own their document
envelopes and choose which supported Poodle feature modules are admitted.

`RichTextRenderer` is part of the first delivery. It renders the same document
against the same feature configuration without mounting a live editor.

## 2. Anatomy

```text
[Editor root]
  |-- [Toolbar]                    optional; derived from enabled features
  `-- [Editing viewport]
        `-- [ProseMirror surface]

[Renderer root]
  `-- [Document content]           non-editable; same schema and tokens
```

Tables and images remain document content, not bespoke outer widgets. A hidden
editor used as the renderer is non-conforming.

## 3. Props And Inputs

### Editor Props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `value` | `ProseMirrorDocumentJSON` | - | yes | Host-controlled ProseMirror document JSON. |
| `features` | `readonly RichTextFeature[]` | `richTextStandardFeatures` | no | Unique curated feature modules; order has no semantic meaning. |
| `toolbar` | `"auto" \| readonly RichTextCommand[]` | `"auto"` | no | `auto` derives commands from enabled features. An explicit list may only use commands those features provide. |
| `readOnly` | `boolean` | `false` | no | Preserves selection, copy, links, scrolling, and accessible reading. |
| `disabled` | `boolean` | `false` | no | Removes editing and toolbar controls from interaction and focus order. |
| `placeholder` | `string` | `""` | no | Shown only for one empty editable paragraph. |
| `ariaLabel` | `string` | `"Rich text editor"` | no | Accessible name for the editing surface. |
| `requestImage` | `(() => Promise<RichTextImageInput \| null>) \| null` | `null` | no | Host-owned asset choice for `insert-image`; valid only with `images`. Poodle inserts the returned node at the retained selection. |
| `onChange` | `((document: ProseMirrorDocumentJSON) => void) \| null` | `null` | no | Fires once for each committed user transaction that changes the document. |

### Renderer Props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `value` | `ProseMirrorDocumentJSON` | - | yes | Same controlled document representation as the editor. |
| `features` | `readonly RichTextFeature[]` | `richTextStandardFeatures` | no | Must admit every node and mark in `value`. |
| `ariaLabel` | `string \| null` | `null` | no | Optional accessible name when the rendered document is a labelled region. |

### Public Types

```ts
interface ProseMirrorMarkJSON {
  type: string;
  attrs?: Record<string, unknown>;
}

interface ProseMirrorNodeJSON {
  type: string;
  attrs?: Record<string, unknown>;
  content?: ProseMirrorNodeJSON[];
  marks?: ProseMirrorMarkJSON[];
  text?: string;
}

type ProseMirrorDocumentJSON = ProseMirrorNodeJSON & { type: "doc" };

interface RichTextImageInput {
  src: string;
  alt: string;
  title?: string | null;
}

type RichTextFeature =
  | "formatting"
  | "headings"
  | "links"
  | "lists"
  | "blockquote"
  | "code-block"
  | "horizontal-rule"
  | "tables"
  | "images";

type RichTextCommand =
  | "undo" | "redo"
  | "bold" | "italic" | "strike" | "inline-code"
  | "heading-1" | "heading-2" | "heading-3"
  | "link" | "bullet-list" | "ordered-list"
  | "blockquote" | "code-block" | "horizontal-rule"
  | "insert-table" | "add-row" | "add-column" | "delete-table"
  | "insert-image";
```

These JSON interfaces type ProseMirror's serialized node shape; they do not
define a competing schema. Node and mark meaning comes only from the configured
ProseMirror schema assembled by Poodle's supported modules.

The schema always contains `doc`, `paragraph`, `text`, and `hardBreak`.
`richTextStandardFeatures` enables every listed feature except `images`.
Projects opt into images explicitly. Embeds are not a v1 feature.

The `images` module uses the standard image node attributes `src`, `alt`, and
optional `title`. It does not upload, browse, proxy, rewrite, or persist assets.
`alt` is required; an empty value is an explicit decorative image. Consumers
own source admission and Content Security Policy. Poodle refuses executable
URL schemes. The automatic toolbar shows `insert-image` only when `images` and
`requestImage` are both present. Cancellation returns `null` and changes
nothing. Link editing uses an editor-owned URL affordance; it never invokes a
browser prompt.

### Configuration And Controlled State

- Duplicate or unknown features fail before mount.
- Controlled documents are validated exactly against the configured schema
  before editor or renderer output. Unknown nodes, marks, attributes, or invalid
  content fail closed; they are never silently dropped and re-emitted.
- Changing `features` validates `value` against the next schema before replacing
  the engine state. A valid change emits no callback. An invalid change leaves
  the prior editor intact and reports a development error.
- A user edit updates the visible document immediately and emits `onChange`.
  The next host value is authoritative. Restoring the prior value rejects the
  edit without callback echo.
- Prop-driven document, configuration, toolbar, state, or label changes never
  emit `onChange`.
- An image request retains the insertion selection while the host chooses an
  asset. Resolve inserts once when still editable and enabled; rejection or
  cancellation changes nothing and leaves focus recoverable.
- Paste and drop are user transactions. Supported content is parsed through the
  active schema and unsafe HTML is discarded before one resulting document is
  emitted. Poodle makes no lossless-import claim for clipboard HTML.

## 4. States

| State | Expected result |
| --- | --- |
| editable | Document and enabled toolbar commands may emit exact controlled changes. |
| read-only | Document remains selectable, copyable, scrollable, and link-interactive; mutation commands are absent. |
| disabled | No editor or toolbar focus entry and no user transaction. |
| empty | One empty paragraph remains addressable and may show the placeholder. |
| table selection | Cell/row/column context exposes only commands admitted by `tables`. |
| image selected | Image controls exist only when `images` is enabled. |
| rendering | Static semantic output; no contenteditable node, history, selection state, or editor instance. |
| invalid document | No partial content mounts; the component fails closed. |

## 5. Events

| Event | When | Payload |
| --- | --- | --- |
| `onChange` | One committed user transaction changes the document. | Complete ProseMirror document JSON. |

Selection, toolbar state, history, and viewport are ephemeral editor state.
TipTap editors, ProseMirror nodes, steps, transactions, selections, commands,
plugins, and extension objects never appear in public event payloads.

## 6. Accessibility

### Semantics

- The editing surface exposes a labelled multiline rich-text input.
- Headings, paragraphs, blockquotes, lists, links, code blocks, tables, and
  images keep their native semantic structure in both editor and renderer.
- Toolbar controls have names, pressed state where relevant, and one logical
  toolbar relationship to the editor.
- Tables expose row/header/cell structure. Header cells are not inferred from
  visual styling.
- Images always carry the configured alt value.
- Read-only rendering is ordinary document content. It does not announce
  itself as editable.

### Keyboard

| Key | Behavior |
| --- | --- |
| platform undo/redo | Applies one editor history transaction when editable. |
| Tab | Leaves ordinary text; follows ProseMirror table-cell navigation while in a table. |
| Escape then Tab | Always provides a focus escape from a table or other engine-owned Tab behavior. |
| toolbar arrow keys | Rove between visible toolbar controls. |
| Enter / Space on toolbar control | Runs the named command when enabled. |
| standard text and selection keys | Follow the platform editing model and IME lifecycle. |

Focus must not be trapped. Prop updates and schema-valid reconfiguration do not
steal focus. Disabled removes the full editor composition from focus order.

## 7. Layout And Bounds

- The editor and renderer fill their sized container with `min-width: 0`.
- The editor viewport owns overflow; tables may scroll horizontally inside it
  rather than widening the page.
- A supported document is at most 2 MiB when serialized as UTF-8 JSON and at
  most 10,000 nodes. Consumers refuse larger input before mount.
- Images must not establish layout beyond the content width. Intrinsic size is
  constrained while aspect ratio is preserved.
- Renderer output is deterministic for a document, feature set, and theme. It
  must not require browser editing state.

## 8. Token Usage

| Part | Semantic token purpose |
| --- | --- |
| editor / renderer root | surface, border, radius, primary text |
| toolbar | control surface, spacing, separators, pressed and disabled states |
| selection / caret | shared accent and focus treatment |
| links | semantic link colour and focus treatment |
| code | code typography and inset surface |
| blockquote | secondary text and structural border |
| table | default borders, header surface, cell focus/selection |
| image | media radius, selection outline, fallback surface |

Consumers must not need global TipTap or ProseMirror CSS. Poodle maps engine
classes to semantic tokens inside the dedicated distribution.

## 9. Runtime Notes

- Svelte and React form the first admission and share one TypeScript engine
  assembly, schema validation, feature registry, command registry, and renderer.
- TipTap and ProseMirror package versions are pinned exactly. Their runtime
  objects remain implementation details even though ProseMirror JSON and schema
  semantics are public authority.
- Poodle exports curated feature identifiers and commands, not arbitrary
  consumer extensions. New supported modules require a contract addition and
  paired editor/renderer proof.
- SSR imports do not touch the DOM. Editor creation occurs only on mount;
  teardown removes listeners, plugins, observers, and engine state.
- Rust, GPUI, and Jetstream expose no placeholder and receive no parity credit
  from this web admission.

## 10. Parity Checklist

### Strict

- [ ] Svelte and React accept and emit the same ProseMirror JSON
- [ ] configured schemas and command availability match exactly
- [ ] invalid documents and configurations fail closed before partial output
- [ ] controlled updates and rejected edits do not echo callbacks
- [ ] toolbar, read-only, disabled, paste, undo, redo, and IME behavior match
- [ ] table editing and focus escape match
- [ ] optional images are absent when disabled and semantic when enabled
- [ ] editor and renderer produce matching semantic document structure

### Visual

- [ ] editor and renderer use the same document tokens
- [ ] toolbar, table selection, links, code, blockquotes, and images map to
      Poodle semantic roles
- [ ] bounded tables and media do not widen the host page

### Implementation Freedom

- [ ] engine plugin and view composition remain private
- [ ] browser selection, clipboard, history, and IME internals may vary while
      preserving observable behavior

## 11. Compatibility Limits

- ProseMirror JSON is the sole document representation. HTML and Markdown are
  import/export concerns outside this component.
- Only Poodle-supported feature modules are accepted. Arbitrary extension,
  plugin, node-view, command, or editor-instance injection is absent.
- `images` is optional and generic; uploads, asset pickers, captions, resizing,
  transforms, and project media policy are outside v1. `requestImage` delegates
  asset choice without exposing an engine command or selection object.
- Embeds, mentions, custom nodes, collaboration, comments, and tracked changes
  require later contract work.
- Documents are portable only between consumers that enable a compatible
  feature set. Poodle never strips unsupported content to manufacture
  compatibility.

## 12. Specimen And Focused Selectors

Required specimens: standard editable document; headings and mixed marks;
links and lists; blockquote and code block; table editing; images disabled;
images enabled with semantic alt text; read-only renderer; read-only editor;
disabled; empty; invalid document refusal; 2 MiB and 10,000-node boundaries.

Required selector families:

- exact controlled change, rejection, no-echo, IME, clipboard, undo, and redo;
- schema validation, unknown node/mark/attribute refusal, and feature changes;
- toolbar derivation and explicit-command compatibility;
- table structure, editing, overflow, keyboard navigation, and focus escape;
- optional-image absence/presence, URL refusal, alt semantics, and bounds;
- image-request cancellation, rejection, stale selection, and insert-once;
- editor-to-renderer semantic equivalence for every supported feature;
- Svelte/React public-surface and packed-distribution parity;
- SSR mount/update/destroy and listener/plugin cleanup;
- explicit proof that root imports load no TipTap or ProseMirror module.

## 13. Adoption Boundary

Poodle owns the editor and renderer projection, curated feature registry,
toolbar commands, validation posture, accessibility, and tokens. Consumers own
their document envelope, enabled feature selection, image sources, persistence,
drafts, revisions, saves, review, recovery, and any Markdown or HTML conversion.

## 14. Admission And Distribution

- First admission: Svelte and React together, implemented in TypeScript over
  TipTap 3 and ProseMirror.
- Package entries: `@inflatable-cookie/poodle-svelte/rich-text` and
  `@inflatable-cookie/poodle-react/rich-text`.
- The entries export `RichTextEditor`, `RichTextRenderer`, public JSON/types,
  feature identifiers, standard feature set, and public command identifiers.
- Root package entries do not eagerly import TipTap, ProseMirror, or feature
  modules.
- Status until native admission: `web-admitted`, not parity-complete.
