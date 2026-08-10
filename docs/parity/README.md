# Historical Parity Audits

This directory preserves the manual component audit completed before the
g12.019 native-renderer consolidation. It is project history, not a current
implementation register.

The child files refer to native package tiers that no longer exist, including
`packages/gpui/components` and `packages/jetstream/components`. Their status
comments, todo counts, implementation paths, and runtime limitations describe
that earlier architecture. Do not use them to decide whether a current
component exists or is release-ready.

## Current Authority

Use these sources instead:

1. [Component contracts](../contracts/components/README.md) define the public
   inputs, behavior, accessibility, composition, and token use.
2. The Svelte and React component catalogues provide live web specimens and
   generated API documentation.
3. `packages/gpui/cross-runtime-parity-report.json` records current native
   cross-runtime results.
4. `packages/svelte/preview/scripts/parity-report.ts` generates the current
   parity report from repository evidence.
5. `effigy parity:check`, `effigy test:components`, and `effigy docs:check`
   enforce the release gates.

## Why These Files Remain

The audits contain useful point-in-time reasoning and explain why later
contracts and renderer work exist. Keeping them here preserves provenance
without pretending a manually maintained matrix can stay authoritative after
the implementation architecture changes.

New parity findings belong in contracts, generated reports, tests, or an active
roadmap. Do not add new component files to this directory.
