# Historical Parity Audits — Archived

The manual component audits that lived in this directory were archived
2026-09-05 (g16.108 docs spine compaction): they now sit under
[`docs/archive/parity/`](../archive/parity/README.md), where the full
historical explainer and the per-component files remain.

This pointer exists so historical references and bookmarks keep resolving.
The audits are project history, not a current implementation register, and
they were never an edit target after the g12.019 native-renderer
consolidation. Do not add new component files here.

## Current Authority

Use these sources instead:

1. [Component contracts](../contracts/components/README.md) define the public
   inputs, behavior, accessibility, composition, and token use.
2. The g16.001 [active-cohort evidence ledger](../roadmaps/g16/parity-evidence-ledger.md)
   is the current component-level denominator and evidence authority: 175
   public Svelte components, 174 portable native routes, and one web-only
   MeterSurface exclusion.
3. The Svelte and React component catalogues provide live web specimens and
   generated API documentation; Svelte axe evidence does not transfer to
   React.
4. `packages/gpui/cross-runtime-parity-report.json` records the current GPUI
   construction, bounded mounted, manual accessibility, and Button-only visual
   posture.
5. `packages/svelte/preview/scripts/parity-report.ts` generates the current
   parity report from repository evidence.
6. `effigy check:parity-evidence-ledger`, `effigy parity:check`,
   `effigy test:components`, and `effigy docs:check` enforce the release gates.
