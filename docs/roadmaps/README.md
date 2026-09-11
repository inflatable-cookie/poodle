# Roadmaps

Status: reference
Updated: 2026-09-10

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
catalogue pages for CodeEditor, RichTextEditor, and RichTextRenderer (PR
in both web wrappers (PR #242). Operator-approved g18.015 merged the preview
distribution build preflight so both public preview selectors rebuild their package graph before Vite listens (PR #243). Operator-approved g18.016 merged the CodeEditor live line-number reconfiguration so the mounted gutter follows the host prop without remounting (PR #244). Operator-approved g18.013 merged the RichTextEditor toolbar controls so both web toolbars use grouped Poodle controls modelled on MarkdownEditor (PR #245).

Preflight is closed; `g18.006` is the ready `0.4.0` candidate and g18.009 is
the serial certification/publication step that unblocks Desktop. Final source
was rechecked and Tom granted explicit authority for both on 2026-09-11.

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
