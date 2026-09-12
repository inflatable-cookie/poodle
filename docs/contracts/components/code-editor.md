# CodeEditor

Status: detailed contract
Updated: 2026-09-10

## 1. Purpose

- Component name: `CodeEditor`
- Layer: `composites`
- Summary: a web-admitted controlled code and plain-text editing surface with
  syntax, search, line numbers, diagnostics, and bounded large-document behavior
- In scope: exact text editing, semantic change intent, language mode, search,
  line numbers, diagnostic decoration and navigation, read-only and disabled
  states, keyboard/focus behavior, editor-local scrolling
- Out of scope: paths, file loading, persistence, drafts, revisions, saves,
  recovery, Markdown/frontmatter projection, review policy, diffs, validity,
  collaboration, arbitrary editor plugins, native parity in the first admission

`CodeEditor` edits the string the host supplies. It never interprets that
string as a file or claims authority over what it means. `CodeInput` remains
segmented short-code entry. `Code` remains read-only code display.
`MarkdownEditor` remains Markdown toolbar and preview composition.

## 2. Anatomy

```text
[Root]
  |-- [Gutter]                 optional line numbers and diagnostic marks
  |-- [Editing viewport]
  |     `-- [Text input surface]
  |-- [Search panel]           conditional, editor-owned
  `-- [Diagnostic message]     conditional, active diagnostic
```

The editing viewport owns both scroll axes. Syntax tokens, selections, carets,
diagnostic marks, and the input surface form one editor; a hidden textarea plus
an unrelated painted copy is not conforming.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `value` | `string` | - | yes | Host-controlled exact text. No newline, Unicode, or whitespace normalization. |
| `language` | `CodeEditorLanguageId` | `"plain-text"` | no | Serializable language id. `plain-text` is built in; every other id resolves through `languageRegistry`. |
| `languageRegistry` | `CodeEditorLanguageRegistry \| null` | `null` | no | Opaque, consumer-constructed registry mapping admitted ids to lazy language loaders, built through the substrate adapter subpath. Required for any non-plain-text `language`. |
| `lineNumbers` | `boolean` | `true` | no | Shows the logical-line gutter. |
| `searchable` | `boolean` | `true` | no | Enables the editor-owned find panel and search shortcuts. |
| `diagnostics` | `CodeEditorDiagnostic[]` | `[]` | no | Host-authored messages attached to positions in the current value. |
| `readOnly` | `boolean` | `false` | no | Keeps selection, copy, search, scrolling, and diagnostic navigation; refuses text mutation. |
| `disabled` | `boolean` | `false` | no | Removes the editor from interaction and focus order. |
| `placeholder` | `string` | `""` | no | Shown only for an empty editable value. |
| `ariaLabel` | `string` | `"Code editor"` | no | Accessible name for the editing surface. |
| `wrapLines` | `boolean` | `false` | no | Wraps long logical lines inside the viewport. |
| `tabSize` | `number` | `2` | no | Positive integer used for indentation width and tab display. |
| `tabBehavior` | `"focus" \| "indent"` | `"focus"` | no | `focus` lets Tab leave. `indent` inserts indentation; Escape then Tab leaves. |
| `performanceMode` | `"full" \| "plain"` | `"full"` | no | Explicit host choice. `plain` disables syntax tokenization but preserves editing, line numbers, search, diagnostics, and exact events. |
| `density` | `ControlDensity \| null` | `null` | no | Explicit spacing-density override. |
| `onChange` | `((change: CodeEditorChange) => void) \| null` | `null` | no | One exact callback for each committed user edit transaction. |

### Supporting Types

```ts
type CodeEditorLanguageId = string;

const CODE_EDITOR_PLAIN_TEXT: CodeEditorLanguageId; // "plain-text"

type CodeEditorLanguageLoader = () => Promise<unknown>;

interface CodeEditorLanguageRegistry {
  has(language: CodeEditorLanguageId): boolean;
  load(language: CodeEditorLanguageId): Promise<unknown>;
}

type CodeEditorLanguageRegistryInput =
  | Iterable<readonly [CodeEditorLanguageId, CodeEditorLanguageLoader]>
  | Record<string, CodeEditorLanguageLoader>;

interface CodeEditorRange {
  from: number;
  to: number;
}

interface CodeEditorDiagnostic {
  id: string;
  severity: "error" | "warning" | "info";
  message: string;
  range: CodeEditorRange;
  source?: string | null;
  code?: string | null;
}

interface CodeEditorTextEdit {
  range: CodeEditorRange;
  insert: string;
}

interface CodeEditorChange {
  value: string;
  edits: CodeEditorTextEdit[];
}
```

Poodle does not enumerate languages. `plain-text` is built in; every other
language id is consumer-defined and must be admitted by the registry the host
supplies through `languageRegistry`. The registry maps admitted ids to lazy
language loaders and is constructed through the substrate adapter subpath
(`@inflatable-cookie/poodle-svelte/editor/codemirror` and
`@inflatable-cookie/poodle-react/editor/codemirror`), whose loaders are typed
to resolve a CodeMirror `LanguageSupport`. Consumers install exactly the
grammar packages their loaders name; Poodle ships none.

Registry construction and resolution fail closed: empty ids, non-function
loaders, duplicate ids, a built-in `plain-text` entry, unknown ids, rejected
loads, and loads that do not resolve to a language extension all throw or
reject before an editor can present false syntax state. A selected language
loads lazily and at most once per registry instance; re-selecting it reuses
the memoized load. `performanceMode="plain"` never consults the registry.

Offsets count UTF-16 code units, matching JavaScript strings and CodeMirror's
document model. Ranges are start-inclusive and end-exclusive. Every edit range
is measured against the previous `value`. Edits in one callback are ordered and
non-overlapping. `value` is the exact complete result after applying them.
Poodle does not guess edit origin from browser events.

Diagnostics with an invalid or out-of-bounds range are omitted and reported
through development diagnostics. They are never clamped onto different text.
Duplicate diagnostic ids are invalid.

### Controlled State

- `value` has no uncontrolled mode.
- A user edit updates the visible editor immediately and emits `onChange`.
- The next host value remains authoritative. Supplying the old value rejects
  the edit and restores it without emitting a second callback.
- A prop-driven value change, language change, diagnostic update, or mode
  change never emits `onChange`.
- The component owns ephemeral caret, selection, viewport, folded syntax
  state, search query, and active diagnostic. None is persistence authority.

## 4. States

| State | Expected result |
| --- | --- |
| editable | Keyboard, pointer, paste, drop, undo, and redo may produce `onChange`. |
| read-only | Text remains selectable, searchable, scrollable, and copyable; mutation commands are inert. |
| disabled | No editing, search, diagnostic navigation, pointer input, or focus entry. |
| empty | Placeholder appears; line 1 remains addressable. |
| searching | Find panel stays inside the editor and does not replace host state. |
| diagnostic active | The range and message are associated; navigation announces severity and location. |
| plain performance | Syntax tokenization is absent by explicit host request; all other contract behavior remains. |

## 5. Events

| Event | When | Payload |
| --- | --- | --- |
| `onChange` | One committed user edit transaction changes the value. | `CodeEditorChange` |

Selection, scroll, search query, and diagnostic focus are editor-local. They do
not emit public callbacks. Hosts needing durable cursor or viewport state need
a separate contract amendment; DOM or engine transaction objects are never
public payloads.

## 6. Accessibility

### Semantics

- The editing surface exposes a multiline text-input role and `ariaLabel`.
- Read-only state is announced without disabling selection or copy.
- Diagnostic marks expose severity, message, and `line:column`; colour or
  underlining alone is insufficient.
- The active diagnostic message is associated with the editing surface.
- Search inputs and result counts have explicit names and status semantics.

### Keyboard

| Key | Behavior |
| --- | --- |
| platform Find shortcut | Opens the local find panel when `searchable`. |
| Enter / Shift+Enter in find | Next / previous result. |
| Escape | Closes find; when `tabBehavior="indent"`, arms one Tab press to leave. |
| F8 / Shift+F8 | Next / previous diagnostic, moving the caret to its range and announcing it. |
| Tab | Leaves when `tabBehavior="focus"`; inserts indentation in `indent` mode. |
| Escape then Tab | Leaves an editor in `indent` mode without changing text. |
| platform undo/redo | Emits one exact change when editable. |

Keyboard editing follows the platform text system, including IME. Composition
does not emit partial callbacks; the committed composition emits one `input`
transaction.

### Focus

- Keyboard navigation into the editing surface has the shared Poodle focus
  treatment. Pointer focus does not.
- The outer treatment locates newly navigated focus; it is not a persistent
  editing-state border. The first text-editing intent, including composition,
  paste, cut, undo, or redo, removes it while the caret and selection remain
  visible. Ordinary typing must not recreate it merely because the document's
  last input modality is keyboard.
- Focus leaving the component resets this local entry state. A later Tab or
  Shift+Tab entry may show the treatment again. Internal search and toolbar
  controls keep their own `:focus-visible` treatment.
- Prop updates, diagnostics, and external tab selection never steal focus.
- Opening find moves focus into its input; closing it returns focus to the
  editing surface when focus was inside the find panel.

## 7. Layout And Large Documents

- Root fills its sized container with `min-width: 0`, `min-height: 0`.
- The internal viewport is the scroll owner. The document must not expand the
  host pane or page.
- Rendering is viewport-bounded. Hidden off-screen lines must not create one
  DOM or native view per line.
- The supported document envelope is valid UTF-8 through 2 MiB inclusive.
  Hosts refuse larger sources before mounting; Poodle does not load files or
  define the refusal UI.
- `performanceMode="full"` keeps the selected syntax mode throughout that
  envelope. `plain` is an explicit host-controlled escape hatch, never an
  automatic or silent downgrade.
- Search and diagnostic navigation remain bounded to one active query or
  diagnostic traversal. Decorations outside the viewport may be represented
  as editor-engine data rather than mounted views.

## 8. Token Usage

The g18.023 dual syntax palettes are the shipped token revision; the mapping
described below is the implemented contract, not a pending target.

| Part | Semantic token purpose |
| --- | --- |
| root / viewport | surface background, default border, surface radius |
| text / gutter | primary text, secondary text, code typography |
| selection / active line | accent selection and subtle surface state |
| focus | shared keyboard focus ring |
| diagnostic error / warning / info | matching status colour plus non-colour mark |
| search match | accent tint; active match receives stronger outline |
| syntax tokens | token groups bound to dedicated light/dark syntax roles |

Syntax presentation is visible only when a non-plain language is active in
`performanceMode="full"`. The internal editor maps stable Lezer tags to
dedicated `color.syntax.*` roles: comment, keyword, string, literal, type,
callable, property/attribute, operator, punctuation, and invalid. Ordinary
identifiers keep primary text. These roles draw from independently tuned dark
and light primitive syntax palettes; light Poodle theme modes select the light
palette and any named theme may override one role without replacing the whole
palette. UI status and product-accent tokens are not syntax roles.

Every syntax foreground meets AA text contrast against its theme’s editor
panel. At least keyword, string, literal, type, and callable are perceptually
distinct in representative source; comments and punctuation may use restrained
neutrals. Invalid syntax also carries a non-colour mark. Colours resolve from
generated semantic CSS variables, so switching themes restyles a mounted
editor live without remount or grammar reload. `plain-text` and
`performanceMode="plain"` render no syntax spans and load no grammar. The
mapping stays private: no CodeMirror tags, highlight styles, or editor
extensions cross the public API. Consumers can override ordinary documented
syntax CSS variables through Poodle theming, not a component prop.

Consumers must not need global editor-engine CSS overrides. Poodle maps any
engine classes to its semantic tokens inside the component distribution.

## 9. Runtime Notes

- Svelte and React form the first complete admission. Both use CodeMirror 6 and
  expose the same value, language, diagnostic, and `onChange` semantics.
- Shared TypeScript owns engine-independent types and transaction translation.
- CodeMirror `EditorView`, extensions, transactions, decorations, and themes
  stay private. There is no arbitrary extension prop or raw editor handle.
- The implementation pins its exact base CodeMirror packages plus the
  `@lezer/highlight` tag substrate its internal highlight style imports. It
  imports no grammar package; language extensions arrive only through consumer
  loaders at runtime.
- The engine owns line-break representation: CR and CRLF load as LF, matching
  CodeMirror's document model. Astral characters, tabs, and trailing spaces
  are byte-exact. Every `onChange` payload replays exactly against the owned
  previous document; hosts that persist CRLF sources compare accordingly.
- Rust and GPUI are future admission work. They expose no placeholder and make
  no current parity claim. A later native editor uses its native text system
  against this semantic contract.
- Jetstream remains deferred under the programme-wide working rule.
- Syntax parser internals, viewport implementation, and undo storage remain
  private. No engine type crosses the public API.

## 10. Parity Checklist

### Strict

- [ ] Svelte and React preserve exact value, line endings, whitespace, and Unicode
- [ ] one user transaction emits one complete value plus prior-value edits
- [ ] prop updates and rejected edits do not echo callbacks
- [ ] language modes, line numbers, search, read-only, disabled, and plain
      performance mode have the same meaning
- [ ] diagnostic ranges refuse invalid coordinates and remain accessible
- [ ] IME, undo, redo, paste, cut, and drop preserve exact text
- [ ] Tab always has a documented keyboard exit
- [ ] a 2 MiB boundary fixture remains viewport-bounded and usable

### Visual

- [ ] Poodle themes, density, focus, selection, gutter, search, and diagnostic
      severity use the same semantic tokens
- [ ] editor fills a bounded main pane without making the page the scroll owner

### Implementation Freedom

- [ ] editor and syntax engines may differ
- [ ] token tree, line virtualization, and undo internals may differ
- [ ] platform-native selection, IME, clipboard, and accessibility adapters may
      differ while preserving observable behavior

## 11. Compatibility Limits

- Valid UTF-8 text only, through 2 MiB inclusive.
- No binary, remote stream, rich-text document, notebook, collaborative model,
  or arbitrary plugin contract.
- The language domain is open and consumer-owned. Poodle fixes no language
  vocabulary: an id the registry does not admit is refused, never silently
  downgraded to plain text; hosts request `plain-text` explicitly.
- The component prop accepts only the opaque registry. Arbitrary CodeMirror
  extensions, themes, keymaps, plugins, and DOM hooks have no path through the
  adapter subpath or the component.
- Diagnostics are annotations only. Quick fixes, code actions, hover docs,
  completion, formatting, and language servers require later contracts.
- `MarkdownEditor` may sit beside this component, but neither owns conversion
  between Markdown body and full-file source.

## 12. Specimen And Focused Selectors

Required specimens: editable TypeScript; plain text; line numbers off; search
with multiple results; mixed-severity diagnostics; read-only; disabled; empty;
wrapped long line; plain performance mode; 2 MiB boundary fixture.

Required focused selector families:

- exact controlled change, rejection, no-echo, IME, clipboard, undo, and redo;
- language registry resolution: consumer-defined ids absent from Poodle source,
  lazy single loads across controlled switching, missing-id and rejected-load
  refusal before false ready presentation, and Svelte/React parity;
- search keyboard, focus return, and result traversal;
- diagnostic coordinate refusal, F8 navigation, and accessible announcement;
- read-only, disabled, Tab exit, pointer/keyboard focus origin, and focus-ring
  dismissal on the first editing intent without mutating global modality;
- viewport ownership and 2 MiB bounded-document behavior;
- Svelte/React public-surface and packed-distribution parity;
- explicit proof that root imports do not load the editor engine.

## 13. Adoption Boundary

Poodle owns this reusable surface. A consumer owns source identity, byte
admission, language selection, diagnostics meaning, drafts, revision tokens,
save/review/recovery policy, and any conversion between source projections.
No consumer semantics enter the component API.

## 14. Admission And Distribution

- First admission: Svelte and React together, implemented in TypeScript over
  CodeMirror 6.
- Package entries: `@inflatable-cookie/poodle-svelte/editor` and
  `@inflatable-cookie/poodle-react/editor`, plus the CodeMirror language
  adapter subpaths `@inflatable-cookie/poodle-svelte/editor/codemirror` and
  `@inflatable-cookie/poodle-react/editor/codemirror` (the only supported
  registry constructor).
- Root package entries do not eagerly import CodeMirror or its language
  packages, and no Poodle web package depends on a `@codemirror/lang-*` or
  legacy-modes grammar package.
- Status until native admission: `web-admitted`, not parity-complete.
- GPUI and shared Rust are a future task and do not block the web release.
