# Roadmap Generation Index

Updated: 2026-09-12

## Active generation

- [`g18`](g18/README.md) — GPUI functional completion
  - Status: active
  - Completed tasks: `g18.001` contract-bound GPUI functionality census; `g18.002` CodeMirror web CodeEditor (PR #236); `g18.003` TipTap/ProseMirror rich-text editor (PR #237); `g18.004` Tabs card inactive surfaces (PR #239); `g18.005` v0.4.0 release preflight (PR #238); `g18.007` ordinary changelog maintenance scope (PR #240); `g18.008` web editor preview specimens (PR #241); `g18.010` CodeEditor editing focus treatment (PR #242); `g18.013` RichTextEditor toolbar controls (PR #245); `g18.015` preview distribution build preflight (PR #243); `g18.016` CodeEditor live line-number reconfiguration (PR #244); `g18.017` block Slider fixed inline presentation (PR #246); `g18.019` MarkdownRenderer shared safe/trusted rendering (PR #247); `g18.014` rich-text image-policy specimen proof (PR #249); `g18.012` CodeEditor extensible language registry (PR #250); `g18.018` RichTextEditor controlled-echo selection preservation (PR #248); `g18.020` RichTextEditor heading mode select (PR #251); `g18.021` CodeEditor token-bound syntax presentation (PR #252); `g18.011` four-surface web editor UX acceptance sweep (PR #253)
  - Merged task: `g18.011` four-surface web editor UX acceptance sweep (PR
    #253); paired Chromium/WebKit evidence with zero unresolved
    release-blocking findings and one retained follow-up (F12); operator
    acceptance gates retained g18.006.
  - Merged task: `g18.022` block-first Slider family (PR #254); block is now
    the default Slider/RangeSlider variant with fixed anchors and vertical
    parity after exact-head review.
  - Ready tasks: operator-approved `g18.023` CodeEditor dual syntax palettes
    and `g18.024` Slider-family layout and vertical repair may dispatch in
    parallel; both must merge before g18.006 resumes.
  - Merged repair: `g18.021` token-bound CodeEditor syntax presentation (PR
    #252); both engines now present Poodle-token syntax in full mode and the
    repair is an in-place g18.011 dependency.
  - Merged task: `g18.012` CodeEditor extensible language registry (PR #250);
    both web wrappers now take a consumer-owned lazy language registry with
    `plain-text` built in and fail-closed unknown/rejected loads.
  - Merged repair: `g18.013` RichTextEditor toolbar controls (PR #245);
    both web toolbars now use grouped Poodle controls modelled on
    MarkdownEditor.
  - Merged task: `g18.014` rich-text image-policy specimen proof (PR #249);
    both previews now seed a self-contained offline fixture with visible host
    feedback and insert-exactly-once proof.
  - Merged repair: `g18.015` preview distribution build preflight (PR
    #243); both public preview selectors rebuild their package graph
    before Vite listens.
  - Merged repair: `g18.016` CodeEditor live line-number reconfiguration (PR
    #244); the mounted gutter now follows the host prop without remounting.
  - Merged repair: `g18.017` block Slider fixed inline presentation (PR
    #246); the block Slider family now uses rounded-square corners and
    single-Slider text stays fixed inside the track with split-colour
    crossover across active runtimes.
  - Merged task: `g18.018` RichTextEditor controlled-echo selection
    preservation (PR #248); accepted controlled echoes now preserve caret,
    selection, history and focus in both web wrappers.
  - Merged task: `g18.019` MarkdownRenderer shared safe/trusted rendering
    (PR #247); both frameworks now share one safe-by-default content path
    between MarkdownEditor preview and the standalone renderer, with raw
    output only behind explicit `htmlPolicy="trusted"`.
  - Merged task: `g18.020` RichTextEditor heading mode select (PR #251);
    both wrappers now project consumer-configured heading levels as one
    Normal/H1–H6 selector over real H4–H6 schema, editor and renderer support.
  - Merged specimen admission: `g18.008` web editor preview specimens;
  - Merged repair: `g18.007` ordinary changelog maintenance scope;
    unblocked retained g18.005 PR #238 without a CI exception, and g18.005
    merged after exact-head re-review.
  - Merged repair: `g18.010` CodeEditor editing focus treatment;
    the outer ring is now a local keyboard-entry affordance dismissed on
    committed edits, with the document modality left truthful.
  - Paused candidate: retained g18.006 task/workspace resumes only after
    accepted g18.011 and merged g18.023/g18.024.
  - Final release: g18.009 is dependency-queued behind g18.006 with no manual
  - Next checkpoint: dispatch g18.023 and g18.024 in parallel; merge both,
    accept the repaired specimens, then resume g18.006.
## Compacted generations

| Generation | Durable result | Roll-up |
| --- | --- | --- |
| g01 | foundations, tokens, contracts, first shared surface | [`g01`](archive/g01.md) |
| g02 | composite depth, docs, package/release baseline | [`g02`](archive/g02.md) |
| g03 | ecosystem hardening and adoption | [`g03`](archive/g03.md) |
| g04 | Underlay parity and specialist components | [`g04`](archive/g04.md) |
| g05 | GPUI foundation and parity baseline | [`g05`](archive/g05.md) |
| g06 | shared multi-renderer contracts | [`g06`](archive/g06.md) |
| g07 | GPUI adapter build-out | [`g07`](archive/g07.md) |
| g08 | GPUI production quality | [`g08`](archive/g08.md) |
| g09 | native package consolidation | [`g09`](archive/g09.md) |
| g10 | Jetstream feasibility and GPUI hardening | [`g10`](archive/g10.md) |
| g11 | Svelte modernization and shared web machinery | [`g11`](archive/g11.md) |
| g12 | React parity and native verification depth | [`g12`](archive/g12.md) |
| g13 | rejected Rust-authored IR pilot | [`g13`](archive/g13.md) |
| g14 | rejected executable-conformance pilot | [`g14`](archive/g14.md) |
| g15 | v0.2.x release and adoption programme | [`g15`](archive/g15.md) |
| g16 | v0.3.0, consumer adoption, Nucleus M1/A1 | [`g16`](archive/g16.md) |
| g17 | Nucleus M1/A1/V1 switch evidence and background-safe capture | [`g17`](archive/g17.md) |

Only g18 remains expanded. Roll-ups are historical summaries, never executable
task surfaces.
