# Roadmap Generation Index

Updated: 2026-09-11

## Active generation

- [`g18`](g18/README.md) — GPUI functional completion
  - Status: active
  - Completed tasks: `g18.001` contract-bound GPUI functionality census; `g18.002` CodeMirror web CodeEditor (PR #236); `g18.003` TipTap/ProseMirror rich-text editor (PR #237); `g18.004` Tabs card inactive surfaces (PR #239); `g18.005` v0.4.0 release preflight (PR #238); `g18.007` ordinary changelog maintenance scope (PR #240); `g18.008` web editor preview specimens (PR #241); `g18.010` CodeEditor editing focus treatment (PR #242); `g18.015` preview distribution build preflight (PR #243); `g18.016` CodeEditor live line-number reconfiguration (PR #244)
  - Active task: `g18.013` RichTextEditor toolbar controls; `g18.017`
    block Slider fixed inline presentation is ready in parallel.
    `g18.018` RichTextEditor controlled-echo selection preservation is queued
    behind g18.013. `g18.011` stays held until g18.013, g18.014, g18.017 and
    g18.018 merge.
  - Queued task: `g18.011` three-component web editor UX acceptance sweep,
    dependency-ordered behind g18.010.
  - Queued task: `g18.012` CodeEditor extensible language registry,
    dependency-ordered behind g18.011.
  - Active task: `g18.013` RichTextEditor toolbar controls, required before
    g18.011 is released.
  - Queued task: `g18.014` rich-text image-policy specimen proof,
    dependency-ordered behind g18.013 and required before g18.011 is released.
  - Merged repair: `g18.015` preview distribution build preflight (PR
    #243); both public preview selectors rebuild their package graph
    before Vite listens.
  - Merged repair: `g18.016` CodeEditor live line-number reconfiguration (PR
    #244); the mounted gutter now follows the host prop without remounting.
  - Ready task: `g18.017` block Slider fixed inline presentation, parallel and
    required before g18.011 is released.
  - Queued task: `g18.018` RichTextEditor controlled-echo selection
    preservation, dependency-ordered behind g18.013 and required before
    g18.011 is released.
  - Merged specimen admission: `g18.008` web editor preview specimens;
  - Merged repair: `g18.007` ordinary changelog maintenance scope;
    unblocked retained g18.005 PR #238 without a CI exception, and g18.005
    merged after exact-head re-review.
  - Merged repair: `g18.010` CodeEditor editing focus treatment;
    the outer ring is now a local keyboard-entry affordance dismissed on
    committed edits, with the document modality left truthful.
  - Paused candidate: retained g18.006 task/workspace resumes only after
    g18.010–g18.018, all other blocking repairs, and operator sweep acceptance.
  - Held release: queued g18.009 remains behind g18.006 but is explicitly held;
    no tag or publication while candidate source is moving.
  - Next checkpoint: merge g18.013, complete g18.014, g18.017 and g18.018,
    release and complete g18.011, complete g18.012, repair other blocking
    findings, and get operator acceptance before resuming g18.006.

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
