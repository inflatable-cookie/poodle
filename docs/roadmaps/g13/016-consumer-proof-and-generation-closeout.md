# g13.016 Consumer Proof And Generation Closeout

Status: closed — superseded by the `g13.008` **revise** verdict
(`docs/roadmaps/g13/pilot-verdict-evidence.md` §7–8). This milestone describes
family-by-family migration to a generative model the verdict declines. It is
retained as evidence and is **not executable**. The replacement runway is
`g13.017`–`g13.020`.
Owner: Poodle core
Depends on: `g13.010`–`g13.015`

## Objective

Prove the Rust-authored core through packed/source consumers, remove retired
surfaces, promote final architecture, and close g13 without residual planning
drift.

## Deliverables

- Svelte and React packed/source consumer installs over generated artifacts.
- GPUI and Jetstream downstream builds over the shared Rust crates.
- Full interaction, accessibility, recipe, visual, axis, docs, drift, and
  generated-artifact gates.
- Removal or explicit deferral of old definitions, compatibility layers, and
  one-off registry generators.
- Final architecture/working-rule promotion, execution log, generation index,
  and next-program decision.

## Acceptance

- One authored definition and one specimen scene source exist per migrated
  component, plus only declared runtime capabilities/extensions.
- All four consumer paths pass from a clean checkout.
- g13 has no live card or provisional spec left behind at closeout.

## Next

Close g13. Open another generation only for a materially new sequencing
baseline.
