# CodeEditor implementation hold

Status: held — contract accepted; implementation and release unapproved
Captured: 2026-09-10
Owner: Chatterbox
Source: Desktop contract 034 and g02.058 at `7c7f6f13864512189ba53949fd65534140c57597`

Operator clarification: Poodle should wrap an established editor engine rather
than build web editing machinery from scratch. The operator named Monaco as an
example and remains open on the engine and delivery route.

## Accepted planning

- `docs/contracts/components/code-editor.md` owns the reusable seam.
- `Tabs` gains the planned `TabItem.pinned` boundary needed for an immutable
  leading Details tab beside reorderable editor tabs.
- Desktop keeps source semantics, exact-byte drafts, Markdown body/full-file
  projection, diagnostics meaning, persistence, review, saves, and recovery.

## Hold

No implementation task, package release, or consumer pin is authorized by the
intake. Poodle's active-cohort rule requires Svelte, React, shared Rust,
`poodle-render`, and GPUI together; a Svelte-only shipment is not admissible.

Promotion route: operator approves one bounded Northstar task covering the
contract, the Tabs pinning amendment, focused selectors, distribution, and
active-cohort evidence. Release remains a separate approval after merge.

## Chatterbox recommendation

Use CodeMirror 6 for the Svelte and React adapters. Keep its `EditorView`,
extensions, transactions, and diagnostic objects private behind the Poodle
contract. Load only the admitted language extensions and ship the editor from a
dedicated `./editor` package subpath so ordinary Poodle consumers do not absorb
the engine.

Why CodeMirror over Monaco for this seam:

- it directly supplies viewport rendering, exact document transactions, line
  numbers, search, lint diagnostics, language extensions, read-only state, and
  accessible Tab escape;
- it is modular and does not require Monaco's VS Code model/URI/service and web
  worker integration;
- the current contract excludes completion, hover, language servers, command
  palette, and diff editing — the capabilities that would justify Monaco;
- Bovine is a Svelte/Vite/Tauri WebView app with a 2 MiB source ceiling, a good
  fit for CodeMirror's bounded editor model.

Svelte syntax support is not in CodeMirror's official language set. If Bovine
needs `.svelte` source highlighting in the first release, the task must pin and
audit a third-party language extension or remove `svelte` from the initial
closed language domain. Do not silently treat it as HTML.

Keep the active-cohort rule. The same task should build the web wrappers over
CodeMirror and the native surface over Poodle's existing GPUI text-editing
substrate. Do not release a Svelte-only component. If schedule pressure makes
that unacceptable, return to Chatterbox for an explicit architecture decision;
do not smuggle in a temporary exception.

Before dispatch, tighten two engine-facing contract details:

- decide whether public columns use UTF-16 code units, matching CodeMirror and
  common editor protocols, instead of the promoted Unicode-scalar convention;
- prove which change origins can be reported reliably across CodeMirror and
  GPUI, narrowing the closed origin set rather than guessing from DOM events.

Status: recommendation only. No engine, version, task, release, or consumer pin
is approved until the operator confirms this route.

Evidence checked 2026-09-10:

- CodeMirror guide and extension reference:
  <https://codemirror.net/docs/guide/> and
  <https://codemirror.net/docs/extensions/>
- CodeMirror Tab accessibility rule:
  <https://codemirror.net/examples/tab/>
- Monaco editor model, worker, ESM, and support posture:
  <https://github.com/microsoft/monaco-editor/blob/main/README.md>
