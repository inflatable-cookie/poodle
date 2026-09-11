# 012 — CodeEditor extensible language registry

Status: ready — operator-confirmed release requirement; parallel with active rich-text work
Owner: Poodle web components
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/code-editor.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../../packages/svelte/components/src/editor.ts`,
`../../../packages/react/components/src/editor.ts`
Depends on: merged g18.002, g18.008, g18.010, g18.015 and g18.016

## Outcome

Replace CodeEditor’s closed Poodle-owned language catalogue with a lazy,
consumer-supplied language registry. Consumers install and import only the
CodeMirror language packages their product needs. Poodle keeps plain text in
the base editor, owns the registry/refusal contract, and provides the narrow
CodeMirror adapter that turns consumer loaders into an opaque registry shared
by the Svelte and React wrappers.

This is a pre-1.0 contract correction. Do not preserve the closed language
union through aliases or fallback behavior.

## Ready-State Rubric

- [x] The operator confirmed that Poodle cannot own every language and made
  extensibility the required boundary.
- [x] The current closed switch and direct grammar dependencies are located in
  both web component packages.
- [x] This task can establish its own reproducible before/after package,
  emitted-chunk and runtime-loading baseline.
- [x] g18.006 remains paused and g18.009 remains dependency-queued until this
  repair and the complete editor sweep are accepted.
- [x] The operator corrected the ordering: implementation precedes final
  acceptance, and existing Queue dependencies may be changed in place.

## Decisions

- `plain-text` remains available without a registry or grammar package.
- A consumer selects a serializable language id and supplies a registry that
  maps admitted ids to lazy CodeMirror language loaders. Unknown ids, duplicate
  ids, malformed loaders, and rejected loads fail closed.
- Publish the registry constructor through an explicit CodeMirror adapter
  subpath. That adapter may type its loader against CodeMirror; the component
  prop receives Poodle’s opaque registry and does not accept arbitrary editor
  extensions, plugins, themes, keymaps, or DOM hooks.
- Svelte and React consume one semantic registry contract. Do not create
  framework-specific language catalogues or divergent refusal behavior.
- Remove direct dependencies on individual `@codemirror/lang-*` grammars from
  the web component packages. Consumers own the dependencies selected by their
  registry. Keep only substrate dependencies required by the base engine and
  adapter.
- Preserve lazy loading. Merely moving the existing closed switch or exporting
  every grammar from a barrel does not satisfy extensibility or package cost.
- Documentation and specimens show at least two consumer-selected languages
  plus plain text without making those examples a new admitted closed set.

## Dispatch manifest

- **State:** ready and path-independent from active g18.014/g18.018/g18.020;
  explicit prerequisite of g18.011 and serial before retained g18.006 resumes;
  g18.009 remains dependency-queued behind g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** CodeEditor contract and focused guides; shared editor
  types needed for the opaque registry; Svelte and React editor entries,
  language adapters, engines and wrappers; their package manifests/lockfile;
  paired specimens and focused unit/browser/package tests; one g18.012 log
- **Reserved closeout surfaces:** g18 README/index/dispatch; g18.006, g18.009,
  g18.010 and g18.011 Queue/workspace/PR state; versions, changelog/release
  notes, release/tag/publication, Desktop repository, native/GPUI/Jetstream
- **Worker:** TypeScript package/API worker comfortable with CodeMirror 6,
  Svelte, React, lazy module graphs, package exports and declaration surfaces
- **Excluded:** general CodeMirror extension escape hatch; bundled exhaustive
  language catalogue; editor feature additions; rich-text changes; release or
  Desktop mutations; workflow changes
- **Escalation:** Chatterbox for any need to expose unrestricted CodeMirror
  extensions through CodeEditor, retain all grammar dependencies, diverge the
  framework APIs, or weaken fail-closed language selection

## Work

1. Capture the current closed catalogue's initial-load, emitted-chunk,
   installed-dependency and active-loader baseline. Bind tests that fail against
   it, then preserve the same measurements as before/after evidence.
2. Replace `CodeEditorLanguage`’s fixed union with the shared language-id and
   opaque registry contract. Add the explicit CodeMirror adapter constructor
   for typed lazy loaders.
3. Wire both engines to resolve the selected id through the registry, preserve
   plain mode/plain text, reconfigure on controlled language/registry changes,
   and reject invalid or failed providers before presenting false syntax state.
4. Remove individual grammar packages from Poodle’s component dependencies.
   Prove a consumer fixture installs only its chosen grammar packages and that
   an unselected grammar is absent from both install and output graph.
5. Update paired specimens and docs with consumer-owned TypeScript/Markdown
   examples. Prove registry switching, load failure, missing id, remount, and
   Svelte/React parity in real browser tests.
6. Run focused component/package/browser checks, both preview builds, package
   declaration/export checks, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Language support is open | public type remains a fixed union or engine switch names every grammar | consumer-defined id absent from Poodle source works in both wrappers |
| Consumers pay only for choices | all `@codemirror/lang-*` packages remain direct dependencies | isolated install and bundle graph contain selected grammar, omit unselected grammar |
| Base editor stays small | importing `./editor` eagerly imports grammar adapters | import/package graph proof for plain-text-only consumer |
| Adapter stays bounded | registry accepts arbitrary CodeMirror extensions, themes or keymaps | public declarations expose only language-loader construction and opaque registry use |
| Loading remains lazy | registry constructor invokes every loader at setup | counters prove only active language loads and switching loads once as specified |
| Selection fails closed | missing id silently becomes plain text | exact thrown/refused state before false ready presentation |
| Frameworks agree | Svelte and React use different registry shapes or errors | shared type plus paired browser cases |
| Controlled switching works | new id leaves old parser active or loses text/selection unexpectedly | live switch and remount assertions with exact host state |
| Acceptance sees final behavior | g18.011 runs before the registry exists and cannot exercise it | merged registry before the sweep, with full/sparse consumer journeys in g18.011 |
| Release remains gated | candidate resumes with closed catalogue or before sweep acceptance | merged repair and operator acceptance before Queue continuation |

## Stop conditions

- Stop if implementation requires a general-purpose CodeMirror extension prop.
- Stop if package constraints make consumers install all grammar packages;
  return the exact manifest/export blocker instead of claiming modularity.
- Stop before release, Desktop, native, or retained-task mutations.

## Evidence

The 2026-09-11 Svelte preview build emitted grammar chunks while both component
packages still declared the full fixed grammar set and the engines referenced
every loader from one switch. The operator ruled that this cannot be Poodle’s
permanent boundary: language support must be extensible and consumer-selected.

## Next task

After this repair and the parallel rich-text chain through g18.020 merge,
g18.011 runs the complete four-surface sweep against the final product shape.
Obtain operator acceptance, then resume retained g18.006 task
`17ac3fee-de90-4b32-9672-1134770bb086`. g18.009 dispatches only after that
repaired candidate closes.
