# 002 — CodeMirror web CodeEditor

Status: complete
Owner: Poodle web components
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/code-editor.md`,
`../../contracts/components/tabs.md`,
`../../specs/070-compiled-web-distribution-contract.md`
Depends on: none

## Outcome

Ship a typed Poodle `CodeEditor` for Svelte and React over pinned CodeMirror 6
packages, plus the existing Tabs contract's pinned-item behavior across its
normal cohort. Produce merge-ready web packages and evidence for a later
release. Do not implement GPUI CodeEditor or adopt the package in Desktop.

## Ready-State Rubric

- [x] CodeMirror 6 and the web-first admission are operator-confirmed.
- [x] Public editor semantics and product ownership are canonical.
- [x] The task has no filesystem, draft, save, or Desktop pin authority.
- [x] Package isolation, accessibility, large-document, and parity oracles are explicit.
- [x] GPUI CodeEditor and release remain separate decisions.

## Decisions

- CodeMirror 6 is an implementation detail shared by the Svelte and React wrappers.
- Pin direct `@codemirror/*` packages exactly in the lockfile. Do not use a
  framework wrapper or expose CodeMirror types.
- First languages: plain text, Markdown, JSON, YAML, TOML, JavaScript,
  TypeScript, HTML, CSS, Rust, and shell. Svelte syntax waits for an audited
  language package; it must not silently fall back to HTML.
- Public ranges are UTF-16 offsets into the previous value. `onChange` carries
  the complete exact next value and ordered replacement edits; it does not
  guess browser-event origin.
- Dedicated `./editor` entries isolate the engine from root consumers.
- CodeEditor is web-admitted. GPUI/native CodeEditor is future work.
- `Tabs.pinned` remains ordinary active-cohort work, not part of the exception.

## Dispatch manifest

- **State:** ready after separate operator go; serial ownership of web package
  manifests, lockfile, editor files, Tabs implementation, and generated docs
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** CodeEditor contract; Tabs contract; CodeMirror-facing
  core/styles; Svelte and React CodeEditor sources, tests, previews, package
  manifests and distribution entries; Tabs core/web/Rust/native sources and
  focused tests; `bun.lock`; spec 070; generated web documentation/evidence;
  one execution log and one Desktop adoption-request handoff
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, release records, Desktop repository
- **Worker:** high-reasoning TypeScript/component worker comfortable with
  CodeMirror lifecycle, SSR packaging, accessibility, and Rust parity for Tabs
- **Excluded:** release/tag/publish; Desktop edits or pins; raw CodeMirror API;
  arbitrary extensions; completion/LSP/hover/code actions/diff editor;
  GPUI or Jetstream CodeEditor; file, draft, save, review, or recovery policy
- **Escalation:** Chatterbox for any public API, language-package, web-only
  boundary, size-envelope, or package-entry change

## Work

1. Pin the minimal CodeMirror state, view, command, language, search, lint, and
   admitted language packages. Record licences. Avoid `basicSetup` if it pulls
   features outside the contract.
2. Build shared TypeScript types, extension assembly, theme mapping, controlled
   update/no-echo translation, diagnostic conversion, and cleanup.
3. Add thin Svelte and React wrappers with SSR-safe mount/update/destroy and
   dedicated `./editor` package entries.
4. Implement exact editing, language reconfiguration, search, line numbers,
   diagnostics, read-only/disabled, focus origin, Tab escape, internal scrolling,
   and explicit plain performance mode.
5. Implement `TabItem.pinned` in core, Svelte, React, shared Rust, render, and
   GPUI. Refuse invalid partitions and every cross-boundary reorder.
6. Add focused specimens/tests, packed-consumer proof, docs/export generation,
   execution log, and a Desktop adoption request naming the exact merge/release
   gate without changing Desktop.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Engine stays private | public prop/type imports `@codemirror/*` | declaration and packed-consumer scan fail |
| Root stays light | root package import includes editor or a language chunk | bundle graph proves no CodeMirror module from root-only consumer |
| Controlled text is exact | CRLF, astral Unicode, trailing spaces, or rejected host edit changes bytes or echoes | Svelte and React transaction/no-echo fixtures |
| Edits bind the previous value | multi-range edit reports offsets after an earlier replacement | replay edits against prior value equals callback value exactly |
| TypeScript is real | `.ts` fixture is treated as JavaScript/plain text | syntax tree/highlight fixture binds TypeScript mode |
| Unsupported language fails closed | `svelte` silently renders as HTML | closed type plus runtime refusal test |
| Diagnostics stay host-owned | wrapper invents validity or clamps bad ranges | exact valid mapping; invalid and duplicate diagnostics refused |
| Keyboard remains escapable | Tab indentation traps focus | default Tab exit and indent-mode Escape-Tab browser proof |
| Large files stay bounded | 2 MiB fixture expands page or mounts all lines | viewport/scroll-owner and mounted-node bound proof |
| Both web wrappers are one surface | one framework lacks search, diagnostics, or cleanup | paired cases plus exact prop/event drift checks |
| SSR/package lifecycle is safe | import touches DOM or worker before mount; destroy leaks listeners | server import, mount/remount/destroy, packed Vite consumer |
| Pinned tabs stay first | drag or Alt+Arrow moves/crosses a start-pinned tab | core, both web runtimes, Rust/render, and mounted GPUI Tabs tests |
| CodeEditor is not false native parity | placeholder/native receipt raises GPUI coverage | evidence labels web-admitted and census excludes it from portable rows |

## Stop conditions

- Stop if CodeMirror requires public engine types, a global worker/CSP escape,
  or a root eager import.
- Stop if a required language lacks an acceptable maintained/licensed package;
  return the exact language gap rather than adding an unaudited dependency.
- Stop if exact controlled updates cannot preserve selection without callback echo.
- Stop if the 2 MiB or accessibility oracle fails; do not weaken the contract.
- Stop before release, Desktop changes, or GPUI CodeEditor work.

## Evidence

Shipped in PR #236, merged as `308fa52c5cd68d9c776f320c368e4fb0896e4d4d` on 2026-09-10 after independent exact-head `ready_to_merge` review ([comment #5623576100](https://github.com/inflatable-cookie/poodle/pull/236#issuecomment-5623576100)) of head `6d875b30c7f36612ec7acec369038a97920c6ddc`. Execution log: `docs/logs/2026-09/20260910-g18-002-codemirror-web-code-editor.md`. Paired Svelte/React CodeEditor over 13 pinned `@codemirror/*` packages with dedicated `./editor` entries; `TabItem.pinned` across core, both web runtimes, shared Rust, render, and mounted GPUI. Validation-gate repairs in-round: content-aware ordinary JS manifest classifier (strict g16.059 and candidate g16.054 paths unchanged) and Nucleus M1/A1 receipt repin (62 receipt files plus manifest, `source_commit` only) with ledger regeneration. Web-admitted only; no GPUI CodeEditor, no Desktop change, no release.

## Next task

After merge and accepted review, return to Chatterbox for release/adoption
authority. GPUI CodeEditor remains a separately promoted future task.
