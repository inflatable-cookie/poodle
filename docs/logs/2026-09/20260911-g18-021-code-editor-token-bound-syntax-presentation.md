# g18.021 — CodeEditor token-bound syntax presentation

Status: complete — merged as `e69512038a4f032ecfad398562bbab28d40e9ffc` (PR #252) on 2026-09-11 after exact-head independent review (PR comment `5640455224`, `ready_to_merge`) at `d831b559c6e5cf4d2e4e9fd233043641422f4c8d` with green rust/web checks
Date: 2026-09-11
Branch: `ns-7ff96ca9-20a1-43e4-afa3-f2025c8f6409`
Card: `docs/roadmaps/g18/021-code-editor-token-bound-syntax-presentation.md`
Handoff: `docs/handoffs/20260911-g18-021-code-editor-token-bound-syntax-presentation.md`
Governing refs: `docs/contracts/components/code-editor.md`,
`docs/specs/070-compiled-web-distribution-contract.md`,
`packages/svelte/components/src/code-editor-engine.ts`,
`packages/react/components/src/code-editor-engine.ts`,
`test/code-editor-language-registry/`
Base: `origin/main` at `b009920af1f159634052b8519050623ba32e8134`

## Outcome

`CodeEditor` now presents token-bound syntax in `performanceMode="full"` for
every registry-admitted non-plain language, in both web wrappers. The paired
engine twins install one private CodeMirror `HighlightStyle` (with
`syntaxHighlighting`) whose rules bind stable Lezer tag groups to Poodle
semantic CSS variables: comments and metadata to secondary text, the keyword
family to accent, strings to success, numbers/booleans/constants to info,
types and definitions to warning, and `tags.invalid` to danger. Ordinary names
stay unmapped and inherit primary text. Because every rule resolves a CSS
variable, switching themes (Eclipse ↔ Iceberg proven live) restyles a mounted
editor without remount or engine recreation. Grammars remain consumer-owned
through the g18.012 registry: no grammar package becomes a Poodle dependency,
manifest entry, lockfile entry, or eager emitted module. `plain-text` and
`performanceMode="plain"` install no presentation and load no grammar.

## What changed

- Both engine twins (`packages/svelte/components/src/code-editor-engine.ts`,
  `packages/react/components/src/code-editor-engine.ts`, byte-identical apart
  from the header comment): a private `codeEditorHighlightStyle`
  (`HighlightStyle.define`) maps the tag groups above; a private
  `invalidSyntaxHighlighter` ViewPlugin marks visible Lezer parser error
  nodes (node types with `type.isError`) with a
  `poodle-code-editor__syntax-invalid` mark whose inline style is
  `var(--poodle-color-status-danger)`. Both ride the language compartment
  through `resolveLanguageExtension`, so they are installed exactly when a
  full-mode non-plain language is active and are removed when plain text or
  plain performance mode is selected — same mounted editor, no remount.
  Rationale for the plugin: Lezer error recovery nodes carry no tag, so a
  tag-bound style alone cannot reach them; the JS and JSON grammars emit no
  `tags.invalid` nodes. The tag rule stays for grammars that do.
- Both shell manifests, `bun.lock`, the distribution build spec
  (`scripts/web-distribution/shell-contract.ts` `CODEMIRROR_EXTERNAL_MODULES`),
  and `docs/specs/070` gain the exact `@lezer/highlight@1.2.3` dependency —
  base editor presentation machinery (the tags imported by the internal
  style), externalized like the pinned base `@codemirror/*` set, never
  bundled into `dist/`, and never a grammar package.
- Contract clarification (`docs/contracts/components/code-editor.md` §8/§9):
  documents the proven presentation semantics (token-group → semantic-token
  mapping, live theme response, plain-mode absence, private mapping) and the
  pinned `@lezer/highlight` substrate.
- Paired registry fixture (`test/code-editor-language-registry/`): the
  consumer sample documents now exercise comment/keyword/definition/number/
  string token groups (TypeScript) and string/number/boolean groups with
  ordinary property names (JSON); the harnesses gain full/plain performance
  mode buttons, TypeScript/JSON/malformed sample-document buttons, and the
  probe (`probe.ts`) extends the g18.012 paired Chromium/WebKit board with
  the g18.021 checks described below.
- No public API, prop, type, or preview routing change. g18.012 lazy loading,
  memoization, live switching, rejection, and package-cost behavior are
  untouched.

## Planted regression (before/after)

The probe's new g18.021 checks were run against the pre-repair engine (engine
twins reverted, fixture and probe already extended): 30 paired failures —
15 per framework — exactly the real-grammar proof surface: zero styled spans
for the real TypeScript grammar, no accent/secondary/success/info/warning
token colours, no JSON token colours, no danger treatment for parser error
nodes, and no theme-responsive tokens. All pre-existing g18.012 checks kept
passing, so the failure is precisely the missing syntax presentation. With
the repair applied, every check passes in Chromium and WebKit for both
frameworks.

## Validation

- `effigy test:code-editor-language-registry-chromium` and
  `-webkit` — all checks pass for both frameworks in both engines (33 checks
  per framework per engine): the g18.012 board unchanged (lazy single loads,
  memoized re-selection, live switching, typed text and undo history across
  switches, fail-closed rejected loads, mount-time unknown-id refusal) plus
  the g18.021 proofs — real TypeScript spans for comment (secondary),
  keyword (accent), definition (warning), number (info), and string
  (success) with computed colours exactly matching the live CSS variables;
  real JSON spans (number/boolean info, string success, ≥2 distinct colours,
  property names ordinary); a malformed TypeScript sample whose parser error
  nodes receive the danger token and lose it when the document is repaired;
  zero styled spans for plain-text language; full→plain performance mode
  dropping spans and loading no grammar on the same editor node; full mode
  reinstating presentation without remount; live Eclipse→Iceberg→Eclipse
  theme switches restyling keyword and comment tokens while the editor node
  identity (probe-installed JS property plus `data-probe-id`) is preserved;
  registry load counters memoized through every language and mode toggle.
- `bunx vitest run packages/svelte/components/test/CodeEditor.test.ts
  packages/react/components/test/CodeEditor.test.tsx
  packages/svelte/components/test/CodeEditorPackaging.test.ts` — 3 files,
  101 pass (packaging suite now pins `@lezer/highlight@^1.2.3`-style exact
  specifier alongside the base set and still refuses every grammar package).
- `bunx vitest run --project svelte-components --project react-components`
  — 363 files, 3065 pass.
- `bunx vitest run --project svelte-preview --project react-preview` — 126
  pass. `bunx vitest run --project a11y` — 183 pass.
- `effigy svelte:package`, `effigy react:package` — clean (declaration emit,
  receipt, staged-dist, dependency audits; grammar modules still forbidden).
  Both emitted `./editor` graphs import `@lezer/highlight` as an external,
  never a bundled copy.
- `effigy svelte:build`, `effigy react:build` — clean (pre-existing chunk
  size warnings only).
- `effigy test:web-pack-install` — clean: the isolated packed-install
  consumer still resolves `@lezer/highlight` through the shells' pinned
  dependency, constructs registries through both shipped adapters, and
  proves every unselected grammar package stays absent from the install
  graph (file absence plus failed dynamic import).
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  `effigy check:react-components` — 12 errors, all pre-existing and
  byte-identical with and without this branch's changes (documented
  backlog). `effigy check:svelte-preview` — 0 errors. `effigy
  check:react-preview` — 265 pre-existing errors, unchanged by this branch.
- `effigy docs:check` — pass. `git diff --check` — clean. Merge gate: PR #252 merged as `e69512038a4f032ecfad398562bbab28d40e9ffc` with green rust/web checks; the first `web` run failed on a transient preview-preflight listener timeout unrelated to the diff and passed on retry.

## Explicitly not done

- No public CodeMirror extension, highlight-style, tag, theme, or engine
  surface; the adapter and component APIs are unchanged.
- No grammar package dependency, no bundled language catalogue, no parser
  changes, no editor redesign.
- No preview-routing repair (g18.011 F12 stays in the sweep finding set), no
  g18.011 retry or merge, no release/publish/tag, no Desktop, native, or
  workflow mutation.

## Continuation

g18.021 is merged. The existing blocked g18.011 task
is retried from this head with this task added to its dependency graph; the
four-surface sweep resumes on the same task thread, g18.006 stays closed
until operator sweep acceptance, and g18.009 stays dependency-queued.
