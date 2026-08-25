# g15.077 — Jetstream Poodle 0.2.2 adoption

Status: **complete — Jetstream PR 1 merged at `be969259`**
Depends on: `g15.061`, `g15.062`, completed `068`–`076` and `079`
Target repository: `/Users/tom/Dev/projects/jetstream`
Target base: `2106faec624973607c16a52985c9d1044e830261`
Governing refs: `055-consumer-adoption-inventory.md`,
`062-longhorn-poodle-v022-adoption.md`, Jetstream `AGENTS.md`, working rules,
active generation, and paired-runtime contracts

## Outcome

Move Jetstream's web editor from Poodle 0.1.x plus sibling web overrides to
public 0.2.2 while preserving its explicitly paired local Rust integration.

## Scope

- Pin the editor UI's two Poodle dependencies, core and Svelte, to exact
  registry `0.2.2`.
- Remove only the two Poodle web overrides; retain the three local Longhorn
  package dependencies and overrides.
- Keep `crates/jetstream-poodle` Rust Poodle paths local under the existing
  paired-runtime contract.
- Regenerate `editor-ui/bun.lock` narrowly; repair only Jetstream-owned
  compatibility fallout.
- Update Jetstream's `g06.014` evidence only as needed to record the completed
  Poodle npm half; do not claim the retained Longhorn `file:` lane is complete.

## Out Of Scope

- Do not convert paired Rust paths to a public distribution shape in this card.
- Do not edit Poodle/Longhorn, redesign the renderer, admit deferred parity
  work, or add compatibility aliases.
- Do not launch demos or visible runtime windows.

## Acceptance

- Editor UI resolves one public core/Svelte 0.2.2 identity.
- No active web 0.1.x or sibling Poodle web override remains.
- Paired Rust paths remain explicit and unchanged.
- Web/Rust boundary checks and broad headless QA pass or baseline is reproduced.

## Validation

- Run `bun install` in `editor-ui`; inspect the full Bun lock diff and prove the
  two Poodle packages resolve from the registry at 0.2.2 while the retained
  Longhorn packages converge on that Svelte peer.
- Prove `crates/jetstream-poodle/Cargo.toml` and the Rust lock are unchanged.
- Run `effigy editor:test`, `effigy check`,
  `effigy check:sibling-boundaries`, `effigy check:single-ui-stack`,
  `effigy test:cargo`, `effigy validate`, and `effigy qa`.
- Run `git diff --check`. Do not run demo/window selectors.

## Stop Conditions

- Adoption requires a paired-Rust distribution or renderer decision.
- Editor resolves duplicate/local web Poodle after override removal.
- Lock churn is unrelated or evidence needs a visible runtime.
- Product evidence requires native parity work outside this release rollout.

## Evidence And Continuation

Jetstream PR 1 merged reviewed head `3efd9dde` at `be969259`. Editor core and
Svelte now resolve exact public 0.2.2 with published integrity; the retained
local Longhorn adapter converges on Poodle Svelte 0.2.2. The paired Rust paths
and Rust lock stayed unchanged. Adapter compatibility added the new TabPanel
role, the deferred ProgressRing fallback, and current RenderContext-based smoke
probes.

Independent review reproduced the two pre-existing `jetstream-benchmark`
milestone-metadata failures only after docs, formatting, compilation, boundary
checks, demo registry, single-stack checks, Clippy, adapter tests, and 478 broad
tests passed. No visible runtime was launched. This closes the final
authoritative consumer lane; see
`../../logs/2026-08/20260825-g16-023-jetstream-v022-adoption.md`.
