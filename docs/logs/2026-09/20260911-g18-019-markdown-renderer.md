# g18.019 — MarkdownRenderer shared safe/trusted rendering

Status: ready for review (queue-owned merge pending)
Date: 2026-09-11
Branch: `ns-eac944cd-2ee0-4810-bd60-0976e3270e56`
Card: `docs/roadmaps/g18/019-markdown-renderer.md`
Handoff: `docs/handoffs/20260911-105800-g18-019-markdown-renderer.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/markdown-editor.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Base: `origin/main` at `377c12f984b1fd06925f5302157090224a26a8f2`

## Outcome

Paired public Svelte and React `MarkdownRenderer` components ship through the
existing `./markdown` entry. Both editors and both renderers use one private
per-framework content path, and that path sanitizes the complete built-in
`marked` or custom `renderHtml` result by default. Raw output survives only
behind the explicit `htmlPolicy="trusted"` prop. The editor preview's unsanitized
`{@html}` / `dangerouslySetInnerHTML` injection is closed.

The renderer is editor-free (no toolbar, textarea, form behavior, editor state,
or contenteditable), renders neutral output for an empty source, keeps ordinary
document semantics unless `ariaLabel` is supplied, is SSR-deterministic, and
shares the editor preview's prose class so presentation cannot reveal policy.

## Sanitizer boundary

`packages/core/src/markdown-html.ts` adds a DOM-free, dependency-free tokenizer
sanitizer (`sanitizeMarkdownHtml`, `decodeHtmlEntities`, `MarkdownHtmlPolicy`),
exported from `@inflatable-cookie/poodle-core`. It is the smallest boundary the
contract allows: pure string functions give identical output in SSR and browser
builds, and no browser sanitizer or DOM implementation enters the package
graph (core must stay dependency-free).

Policy decisions:

- Allowlisted semantic block/inline elements; unknown elements drop the tag and
  keep their text.
- Raw-text/foreign elements (`script`, `style`, `iframe`, `object`, `embed`,
  `svg`, `math`, `template`, `noscript`, `textarea`, `select`, …) drop the tag
  and their content.
- Attributes are allowlisted per element (`title`; `a[href,title]`;
  `img[src,alt,title]`; `code/pre[class=language-*]`; `ol[start]`;
  `th/td[align]`; task-list `input` reduced to `type=checkbox` plus
  `checked`/`disabled`). Event handlers, `style`, `srcdoc`, `xlink:*`, and every
  unknown attribute are dropped.
- URL attributes are decoded once, stripped of ASCII whitespace/control
  characters (so `java\tscript:` cannot survive), checked against
  `http/https/mailto/sms/tel`, and re-emitted; anything else drops the attribute.
- Text and attributes are entity-decoded once and re-encoded, so unrecognised
  or double-encoded entities cannot be re-interpreted by the browser.
- Malformed markup degrades to escaped text; nothing the tokenizer cannot fully
  read is emitted as markup.

`packages/svelte/components/src/markdown-content.ts` and the React twin own the
single private path (`renderMarkdownHtml`) that both the editor preview and the
renderer call. Supplying `renderHtml` customizes parsing only; it never implies
trust.

## What changed

- Core: new `markdown-html.ts` plus adversarial `packages/core/test/
  markdown-html.test.ts` (56 cases), and the two new core exports.
- Shared CSS (`packages/core/src/styles/markdown-editor.css`): element styling
  moved from `.poodle-md-editor__preview` onto `.poodle-md-prose`; the preview
  keeps layout only; new `.poodle-md-renderer` root (density-derived padding)
  and `.poodle-md-renderer__content` wrapping.
- Svelte/React `MarkdownEditor`: `htmlPolicy` prop (default `safe`), preview
  routed through the shared path, preview element carries `poodle-md-prose`.
- New `MarkdownRenderer.svelte` / `MarkdownRenderer.tsx` with required `value`,
  `renderHtml`, `htmlPolicy`, `ariaLabel`, `density`, and the shared path.
- `./markdown` barrels export `MarkdownRenderer` and `MarkdownHtmlPolicy`.
- Component, SSR, and shared-path tests match editor preview output byte for
  byte and assert no editor mechanics, neutral empty output, region semantics,
  density parity, and the trusted bypass.
- Previews: dedicated `MarkdownRendererSpecimen` pages registered as web-only
  catalogue entries in both galleries, with safe, visible-policy trusted,
  custom-parser, empty, constrained-width, and density postures; component docs
  entries (including `renderHtml`/`htmlPolicy`) and regenerated docs artifacts.
- Certification: MarkdownRenderer added to the installed SSR/declaration smoke
  and both installed fixture mounts; the `./markdown`-graph proof now follows
  stable `dist/chunks/*` re-export stubs.

### Web-only boundary decision

`MarkdownRenderer` has no native counterpart and earns no parity credit (the
task explicitly reserves native/GPUI/Jetstream). The 176/175 native-boundary
denominator is pinned in `packages/gpui/native-accessibility-proof.json`,
`packages/gpui/cross-runtime-parity-report.json`, `scripts/parity-evidence-ledger.ts`,
`docs/evidence/releases/web-package-roster.md`, and `test/package-install/roster.ts`.
Rather than re-cut that reserved boundary, the Svelte barrel re-exports the
renderer outside the `default as` roster idiom the native-boundary parser reads,
with a comment stating why. It is certified through the web-only catalogue
supplement, the paired preview specimens, the component/SSR suites, and the
installed `./markdown` smoke. Spec 070 records the web-only `./markdown` export.

## Planted regression

Replacing the shared path's `sanitizeMarkdownHtml(parsed)` with `parsed` makes
8 of 25 Svelte renderer/editor/SSR cases fail (raw `<script>`, handler,
unsafe-URL, and editor-preview policy cases). Restoring the sanitizer returns
all 47 paired component/SSR cases to green.

## Explicitly not done

- No CHANGELOG mutation: the `Unreleased` section is release-gated by
  `test/package-install/scope.ts`, so the safe-default migration is recorded in
  `docs/contracts/components/markdown-editor.md` instead.
- No public Markdown AST, plugin system, parser exposure, or second Markdown
  model.
- No editing, toolbar, selection, mode, callback, or insertion behavior change.
- No native/GPUI/Jetstream work, no roster-denominator change, no workflow,
  version, release, publication, or Desktop mutation.
- `g18.011` stays held, `g18.006` stays blocked, `g18.009` stays held.

## Validation

- `bunx vitest run --project svelte-components --project svelte-components-ssr
  --project react-components` on the Markdown suites — 5 files, 47 pass.
- `bunx vitest run` (whole board) — 409 files, 4030 pass.
- `effigy test:core` — 1357 pass across 66 files (includes the 56-case
  sanitizer corpus).
- `effigy svelte:package`, `effigy react:package` — clean (dist audit, roster
  inventory, declaration emit).
- `effigy svelte:build`, `effigy react:build` — clean (pre-existing chunk-size
  warnings only).
- `effigy test:web-pack-install` — exit 0 in the clean-checkout certification
  path: installed `./markdown` SSR and declaration smoke for both frameworks,
  both installed fixture mounts, packed graph/CSS proof, and the frozen roster
  proof.
- `effigy check:svelte` — 0 errors (4 + 6 pre-existing warnings).
  `effigy check:react-components` — 12 errors, all pre-existing in untouched
  files (same count as the g18.010 baseline). `effigy check:react-preview` —
  265 pre-existing errors, none in the new specimen.
- `effigy docs:check` — exit 0 (surface audit, docs lint, snippet check, prop and
  value-domain drift, ledger and census, docs export, parity/accessibility
  reports, docs build, gate snapshot/clean).
- `git diff --check` — clean.

## Closeout

- Worker opened one non-draft PR from the queue-owned branch; the plugin owns
  review and merge. This log is updated by closeout, not by the worker.

## Continuation

Merge before the held g18.011 web-editor acceptance sweep. Do not resume
g18.006 or g18.009 from this task.
