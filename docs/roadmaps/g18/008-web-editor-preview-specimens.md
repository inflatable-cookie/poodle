# 008 — Web editor preview specimens

Status: complete — merged as `998b6ddc69f94e405b515f6bddd682a2e8916ea5` (PR #241) on 2026-09-10
Owner: Poodle web previews
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../contracts/components/code-editor.md`,
`../../contracts/components/rich-text-editor.md`,
`../../specs/070-compiled-web-distribution-contract.md`
Depends on: `g18.002`, `g18.003`

## Outcome

Make every editor surface shipped by g18.002 and g18.003 directly discoverable
and reviewable in both web preview applications. Add matched Svelte and React
catalogue pages for `CodeEditor`, `RichTextEditor`, and `RichTextRenderer`, with
real interactive components and representative configuration.

Keep all three entries `web-admitted`. Do not add them to the portable
catalogue, native parity denominator, root package barrels, or GPUI.

## Ready-State Rubric

- [x] CodeEditor and the rich-text pair are merged on main.
- [x] Neither web preview currently registers or renders their specimens.
- [x] The shared web-only catalogue supplement already supports matched Svelte
  and React navigation without claiming native parity.
- [x] The operator confirmed specimen pages in both web runtimes are baseline,
  not optional closeout work.
- [x] Required examples and native exclusions follow existing contracts.

## Decisions

- Register three separate catalogue entries: `CodeEditor`, `RichTextEditor`,
  and `RichTextRenderer`. The read-only renderer is a distinct public surface,
  not a hidden mode on the editor page.
- Use the existing shared web-only catalogue supplement so both galleries have
  identical slugs, labels, sections, search results, and direct routes.
- CodeEditor shows a useful TypeScript editing posture, controlled updates,
  language/configuration controls, and representative diagnostics/read-only
  behavior. Exhaustive language and state matrices stay in focused tests.
- RichTextEditor shows ordinary formatted documents and tables, controlled
  updates, and explicit feature configuration. Images are demonstrated as an
  optional enabled/disabled project choice; embeds remain absent.
- RichTextRenderer renders the same representative ProseMirror document and
  feature configuration without editable state.
- Specimens mount the real subpath exports and engines. Static lookalikes,
  screenshots, or root-barrel imports do not satisfy admission.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch; may run alongside g18.007 and
  retained g18.005 verification; serial before g18.006 release
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** Svelte web-only component registry, component docs,
  specimen registry and three editor specimen files; React gallery registry,
  specimen map and three editor specimen files; focused preview/navigation/
  interaction tests; generated preview documentation/artifacts; preview-only
  package manifests and `bun.lock` only if required to run the already-owned
  subpath dependencies; one g18.008 execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.002/g18.003 historical logs and task evidence, release files,
  component package APIs, Desktop repository
- **Worker:** web component/documentation worker comfortable with Svelte 5,
  React, CodeMirror, TipTap/ProseMirror, and browser interaction evidence
- **Excluded:** editor public API changes; component implementation repairs
  unrelated to mounting a truthful specimen; portable catalogue generation;
  Rust, GPUI, Jetstream, release/tag/publish, Desktop edits, images/uploads
  beyond the existing optional image feature, embeds, exhaustive test matrices,
  windowed native selectors
- **Escalation:** Chatterbox for any required public API change, engine or
  package-entry change, native admission, image-policy expansion, or inability
  to mount the merged components truthfully through their public subpaths

## Work

1. Add the three web-only entries once in the shared supplement and prove both
   web galleries expose matching navigation, search, and direct routes.
2. Build matched Svelte and React `CodeEditor` specimens around a controlled
   TypeScript document. Show useful editing, configuration, diagnostics, and
   read-only behavior without turning the page into the exhaustive test corpus.
3. Build matched Svelte and React `RichTextEditor` specimens with formatted
   content, a table, controlled change evidence, and explicit images-off versus
   images-on configuration. Do not add embeds.
4. Build matched Svelte and React `RichTextRenderer` specimens using the same
   representative ProseMirror JSON and configuration, with no editable state.
5. Add focused registry/route and browser interaction proof that each page
   mounts the real public subpath export in both frameworks and that editor
   interactions visibly update controlled state.
6. Regenerate only required web preview docs/artifacts. Run focused checks,
   both preview builds/typechecks, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Every public editor surface is findable | one export is missing, hidden, or reachable only by a guessed URL | matched navigation/search/direct-route assertions for all three slugs in both galleries |
| Pages use real components | specimen is a styled mock or imports private engine internals | source/import inspection plus mounted interaction evidence through public `./editor` and `./rich-text` entries |
| Both frameworks document the same surface | Svelte has a working page while React is absent or materially thinner | paired specimen structure/content assertions and both preview builds |
| Code editing is observable | page mounts but typing produces no controlled value evidence | browser edit proves exact visible controlled update in each runtime |
| Rich-text editing is observable | toolbar/table example is decorative or transaction never reaches host state | browser formatting or table action proves controlled ProseMirror JSON changes in each runtime |
| Renderer is actually read-only | renderer page mounts an editor or exposes `contenteditable` | DOM assertion plus identical representative document projection |
| Configuration is honest | images appear regardless of project policy, or embeds are implied | images-off rejects/omits image capability, images-on renders the admitted example, no embed claim |
| Web admission does not inflate parity | entries enter generated portable catalogue or native counts | catalogue/census checks remain unchanged apart from the web-only supplement |
| Specimens remain documentation | pages duplicate every language, command, state, and size case | review against the curated Examples rule and focused tests for exhaustive behavior |

## Stop conditions

- Stop if truthful specimens require changing an editor's public contract or
  package entry; return the exact mismatch instead of reaching into internals.
- Stop if the shared web-only supplement would change the portable/native
  catalogue or GPUI denominator.
- Stop before release work, Desktop changes, native editor work, embeds, or
  upload/storage policy.

## Evidence

Planning inspection on 2026-09-10 found no CodeEditor, RichTextEditor, or
RichTextRenderer entry in either preview registry or specimen map. g18.002 and
g18.003 had incorrectly deferred preview catalogue admission to generation
closeout despite owning preview paths. The existing `webOnlyComponents`
supplement already feeds both web galleries without entering portable codegen.

Merged. Independent exact-head `ready_to_merge` review of head
`9ad76cace3a8ac9948696d8a87b5623789ae7477`
([comment #5626460237](https://github.com/inflatable-cookie/poodle/pull/241#issuecomment-5626460237));
closeout record
`docs/logs/2026-09/20260910-g18-008-web-editor-preview-specimens.md`.

## Next task

After merge and closeout, g18.006 may perform its final source/package recheck
now that g18.005 is complete, once the operator grants explicit release authority.
