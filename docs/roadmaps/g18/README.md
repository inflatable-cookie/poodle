# g18 — GPUI functional completion

Status: active
Opened: 2026-09-09
Updated: 2026-09-11
Governing refs: `../../../README.md`, `../../README.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/003-native-accessibility.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`../../evidence/nucleus/parity-evidence-ledger.md`,
`../generation-index.md`

## Generation outcome

Turn the 175-component portable GPUI surface from construction coverage into
contract-bound functional evidence, repair real gaps in bounded tranches, and
make it impossible to call a component complete from a specimen route or test
name alone.

This generation does not revive the rejected g14 conformance corpus. Component
contracts remain semantic authority; mounted runtime evidence proves only the
capabilities it actually drives.

## Current state

- 176 public components; 175 portable native targets; MeterSurface is the one
  contract-approved web-only row.
- 175/175 GPUI specimen routes construct headlessly.
- 29/175 components have validated mounted-behaviour receipts through the
  Nucleus cohort; 146 remain missing in the current ledger.
- Of those 146, 44 have retained named mounted-test expectations but no
  validated execution receipt. The other 102 have no admitted mounted receipt
  or retained expected-test entry.
- GPUI visual evidence covers the 29-row Nucleus cohort; 146 rows remain
  missing. Broad platform accessibility proof remains held separately.
- g18.001 census (PR #235): 73/175 portable rows carry at least one admitted mounted capability, 24/175 fully admitted; 224 refusals and 11 substrate groups await tranche compilation.

Construction is not functional completion. A bounded regression is not whole-
contract proof. The first task establishes the capability-level denominator
needed to compile honest repair tranches.

## Generation runway

| Task or planning horizon | State | Dependency or checkpoint |
| --- | --- | --- |
| [`g18.001`](001-contract-bound-gpui-functionality-census.md) — contract-bound GPUI functionality census | complete | PR #235 (merge `8185a9758f146e901499a8a8704a41202f99ca9e`) |
| [`g18.002`](002-codemirror-web-code-editor.md) — CodeMirror web CodeEditor | complete | PR #236 (merge `308fa52c5cd68d9c776f320c368e4fb0896e4d4d`) |
| [`g18.003`](003-tiptap-prosemirror-rich-text-editor.md) — TipTap/ProseMirror rich-text editor | complete | PR #237 (merge `fb0b73732b5c0a2a9361fddd75962eccd2710b0f`) |
| [`g18.004`](004-tabs-card-inactive-surfaces.md) — Tabs card inactive surfaces | complete | PR #239 (merge `ed6ed66050c5ba8bf62aaf27eee795ca5be052fa`) |
| [`g18.005`](005-v040-release-preflight.md) — v0.4.0 release preflight | complete | PR #238 (merge `93e165fd072aea27f44ede5a794042948654b265`) |
| [`g18.006`](006-v040-web-editor-release-and-desktop-unblock.md) — v0.4.0 web editor release candidate | paused | retained Queue task/workspace; resume after accepted g18.011 and merged g18.023/g18.024 |
| [`g18.007`](007-ordinary-changelog-maintenance-scope.md) — ordinary changelog maintenance scope | complete | PR #240 (merge `ef2e46bb949a766e844e48f071119c9576c6f723`); g18.005 validated and merged after |
| [`g18.008`](008-web-editor-preview-specimens.md) — web editor preview specimens | complete | PR #241 (merge `998b6ddc69f94e405b515f6bddd682a2e8916ea5`); Svelte/React catalogue admission via the web-only supplement, serial before g18.006 |
| [`g18.009`](009-v040-release-certification-and-desktop-unblock.md) — v0.4.0 release certification and Desktop unblock | dependency-queued behind g18.006 | final tag/publication/Desktop-unblock lane; no manual hold |
| [`g18.010`](010-code-editor-editing-focus-treatment.md) — CodeEditor editing focus treatment | complete | PR #242 (merge `a71b48573c7253dfd45f35e482b9bbc7432ea0ca`); product work continues through g18.012 before release resumes |
| [`g18.011`](011-web-editor-ux-acceptance-sweep.md) — web editor UX acceptance sweep | complete | PR #253 (merge `211ec0cb707eba62eadc5b33d1bb54d3605c5239`); four-surface paired Chromium/WebKit sweep, zero blocking findings, F12 follow-up retained; operator acceptance gates g18.006 |
| [`g18.012`](012-code-editor-extensible-language-registry.md) — CodeEditor extensible language registry | complete | PR #250 (merge `f011df5e0a97856bfe022569d77faf69a9f75553`); consumer-selected lazy CodeMirror language providers; last planned product change before the sweep |
| [`g18.013`](013-rich-text-editor-toolbar-controls.md) — RichTextEditor toolbar controls | complete | PR #245 (merge `1e11f59d01dc79f47196ee5f5b18e9e10cce5a70`) |
| [`g18.014`](014-rich-text-image-policy-specimen-proof.md) — rich-text image-policy specimen proof | complete | PR #249 (merge `72c7a9e5e6ce288780c7a2e3e44715949a035221`); self-contained offline seeded/insertion fixtures with visible host proof in both previews |
| [`g18.015`](015-preview-distribution-build-preflight.md) — preview distribution build preflight | complete | PR #243 (merge `0cf6073eb2067c4fc4127ec5c318a0f682c5859f`); accepted prerequisite evidence for the later sweep |
| [`g18.016`](016-code-editor-live-line-number-reconfiguration.md) — CodeEditor live line-number reconfiguration | complete | PR #244 (merge `5932bd0027cab2cf86c07a7878c3b48193252306`); accepted prerequisite evidence for the later sweep |
| [`g18.017`](017-block-slider-fixed-inline-presentation.md) — block Slider fixed inline presentation | complete | PR #246 (merge `f91be412b60739e96c29a45e9c17969c05b85f49`); rounded-square family corners and stable split-colour in-track Slider text across active runtimes |
| [`g18.018`](018-rich-text-controlled-echo-selection.md) — RichTextEditor controlled-echo selection preservation | complete | PR #248 (merge `08e377517af58a2033a145af2fe5c5875fb38215`); accepted controlled echoes preserve caret, selection, history and focus in both web wrappers |
| [`g18.019`](019-markdown-renderer.md) — MarkdownRenderer shared safe/trusted rendering | complete | PR #247 (merge `cc26dd3c09e12addab7c0b6c5a243f83f33cff94`); standalone paired renderer, shared editor-preview path, safe default and explicit trusted opt-in |
| [`g18.020`](020-rich-text-heading-mode-select.md) — RichTextEditor heading mode select | complete | PR #251 (merge `d8ad00b848acf62cc51357fc0caa6dbd559ff7c9`); consumer-configurable Normal/H1–H6 selector plus real H4–H6 schema/editor/renderer support |
| [`g18.021`](021-code-editor-token-bound-syntax-presentation.md) — CodeEditor token-bound syntax presentation | complete | PR #252 (merge `e69512038a4f032ecfad398562bbab28d40e9ffc`); private Poodle-token CodeMirror highlight style in both web engines |
| [`g18.022`](022-block-first-slider-family.md) — block-first Slider family | complete | PR #254 (merge `02ab7f7ec9122d85364beca77d05d681fa4d0124`); block-default two-variant Slider/RangeSlider API, fixed RangeSlider anchors, vertical block parity, repinned evidence |
| [`g18.023`](023-code-editor-dual-syntax-palettes.md) — CodeEditor dual syntax palettes | complete | PR #255 (merge `155dbc7d82fe04479a986c1f5f5698770e366c17`); designed dark/light syntax ramps plus sparse Poodle-theme overrides; ran beside g18.024; serial before g18.006 |
| [`g18.024`](024-slider-family-layout-and-vertical-repair.md) — Slider-family layout and vertical repair | complete | PR #256 (merge `c73db47d0de36dd0ce99ba697424dedb7c7b82da`); shared size alignment, layout-neutral hit targets, step-aware visible values, centred thumbs and xl-anchored vertical geometry; ran beside g18.023; serial before g18.006 |
| First functional repair tranche | planning horizon | compile from accepted g18.001 missing-capability output |
| Remaining mounted behaviour tranches | planning horizon | bounded by dependency and interaction substrate, not arbitrary component count |
| GPUI visual expansion | planning horizon | functional tranche stable; operator-approved background-safe capture |
| GPUI keyboard-origin focus | planning horizon | census identifies affected focus-bearing rows |
| A2 platform accessibility | held | `gpui-apple` publishes a buildable crate and live non-activating tree proof is available |
| Nucleus V2/M2/switch packet | external horizon | Nucleus-owned seeding and journeys plus Lab V2 and A2 |

`g18.003` and `g18.004` are complete after independent review and plugin-owned
merge. The `g18.005` preflight stayed bounded to the changelog/parser repair:
ordinary web CI rejected every changelog change by path, the operator-selected
g18.007 structural, content-aware maintenance rule merged as
`ef2e46bb949a766e844e48f071119c9576c6f723` (PR #240), and PR #238 merged as
`93e165fd072aea27f44ede5a794042948654b265` after exact-head re-review and
green checks. Release, Desktop adoption, native editor work, and GPUI repair
tranches do not auto-start.

The merged editor tasks had omitted their baseline preview catalogue pages.
Operator-confirmed g18.008 added separate CodeEditor, RichTextEditor, and
RichTextRenderer specimens to both web previews through the existing web-only
supplement (PR #241, merged `998b6ddc69f94e405b515f6bddd682a2e8916ea5`). It ran
independently of g18.007 and added no native parity.

The Svelte CodeEditor specimen exposed a focus-origin UX defect: ordinary
typing flipped the document modality to keyboard and the editor's live
`:focus-within` rule painted a persistent outer ring. Operator-approved g18.010
repaired both web wrappers with local navigation-entry state and
transaction-driven dismissal (PR #242, merged `a71b48573c7253dfd45f35e482b9bbc7432ea0ca`). Merged g18.012 replaced the closed grammar catalogue with
a consumer-selected registry (PR #250, merged `f011df5e0a97856bfe022569d77faf69a9f75553`). g18.011 then sweeps CodeEditor, RichTextEditor,
RichTextRenderer, and MarkdownRenderer across both previews after merged
g18.013 replaced the link-like rich-text toolbar. Merged g18.014 fixed the
inert image-policy example with a self-contained offline fixture and visible
host proof before the sweep begins. Merged g18.015 makes both public preview
selectors
rebuild their package graph before Vite listens, preventing stale ignored
distributions after merges. Merged g18.016 repaired the configuration
specimen's inert live line-number toggle without remounting the editor. Merged
g18.017 replaced pill block-slider corners and value-dependent external text
with stable in-track crossover treatment across web and GPUI.
Merged g18.018 repaired the controlled rich-text echo that called
`setContent` and moved the caret to the document end after every character.
With g18.013, g18.014 and g18.017 merged, g18.018 ran ahead of the sweep with g18.012 already merged. Merged g18.019 added
a standalone MarkdownRenderer through the existing `./markdown` entry, routed
MarkdownEditor preview through the same safe-by-default rendering path, and kept
raw output only behind explicit `htmlPolicy="trusted"`. Merged g18.020 replaced
the fixed H1–H3 buttons with one configurable Normal/H1–H6 Select and extended
schema/editor/renderer semantics through H6. The first g18.011 pass then found
a real release blocker: both CodeEditor engines load consumer-selected grammars
but install no CodeMirror highlight style, leaving full mode visually identical
to plain text. Merged g18.021 repaired it with one private Poodle-token
highlight style in both engines (PR #252). Merged g18.011 swept all four
surfaces under both engines with zero unresolved release-blocking findings
(PR #253); operator acceptance of the capsule is the remaining gate.
Merged g18.022 made block the default Slider-family variant, removed the old
standard/track surface, fixed RangeSlider text placement, and added vertical
block parity. Merged g18.023 replaced the rejected accent/status syntax mapping
with designed dark/light palettes (PR #255, merged `155dbc7d82fe04479a986c1f5f5698770e366c17`).
Merged g18.024 repaired the Slider family's size/alignment, numeric display,
and vertical geometry defects (PR #256, merged `c73db47d0de36dd0ce99ba697424dedb7c7b82da`).
g18.006 is the retained
final candidate lane and stays paused until operator acceptance of merged
g18.011 and the merged g18.024 repair. g18.009 waits on g18.006; no candidate
is tagged or published while accepted product source is moving.

## Held and recurring work

- Web-pair composite extraction remains an operator checkpoint when a shared
  composite is next touched or a second React consumer arrives.
- Contributor design-guidance pilot remains operator-gated on named reviewers,
  approvals, and run custody.
- Jetstream admission remains a separate held programme.
- GPUI and shared-Rust CodeEditor work is a future planning horizon. The web
  admission earns no native parity credit.
- GPUI and shared-Rust rich-text editing are future planning horizons. The
  `g18.003` web admission earns no native parity credit.
- Citations, nested menus, keyboard geometry, consumer requests, repository
  settings, and `gpui-unofficial` adoption gates remain in current triage.

## Completion rule

g18 completes only when every portable row has capability-level disposition:
validated mounted proof, a repaired and proved implementation, or a current
contract/platform hold with a named owner and recheck. Aggregate construction
counts and source presence cannot satisfy this rule.
