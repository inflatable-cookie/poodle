# Roadmaps

Updated: 2026-09-14

Northstar roadmaps use one generation-plus-task model:

- the active generation README owns the roadmap and approved frontier;
- `gNN/NNN-<slug>.md` is the sole executable planning unit;
- that unit is a **Northstar task**, referenced as `gNN.NNN`;
- no milestone wrapper, nested `batch-cards/`, or dual task authority is supported.

## Current state

- [`generation-index.md`](generation-index.md) names the active generation.
- [`g18/README.md`](g18/README.md) is the current roadmap and approved frontier.
- [`dispatch.md`](dispatch.md) is only the queue transport projection.
- [`archive/`](archive/) contains compact non-procedural roll-ups for g01–g17.
- [`../evidence/nucleus/`](../evidence/nucleus/) holds the current Nucleus
  cohort, M1/A1 receipts, schemas, and generated ledger.

`g18.002`, `g18.003`, and `g18.004` are complete. They delivered the paired
web CodeEditor, paired rich-text editor/renderer, and Tabs card fill. GPUI
editor parity remains future work and the g18 GPUI repair runway stays open.
The bounded g18.005 release preflight merged (PR #238) after the
operator-approved g18.007 structural unblock lane (PR #240); no CI exception
was authorized. Operator-approved g18.008 merged the baseline Svelte/React
catalogue pages for CodeEditor, RichTextEditor, and RichTextRenderer (PR #241).
g18.010 fixed CodeEditor editing focus (PR #242); g18.015 fixed preview
distribution startup (PR #243); g18.016 fixed live line-number configuration
(PR #244); g18.013 introduced grouped RichTextEditor controls (PR #245);
g18.017 fixed block Slider presentation (PR #246); g18.019 added the paired
MarkdownRenderer (PR #247); g18.014 proved the RichTextEditor image-policy
specimens with a self-contained offline fixture (PR #249); g18.012 replaced
the closed CodeEditor grammar catalogue with a consumer-selected lazy language
registry (PR #250); and g18.018 made accepted RichTextEditor controlled echoes
preserve caret, selection, history and focus in both web wrappers (PR #248).

Merged g18.020 replaced fixed H1–H3 controls with one consumer-configurable
Normal/H1–H6 selector backed by real H4–H6 document support (PR #251).
Merged g18.021 repaired the g18.011 F1 blocker with one private Poodle-token
highlight style in both CodeEditor engines (PR #252). Merged g18.011 swept
all four editor surfaces across both web previews under paired Chromium and
WebKit engines (PR #253): zero unresolved release-blocking findings, one
retained non-blocking follow-up (F12, preview-harness owner); operator
Slider family (PR #254); merged g18.024 repaired its shared-size
alignment, layout-neutral targets, numeric display, and vertical geometry
(PR #256).
Merged g18.023 replaced the rejected accent/status syntax mapping with
designed dark/light palettes (PR #255). Operator-approved g18.025 fixes all
preview header controls at one selected size with `sm` as the default. Merged
g18.029 admits the closed `0.4.0` candidate surface. Merged g18.030 removes
the receipt emitter's hard-coded lock provenance and repins the complete
current cohort. Merged g18.031 (PR #264) aligns root release metadata, keeps
it in the exact lockstep candidate surface and derives GPUI census receipt
versions before the retained g18.006 task resumes. Merged g18.006 (PR #265)
prepared the accepted `0.4.0` candidate, and g18.009 released it on 2026-09-13
through an operator-authorized npm/web wrapper repair: `v0.4.0` is tagged at
`4a39055f3`, core and Svelte `0.4.0` are published with attested provenance,
and a fresh registry consumer proves the Svelte `./editor` entry. Desktop
unblocks from that published release.

Merged g18.033 on 2026-09-13 (PR #268) repairs the `form-dialog` first-draw
non-termination: the probe now closes each route window after its assertions,
restoring all 175 specimen routes with a fail-closed regression, and one
complete `qa:board` is wholly green in 453s. Three baseline reds the board
reached were cleared under operator rulings (`deny.toml` bzip2 drift, the
tracked-symlink audit repair, the Vitest advisory patch). The post-`0.4.0`
consumer/specimen sweep is the current execution frontier. Its first bounded
Poodle-owned finding is
[`g18.036`](g18/036-slider-role-drag-boundary.md): keep `role="slider"`
gestures out of ancestor drag sources without editing Loophole's retained
consumer workaround or starting a release.

## Rules

- Keep task work bounded by current architecture, contracts, and accepted intent.
- Each ready task carries its own scope, ordered work, dependencies, dispatch
  boundary, acceptance oracle, validation, evidence, ownership, and stop gates.
- Use [`templates/task-template.md`](templates/task-template.md) for new tasks.
- A queue task is a control-plane record, not planning authority. An Effigy task
  is a command selector, not a Northstar task.
- Unresolved or deferred candidates live in [`../triage/`](../triage/) until
  promoted; triage notes carry no execution authority. Architecture lives in
  `../architecture/`.
- Generation rollover is manual. Close, pause, supersede, or rehome every live
  task before opening the next generation.
- Completed generations compact to `archive/gNN.md`; git preserves detail.

## Reading order

Start with `generation-index.md`, then the active generation README, then the
ready task. Historical work should not be needed to execute current work.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:f73968278565e1345fa8bcfdfe1450965596bb692c5c253de1bfab565616aa0e -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g18 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g18.034 | complete | none | 8 | sha256:bd6863d116045dec331e616c30985260c441d1ea02735bcfb4bdbdb275f1c106 |
| g18.035 | complete | none | 8 | sha256:daefcba434d2d1941bb670de98b3faf47d988cb82732c6f2f6a848e4f38614e3 |
| g18.036 | complete | none | 8 | sha256:b4f79112ff283c2cf7b3c32214cdeeaab547f0766609dde3d2a9a4f67a5649d3 |
<!-- northstar:lifecycle:end -->
