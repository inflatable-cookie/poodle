# 053 Unwind The Three Pilot Slices

Status: drafted — R1 recorded (scene kept); R2 destination ruling required
before dispatch
Milestone: g13 closeout (executes the `g13.020` verdict)
Owner: Poodle core
Branch: `thread/g13-053-unwind-pilot-slices`
Depends on: `g13.020` signed
Governing refs:
`docs/roadmaps/g13/pilot-verdict-evidence.md` (§2 — the +965-line ledger),
`docs/roadmaps/g13/020-consolidate-and-reassess.md`,
`docs/contracts/001-working-rules.md` (§Runtime Parity Authority)

## Goal

Remove the Button, RangeSlider, and TextInput IR definitions and their
generated artifacts, and restore the nine consumer files to hand-written
states. The public surface of all three components stays byte-identical in
semantics: the slices shipped with "all props, all data attributes, every
pixel unchanged" (b041/b042, b045/b046, b048/b049), and the unwind must
preserve that guarantee in reverse.

The pre-pilot baseline is `0dd58b80`. Do **not** `git revert` — legitimate
post-slice changes (b047 machine shape, dismiss work, field fixes) landed
inside these files. Restore implementation form, not history.

## Fixed By Ruling (maintainer — required before dispatch)

### R1 — Scene system kept (recorded by the maintainer)

The preview shells (`preview_shell.rs` model → `preview-shell.ts` ×2,
`preview-shell.rs` ×2) stay generated. They are the one replacement case in
the whole pilot: four hand-written shell configs became one Rust source,
proven across all four runtimes (b035/b036). `poodle-ir` and
`poodle-codegen` survive slimmed to scene-only; this card deletes the three
component models, their targets, fixtures, and generated artifacts only.
Specimen migration onto the scene system is `g14.003`, not this card.

### R2 — `docs:capability-drift` declaration home

The gate (`packages/svelte/preview/scripts/capability-drift.ts`) reads its
declaration table from generated JSON produced by the codegen models. Before
the models are deleted, the declaration table must be rehomed into the gate
itself or `packages/contracts/headless/` — the gate is a g13.018 deliverable
the verdict keeps, and its source must not die with the generator.

### R3 — b052 thread branch

Delete the `thread/g13-052-*` branch. The five definitions are unmerged and
stay unmerged; their numbers are preserved in the `g13.020` reassessment.
No other disposition is available.

## Scope

### In scope

- The nine consumer files, restored to hand-written states with current
  semantics preserved:
  - `packages/svelte/components/src/{Button,RangeSlider,TextInput}.svelte`
  - `packages/react/components/src/{Button,RangeSlider,TextInput}.tsx`
  - `packages/render/src/{button,range_slider,text_input}.rs`
- Generated artifact removal:
  - `packages/{svelte,react}/components/src/generated/{button,range-slider,text-input}/`
  - `packages/render/src/generated/{button.rs,range-slider,text-input}/`
- Model and target removal: `packages/codegen/src/models/{button,range_slider,text_input}.rs`
  and the matching `targets/{button,button_rust,range_slider,range_slider_rust,text_input,text_input_rust}.rs`
  plus their registrations, fixture JSON, and tests (web generated-artifact
  tests included).
- R2 rehome regardless of anything else.
- The g13 closeout checklist in `../g14/README.md` updated as items close.

### Out of scope — stop conditions if reached

- `packages/contracts/headless/**` and `packages/contracts/headless/vectors/**`
  — machines and conformance vectors are g14's foundation, not IR.
- `docs:capability-drift` logic — rehomed, not rewritten.
- Any contract or public export change. If restoring a consumer would change
  behaviour or a public surface, **stop** and report the entanglement.
- The shell scene. It stays generated (R1).
- Component behaviour changes of any kind.

## Required Tests

- Svelte/React component tests pass unedited against the restored files —
  they pin the public surfaces (props, data attributes, events).
- Core tests, `ci:rust`, `ci:web`, `ci:native`, `docs:lint`, `git diff --check`.
- `ir:check` still green under R1-keep (shell-only artifacts unchanged).
- A grep gate: no `generated/` import remains in the nine files; no
  `poodle-ir`/`poodle-codegen` reference remains outside the R1-kept surface.
- LOC delta reported: expect ≈ −965 consumer lines plus models, targets,
  and fixtures removed.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- R1/R2 are maintainer rulings; if they are absent, do not dispatch.
- Restore current semantics, not pre-pilot semantics: for each file, diff
  against `0dd58b80`, keep every post-slice change that is not IR wiring,
  remove only the artifact consumption.
- Run the full component test suite for the three components before and
  after each restoration; a failing test is a finding, not a reason to
  weaken the test.
- Stage only writable paths by explicit path. Never `git add -A`.
- Commit and push with `git push -u origin thread/g13-053-unwind-pilot-slices`.
  Do not merge.
- `PAPERCUTS.md` and `tasks/effigy.tasks.toml` are shared: append only, do
  not reflow neighbours.

## Writable Paths

- `packages/svelte/components/src/{Button,RangeSlider,TextInput}.svelte`
- `packages/react/components/src/{Button,RangeSlider,TextInput}.tsx`
- `packages/render/src/{button,range_slider,text_input}.rs`
- `packages/{svelte,react}/components/src/generated/**`
- `packages/render/src/generated/**`
- `packages/codegen/**` (shell targets, `preview_shell.rs`, and shell
  fixtures excluded)
- `packages/svelte/preview/scripts/capability-drift.ts` (R2 rehome only)
- `packages/release-manifest.json`
- `tasks/effigy.tasks.toml`
- `docs/logs/2026-08/<DD>-g13-053-unwind-pilot-slices.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `effigy test:components`, `effigy ci:rust`,
   `effigy ci:web`, `effigy docs:lint`, `git diff --check`. All green.
2. R2 first: rehome the capability declaration table; prove
   `docs:capability-drift` still fails on a planted absence and passes
   clean.
3. Per component (Button, then RangeSlider, then TextInput): restore the
   three consumer files against `0dd58b80`, run the component tests, then
   delete the generated dir and the codegen model/targets/fixtures/tests.
4. Run `ir:build`; confirm the shell artifacts stay byte-identical —
   nothing in this card touches the shell scene.
5. Grep the repo: no stale `poodle-ir`, `poodle-codegen`, `ir:build`, or
   `generated/` reference outside the kept surface.
6. Record the LOC delta in the log against the verdict ledger.
7. Validate:
   ```sh
   effigy test:core
   effigy test:components
   effigy ci:rust
   effigy ci:web
   effigy ci:native
   effigy docs:lint
   effigy docs:capability-drift
   git diff --check
   ```

## Acceptance Criteria

- [ ] The nine files consume no generated artifact; public surfaces
  unchanged (component tests pass unedited).
- [ ] Generated dirs, models, targets, and fixtures for the three
  components are gone.
- [ ] R2 rehomed; the capability gate passes clean and fails on a plant.
- [ ] Shell artifacts byte-identical after `ir:build`; component targets
  gone, shell targets intact.
- [ ] LOC delta reported against the pilot ledger.
- [ ] All step-7 commands exit 0.

## Stop Conditions

- A consumer restoration entangles IR wiring with a legitimate post-slice
  change — report the exact file and lines; do not force a separation.
- Restoring a consumer would change behaviour or a public export.
- `ir:check` cannot be made green without touching the shell scene.
