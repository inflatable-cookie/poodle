# Nucleus parity execution receipts

This directory holds reviewed execution receipts emitted by the real headless
GPUI selector. A receipt is evidence only when it passes the schema and
manifest validator in `scripts/nucleus-parity-receipts.ts`.

Receipts are named after their fixed component scenario. They contain no
timestamps or machine paths. A receipt records the exact Poodle source commit
and Cargo.lock resolution used by the mounted run; changing the runtime source
requires a new run and a new receipt. M1 receipts may keep `artifact_paths`
empty; every nonempty entry has the exact repository-relative `path` and its
lowercase SHA-256 `sha256`, and the validator checks the file and digest.

A1 receipts (g16.111) are named `<component>--<scenario>--a1.json` and pair
the mounted GPUI node-tree projection with the mounted Svelte DOM for one
shared scenario. An executed A1 run whose snapshots disagree emits no receipt;
its snapshots and diff are recorded under `a1-divergences/` (ignored by the
validator) until the owning tranche repairs the component.
