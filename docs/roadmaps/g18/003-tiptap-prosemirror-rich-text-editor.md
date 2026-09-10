# 003 — TipTap/ProseMirror rich-text editor

Status: complete — merged as `fb0b73732b5c0a2a9361fddd75962eccd2710b0f` (PR #237) on 2026-09-10
Owner: Poodle web components
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/rich-text-editor.md`,
`../../specs/070-compiled-web-distribution-contract.md`
Depends on: none

## Outcome

Ship paired Svelte and React `RichTextEditor` and `RichTextRenderer` surfaces
over pinned TipTap 3 and ProseMirror packages. ProseMirror JSON/schema semantics
remain authoritative. Poodle supplies curated project-level feature
configuration, a standard table-capable profile, optional images, accessible
controls, strict validation, and isolated `./rich-text` package entries.

Do not adopt the package in Desktop, publish a release, or implement native
rich-text editing.

## Ready-State Rubric

- [x] ProseMirror document/schema authority is operator-confirmed.
- [x] TipTap 3 is the paired web integration engine.
- [x] Curated composable modules are selected over arbitrary extension injection.
- [x] Tables belong in the standard configuration; images are optional per project; embeds are deferred.
- [x] The matching read-only renderer is part of v1.
- [x] Package isolation, invalid-document, accessibility, controlled-state, and renderer-equivalence oracles are explicit.
- [x] GPUI/native work, release, and Desktop adoption remain separate decisions.

## Decisions

- Persist and exchange ProseMirror document JSON. Structural TypeScript carrier
  types do not create a second schema.
- Pin the smallest maintained MIT TipTap/ProseMirror package set exactly.
- Always admit `doc`, `paragraph`, `text`, and `hardBreak`.
- Export curated feature identifiers. The standard set enables formatting,
  headings, links, lists, blockquote, code block, horizontal rule, and tables.
  Images require explicit opt-in. Unknown/duplicate features fail closed.
- Export curated toolbar command identifiers only. No raw TipTap command,
  extension, plugin, transaction, node, or editor handle crosses the API.
- Optional image insertion delegates asset choice through one async
  `requestImage` callback. Poodle retains the selection and inserts the returned
  standard image node once; uploads and asset policy stay downstream.
- `RichTextRenderer` validates and renders the same document/configuration
  without contenteditable state or a live editor.
- Controlled documents fail closed on unknown nodes, marks, attributes, or
  invalid structure. They are never silently stripped or normalized.
- Dedicated `./rich-text` entries isolate all engine weight from roots,
  `./markdown`, and `./editor`.
- First admission is web-only and labelled `web-admitted`.

## Dispatch manifest

- **State:** ready after separate operator go; serial ownership of Svelte/React
  component manifests, lockfile, rich-text engine/configuration, and generated
  web documentation
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** RichTextEditor contract; architecture/working-rule
  staged-admission paragraphs; spec 070 rich-text entries; shared TypeScript
  types/styles; Svelte and React editor/renderer sources, tests, previews,
  package manifests and distribution entries; `bun.lock`; generated web
  documentation/evidence; one execution log and one Desktop adoption request
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, release records, Desktop repository
- **Worker:** high-reasoning TypeScript/component worker comfortable with
  ProseMirror schema validation, TipTap lifecycle, SSR, accessibility, secure
  rich content, tables, and packed-library isolation
- **Excluded:** release/tag/publish; Desktop edits or pins; Markdown/HTML
  conversion; arbitrary engine extensions or handles; uploads/storage; embeds,
  mentions, custom nodes, collaboration, comments, tracked changes; GPUI,
  shared Rust, or Jetstream implementation; workflow changes; windowed runs
- **Escalation:** Chatterbox for any public API, feature-module, ProseMirror
  authority, image-policy, size-envelope, package-entry, or staged-admission
  change

## Work

1. Pin and license-audit the minimal TipTap 3 and ProseMirror package set.
   Prove SSR-safe imports before building either shell.
2. Build the shared TypeScript ProseMirror JSON validator, curated feature
   registry, standard feature set, toolbar command registry, controlled
   update/no-echo translation, and token-owned styles.
3. Implement paired Svelte and React `RichTextEditor` shells with exact
   mount/update/destroy behavior and schema-valid live reconfiguration.
4. Implement `RichTextRenderer` over the same validator, feature registry,
   schema, and tokens without a live editor.
5. Complete formatting, headings, links, lists, blockquote, code, horizontal
   rule, table editing/navigation/overflow, and optional image semantics,
   including cancellation and stale async insertion.
6. Add accessibility, keyboard escape, IME/clipboard/history, invalid-input,
   large-document, SSR cleanup, editor/renderer equivalence, root-light, and
   packed-consumer proof.
7. Record licences, generated docs/evidence, one execution log, and a Desktop
   adoption-request handoff naming the exact merge/release gate.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| ProseMirror stays authoritative | wrapper converts to a Poodle-specific block document | public declarations and controlled round-trip use ProseMirror JSON unchanged |
| Engines stay private | public prop/event imports a TipTap editor, ProseMirror node, transaction, plugin, or extension | declaration and packed-consumer engine-leak scans fail |
| Configuration is real and closed | disabling images still admits an image node, or a consumer injects an arbitrary extension | paired feature/schema refusal cases and no extension slot in declarations |
| Standard profile includes tables | default document rejects a valid table or lacks table commands | paired editor and renderer table fixtures |
| Images vary by project | image mounts when disabled, or opt-in lacks alt semantics | disabled refusal plus enabled editor/renderer and accessibility fixtures |
| Image choice stays downstream | async request inserts twice, at a stale selection, after disable, or after unmount | resolve/cancel/reject/unmount races with one exact controlled callback |
| Invalid documents fail closed | unknown node is stripped and remaining text is emitted | pre-mount refusal with zero callback and zero partial renderer output |
| Controlled state is exact | host rejection or schema reconfiguration echoes `onChange` | paired transaction/rejection/reconfiguration fixtures |
| Renderer is the same semantic surface | renderer uses HTML conversion or produces different headings/lists/tables | editor-to-renderer structure comparison for every feature |
| Keyboard remains escapable | table Tab navigation traps focus | browser proof for table navigation and Escape-Tab exit |
| Paste is safe and explicit | script-bearing HTML survives, or paste claims lossless import | sanitization fixture and one resulting controlled document |
| Root stays light | root, markdown, or code-editor import loads TipTap/ProseMirror | packed bundle graphs and archive membership checks |
| Lifecycle is SSR-safe | import touches DOM or destroy leaves an editor/plugin listener | server import and mount/remount/destroy probes in both wrappers |
| Large documents stay bounded | 2 MiB/10,000-node fixture expands the page or mounts unbounded editor views | refusal boundary and viewport/scroll-owner proof |
| No false native parity | placeholder or receipt raises GPUI coverage | evidence says web-admitted and native census remains unchanged |

## Stop conditions

- Stop if ProseMirror documents must be translated into a new Poodle schema.
- Stop if TipTap requires public engine objects or arbitrary consumer extension
  injection to satisfy the confirmed feature set.
- Stop if controlled input cannot reject invalid content before partial mount
  or silent normalization.
- Stop if editor and renderer cannot share one configured schema and semantic
  output.
- Stop if tables, optional images, accessibility, keyboard escape, SSR, or
  package isolation fail their oracle; do not weaken the contract.
- Stop before release, Desktop changes, or native implementation.

## Evidence

Merged. Independent exact-head `ready_to_merge` review of head
`efae0be0cd7cb27f0bf9c6c683151e6386e19e3f`
([comment #5625982954](https://github.com/inflatable-cookie/poodle/pull/237#issuecomment-5625982954));
closeout record
`docs/logs/2026-09/20260910-g18-003-tiptap-prosemirror-rich-text-editor.md`.

## Next task

After merge and accepted review, return to Chatterbox for release/adoption
authority. GPUI repair-tranche compilation remains a separate g18 horizon.
