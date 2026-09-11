# g18.011 — Web editor UX acceptance sweep

Status: complete — sweep executed across all four surfaces with the repaired
syntax presentation, re-run per engine after independent review corrected the
instrument (WebKit launch, gate exit, vacuous passes, coverage disclosure);
one evidence/test PR updated at a clean pushed head; no unresolved
release-blocking finding. Operator acceptance is still required before g18.006
resumes.
Date: 2026-09-11
Branch: `ns-aad6b776-1c3e-438c-bc9c-4e8ba8750462`
Card: `docs/roadmaps/g18/011-web-editor-ux-acceptance-sweep.md`
Handoff: `docs/handoffs/20260911-g18-011-web-editor-ux-acceptance-sweep.md`
Governing refs: `docs/contracts/components/code-editor.md`,
`docs/contracts/components/rich-text-editor.md`,
`docs/contracts/components/markdown-editor.md`
Base: `origin/main` at `4781446d4` (g18.021 closeout, PR #252; first pass ran
at `a639b1b78`)

## Outcome

The pre-release UX sweep of CodeEditor, RichTextEditor, RichTextRenderer, and
MarkdownRenderer is complete. Every journey ran as a real browser session over
the public `#components/<slug>` specimen routes of both preview applications,
paired Svelte and React, under headless Chromium and headless WebKit — with
WebKit genuinely launched by the instrument (independent review of the first
evidence push found that `--browser=webkit` had only renamed the output
directory; the instrument now launches the engine named on the command line,
following the sibling-probe convention, and the two engines' capture sets are
byte-distinct).

The first sweep pass (2026-09-11, base `a639b1b78`) stopped on release-blocking
finding F1: CodeEditor painted no syntax tokens in any language, making
`performanceMode="full"` indistinguishable from `"plain"`. Chatterbox promoted
the bounded repair g18.021 (PR #252, merged `e69512038`); this resume adopted
integration main at `4781446d4` and re-proved the repaired presentation before
continuing. Nothing from the accepted blocked evidence was discarded. The
first pass's F1 evidence was Chromium-only at engine level (both wrappers);
this round re-proved F1's repair under both engines.

Final finding set: zero release-blocking findings, one retained non-blocking
follow-up (F12, present identically under both engines). The severity-ranked
capsule below is the sweep's decision record; operator acceptance of the
capsule remains the release checkpoint.

## Severity-ranked capsule

Every observation recorded by the sweep, with exact route, action, expected,
observed, framework parity, and evidence. Shared evidence instrument:
`test/g18-011-ux-sweep/probe.ts` (CodeEditor) and
`test/g18-011-ux-sweep/probe-editors.ts` (the three other surfaces), runnable
through the new `test:g18-011-ux-sweep` selectors; per-run screenshots and
machine-readable reports land in the disposable `test/g18-011-ux-sweep/out/`.

### Blocking (repaired inside the sweep window)

- **F1 — CodeEditor presented false syntax state** (first pass, base
  `a639b1b78`; repaired by g18.021 and re-proven on this resume). Route:
  `#/components/code-editor`. Action: mount the live editor with
  `language="typescript"` and the specimen registry, wait for the lazy grammar;
  switch the configured editor to `json`. Expected: visible tokenization in
  `full` mode. Observed: zero highlight spans inside `.cm-content`, every line
  the identical default colour, in TypeScript and JSON, on Svelte and React —
  Chromium evidence at first pass (WebKit was not genuinely launched in that
  round; see review correction below), and confirmed on both engines after the
  engine-launch fix — `full` vs `plain` was unobservable. Root cause: neither
  engine installed a CodeMirror highlight style and no CSS mapped token
  classes. Disposition: Chatterbox promoted bounded repair g18.021
  (`docs/logs/2026-09/20260911-g18-021-code-editor-token-bound-syntax-presentation.md`),
  merged as PR #252. Re-proof on this resume, per engine (Chromium and WebKit):
  TypeScript tokens painted (19/19 spans coloured) and JSON tokens painted on
  both wrappers; `plain-text` and plain performance mode stay unpainted
  through the same mounted editor; live theme changes restyle tokens without
  remount.

### Follow-up (non-blocking, retained)

- **F12 — React preview harness rejects the canonical deep-link spelling.**
  Route: `http://…/?theme=eclipse#/components/code-editor` (the hash form the
  Svelte preview accepts and the visual gate uses). Action: open that URL.
  Expected: the code-editor specimen mounts. Observed: the catalogue landing
  renders instead (zero editors); only `#components/<slug>` without the
  leading slash routes (`parseRoute` regex,
  `packages/react/preview/src/gallery/App.tsx`). Parity: Svelte accepts both
  spellings; React accepts one. Engine parity: identical under Chromium and
  WebKit. Severity: follow-up — internal preview harness routing, not
  component contract surface; recorded so a future harness task can align the
  routers. Owner: preview harness, next editor touch.

### Accepted (swept and conforming; one line per dimension, all paired)

- **CodeEditor editing and host echo**: typing appends through the controlled
  echo with an exact prior-value prefix; platform undo reverts and redo
  re-applies exactly on both wrappers.
- **CodeEditor syntax visibility and switching**: TypeScript and JSON tokens
  paint after g18.021; language buttons report `aria-pressed` truthfully;
  `plain-text` renders without refusal.
- **CodeEditor language loading honesty**: exactly one network fetch per
  grammar per mount (lazy, memoized), `plain-text` fetches nothing, reselection
  refetches nothing. Cost inventory: the previews install exactly
  `@codemirror/lang-javascript@6.2.5` + `@codemirror/lang-json@6.0.2`
  (Poodle packages ship none); built dists emit the grammars as their own lazy
  chunks (~91.8 kB typescript; ~2.0 kB + shared ~26.4 kB json) outside the
  main chunk.
- **CodeEditor line numbers**: the gutter reconfigures live both ways without
  remount (g18.016 behaviour re-proven on the specimen).
- **CodeEditor diagnostics**: marks mount with `data-poodle-diagnostic` and F8
  navigation announces the active message.
- **CodeEditor read-only**: select-all + typing refuses mutation while
  selection, copy, and scrolling remain.
- **CodeEditor keyboard**: Tab leaves the editor (`tabBehavior="focus"`);
  find opens with focus in its input, Enter selects the match, Escape closes
  and returns focus to the surface.
- **CodeEditor focus treatment**: pointer entry paints no outer treatment,
  keyboard Tab entry arms `data-focus-entry="keyboard"`, and the document
  input-modality attribute is never written by the editor (g18.010 behaviour
  re-proven on the specimen).
- **CodeEditor themes, density, and layout**: the surface follows
  eclipse/clay/nord; the route honors the density configuration (comfortable
  vs compact drive `data-density` and the density-driven panel spacing while
  every editor posture stays mounted); no page-level horizontal overflow at a
  900 px Desktop-like width.
- **RichTextEditor controlled echo and history**: typing echoes into the host
  JSON readout; undo reverts through the host echo.
- **RichTextEditor toolbar state**: Bold `aria-pressed` follows the selection
  (on inside bold text, off in plain text); the table context admits the
  row/column/delete actions.
- **RichTextEditor heading configuration**: the full selector offers exactly
  Normal + H1–H6; choosing a level sets it exactly (re-choosing keeps it, no
  toggle-off) and produces real `h4`/`h6` blocks; Normal converts back; a
  selection spanning paragraph and heading reads Mixed; the sparse toolbar
  offers exactly Normal + H2 + H4 and its H4 choice produces a real `h4`
  (g18.020 behaviour re-proven on the specimen).
- **RichTextEditor command postures**: the subset toolbar projects exactly
  bold/italic/link; the disabled editor is inert (`data-disabled`, `inert`,
  `contenteditable="false"`, controls removed).
- **RichTextEditor image policy**: images-on mounts the seeded offline raster
  (data URL, 96×48 decoded, visible box, zero host requests); one host request
  inserts exactly one picked image at the retained selection; images-off swaps
  the editor back while the host document retains the picked image (g18.014
  behaviour re-proven on the specimen).
- **RichTextEditor paste sanitization**: hostile pasted HTML (inline handler,
  `<script>`, `<img onerror>`) keeps its prose text, drops the executable
  markup, executes nothing, and inserts no image under standard features.
- **RichTextEditor keyboard**: Tab inside a table moves cell-to-cell;
  Escape then Tab always leaves the editor.
- **RichTextRenderer inertness**: no `contenteditable`, no ProseMirror
  instance, no toolbar anywhere on the route.
- **RichTextRenderer fidelity**: the standard document renders faithful
  semantics (h1, strong/em, safe link href, list, blockquote, code block,
  rule, bordered table with th/td); H1–H6 all project as real structure; the
  seeded image renders with geometry and alt text.
- **MarkdownRenderer safety posture**: safe output renders full prose
  semantics; explicit `htmlPolicy="trusted"` renders raw HTML under a visible
  warning label; custom `renderHtml` output passes through the same safe path;
  an empty value renders an empty root with no editor placeholder copy.
- **MarkdownRenderer inertness**: no textarea, no contenteditable, no
  markdown-editor toolbar on the route.
- **MarkdownRenderer layout, density, and themes**: a long unbroken token
  wraps inside its constrained container; no page overflow at 900 px; the
  route honors comfortable/compact density; typography follows themes.

Swept at the preview-configuration level: density. The specimens apply the
preview's density configuration to the whole route; the instruments assert the
density attribute, the density-driven spacing variables, and mounted content
under comfortable and compact for the CodeEditor and both renderer routes.

Not browser-swept, with owner and next action (dimensions the specimens or the
headless engines cannot exercise):

- **MarkdownRenderer SSR determinism** — card work item 5. A preview route is
  client-rendered, so SSR output is not observable in a browser journey. SSR
  equivalence and determinism are owned by the component suites (the
  markdown-renderer SSR checks in the paired component boards; g18.019
  validation). Owner: Poodle web quality; next action: none — covered, and
  re-verified by the component boards on this head (`effigy test:components`).
- **Clipboard system integration (copy/paste through the real pasteboard)** —
  card work item 2. Headless Chromium/WebKit cannot grant or observe the OS
  pasteboard deterministically, so the sweep drove the paste direction with a
  synthetic `ClipboardEvent` (sanitization journey above) and proved read-only
  selection retention for copy. Round-trip clipboard/undo exactness is owned
  by the component suites (CodeEditor clipboard/undo/redo cases) and the
  g18.018 controlled-echo probe. Owner: Poodle web quality; next action:
  none under this sweep — revisit only if a clipboard regression escapes the
  suites.
- **IME composition** — card work item 2. No headless engine can drive a real
  input-method composition. IME lifecycle is owned by the component suites
  and the g18.018 IME insertion journey (synthetic composition events over
  real editors). Owner: Poodle web quality; next action: none under this
  sweep.
- **CodeEditor `tabBehavior="indent"`, `wrapLines`, `placeholder`, empty and
  2 MiB postures; RichTextEditor `readOnly`; link editing UI** — not exposed
  by the shipped specimens. Owned by the CodeEditor/RichTextEditor component
  suites (indent/escape, wrap, placeholder, 2 MiB boundary, read-only,
  link-editor cases). Owner: Poodle web quality; next action: a future
  specimen task may expose these postures if the operator wants them
  browser-swept.

## What changed

- `test/g18-011-ux-sweep/probe.ts`: the CodeEditor sweep instrument. Paired
  Svelte + React journeys over the real preview routes with network recording
  (lazy grammar loads), computed-style token proof, keyboard/pointer focus
  journeys, theme, density, and constrained-layout checks, and per-run
  screenshots. Launches the engine named by `--browser` (Chromium/WebKit map,
  sibling-probe convention) and exits non-zero when a blocking finding is
  recorded, so the selectors are a real gate.
- `test/g18-011-ux-sweep/probe-editors.ts`: the same instrument shape for
  RichTextEditor, RichTextRenderer, and MarkdownRenderer (same engine and
  exit conventions, plus a renderer density journey).
- `tasks/effigy.tasks.toml`: `test:g18-011-ux-sweep{,-code-editor,-editors}`
  selectors (Chromium + WebKit each), matching the sibling g18 probe
  convention; not composed into `ci:web`.
- This execution log, including the review-round corrections: the first
  evidence push claimed WebKit coverage the instrument did not perform; that
  claim is withdrawn and replaced by the re-run paired-engine results below.
- No product code, contract, package manifest, release file, Desktop file, or
  workflow was touched.

## Explicitly not done

- No product repairs from the sweep: F1 was routed to and repaired by g18.021;
  F12 stays recorded for the preview-harness owner.
- No API, language-registry, or release-candidate decisions; no g18.006/g18.009
  work; no publication or tagging; no Desktop or native work.

## Validation

- `effigy test:g18-011-ux-sweep` — both engines genuinely launched
  (`--browser` selects the engine; per-engine captures are byte-distinct,
  zero MD5-identical screenshot pairs across engines):
  - CodeEditor: Chromium 61 passes, 1 finding (F12 follow-up), 0 blocking;
    WebKit 61 passes, 1 finding (F12 follow-up), 0 blocking.
  - RichTextEditor + renderers: Chromium 70 passes, 0 findings; WebKit
    70 passes, 0 findings.
  - A blocking finding now fails the selector (exit 1), matching the sibling
    probe convention.
- Focused editor browser board green on this head:
  `test:code-editor-focus-entry` (68 ok), `test:code-editor-line-numbers`
  (56), `test:code-editor-language-registry` (132),
  `test:rich-text-image-policy` (142), `test:rich-text-controlled-echo`
  (36), `test:rich-text-heading-mode` (96),
  `test:markdown-editor-preview-scroll` (40).
- `bun install` after adopting `4781446d4` (g18.021 added the pinned
  `@lezer/highlight@1.2.3` substrate); `effigy svelte:package`,
  `effigy react:package` clean.
- `effigy svelte:build`, `effigy react:build` — clean from this exact head
  (pre-existing chunk-size warnings only).
- `effigy test:components` — component boards green (SSR/clipboard/IME
  coverage cited in the disclosure above re-verified on this head).
- `effigy test:a11y` — pass. `effigy docs:check` — pass.
- `git diff --check` — clean.

## Continuation

The sweep is complete with no unresolved release-blocking finding. The
evidence/test PR now awaits independent review. After merge, the operator
accepts or rejects the sweep capsule; acceptance is the gate for resuming
retained g18.006 task `17ac3fee-de90-4b32-9672-1134770bb086`, with g18.009
still dependency-queued behind it. F12 belongs to the preview harness owner,
not to a release blocker.
