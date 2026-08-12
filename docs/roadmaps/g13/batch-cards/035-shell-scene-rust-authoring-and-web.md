# 035 Shared Preview Shell Scene — Rust Authoring And The Two Web Shells

Status: ready
Milestone: `g13.004` (part 1 of 2 — **this card does not close `g13.004`**)
Owner: Poodle core
Branch: `thread/g13-035-shell-scene-rust-authoring-and-web`
Depends on: `g13-b011` (`4a22c8d8`), `g13-b012` (`911fdfd8`), `g13-b022`
(`143c63a1`), `g13-b025` (`5d9edc9d`) — all merged
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
("Authoring Form", "Generated Artifact Contract"),
`docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md`,
`docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`
(`R1`, `R2`), `packages/contracts/ir/src/scenes.rs`

## Goal

`g13.004` defines the preview shell **once** and renders it in all four
runtimes. This card does the part everything else depends on: author the shell
scene in Rust, and make the two web shells consume it.

The native shells follow in `036`. `g13.004` closes when `036` merges, not
this card.

## Why This Is Split

`g13.003` shipped as `b022` (emitter core) then `b025` (remaining emitters) for
the same reason. The risk here is concentrated in the authoring form and the
scene shape; once those are settled and two shells consume them, the native
shells are a mechanical second pass. Svelte is parity authority
(`docs/contracts/001-working-rules.md`), so the web shells settle the shape.

## Current State — Measured, Not Assumed

Scene IR already models every shell capability this card needs.
`packages/contracts/ir/src/scenes.rs` (291 lines) declares `Scene`,
`SceneAxis`, `SceneLayout`, `NavSection`, `SpecimenTabs`, `SearchConfig`,
`PreviewState`, `ParityHarness`, `SpecimenRegistry` — the `SHELL-01`–`SHELL-10`
rows. **`b011` built the vocabulary and nothing has authored a value into it
yet:** `packages/codegen/fixtures/synthetic-model.json` has
`"scenes": []` and `"specimen_registry": null`.

Each shell composes its own control surface today:

| Shell | Control surface | Lines |
|---|---|---|
| Svelte | `preview/src/components/DisplayControls.svelte` | 134 |
| React | `preview/src/gallery/DisplayControls.tsx` | 83 |
| GPUI | inline in `preview/src/main.rs` | 2207 (whole file) |
| Jetstream | `preview/src/shell.rs` | 699 |

All four implement the axes. The labels have already drifted: the web shells
render `Theme` / `Search`, GPUI renders `THEME` / `SEARCH`. GPUI's contrast is
a continuous range (`app_state.rs:246`, `CONTRAST_MIN`/`CONTRAST_MAX`), which
matches the web's range input — so this is a labelling and ownership problem,
not a missing-capability problem.

## Fixed By Ruling (do not re-decide)

### R1 — The scene is authored in Rust. JSON is the serialized form, not the source.

Spec 063 "Authoring Form": *"Start with ordinary Rust types and constructor
helpers."* Authoring the shell scene as hand-written JSON would pass every gate
and prove nothing about the thesis the pilot exists to test.

Add a Rust module that constructs the shell `Scene` using `poodle-ir` types,
and a path that serializes it to the JSON the existing pipeline already
consumes, so `ir:build` / `ir:check` keep working unchanged.

**Placement.** `packages/codegen/src/models/preview_shell.rs`, reachable from
the existing bin. Not `poodle-ir`: `b003 R1` fixed that crate as **lib only,
no `[[bin]]`**, pure serializable data plus validation. An authored *instance*
is content, not schema. Do not create a new crate — `b003 R1` already paid the
one-new-directory cost for `packages/codegen/`.

This placement is **pilot-scoped**. Where production models are authored is a
`g13.008` question. Say so in the module's header comment so the next reader
does not mistake it for a settled boundary.

Do not add macros. Spec 063: macros come only *"where the pilot proves they
materially improve authoring"*, and the pilot has not run yet.

### R2 — Author values. Do not extend the schema.

Every capability this card needs already has a field. If you find one that does
not, **stop** — schema changes belong to `g13.002`, and inventing a field here
would put the pilot's verdict on a schema nobody reviewed.

Keep `synthetic-model.json` as the emitter's own fixture. The shell model is a
second, separate model — do not overwrite the fixture that `b022`/`b025`'s
tests pin.

### R3 — Runtime hosts hold capability glue, not shell composition.

From `g13.004`'s acceptance. A host owns local state, routing, focus, and
native controls. It does not own what the shell *is*. After this card, deleting
a control from the scene must remove it from both web shells; if a shell still
renders it, the shell is still composing.

The standing boundary is unchanged: runtime adapters keep focus, IME, portals,
measurement, pointer capture, lifecycle, input, hit-testing and accessibility.

### R4 — Labels come from the scene.

`Theme` / `THEME` is the drift this card removes. One scene supplies the label
text; a shell may style it (uppercase via CSS or the native equivalent) but
must not author different text. Casing is presentation. Content is the scene's.

### R5 — No component migration.

`g13.005` is the first component vertical slice, and broad migration is locked
until the maintainer records **adopt** at `g13.008`. This card touches the
shell only. Do not migrate a component, and do not "just also" convert a
specimen.

## Scope

### In scope

- `packages/codegen/src/models/preview_shell.rs` — the Rust-authored shell
  scene, plus whatever `mod` wiring and bin path serializes it.
- The two web shells consuming it: Svelte and React.
- Generated output landing under a `generated/` directory inside the consuming
  web packages (`b003 R1`, "Generated output location").
- Tests and fixtures for the above.

### Out of scope — stop conditions if reached

- The GPUI and Jetstream shells (`036`).
- Any component definition or specimen.
- `HistoryCenter` and its specimens — **`g13-b034` is in flight on those
  files**. Do not touch them.
- Any Longhorn or Loophole file.
- Refreshing visual baselines.
- Schema changes to `poodle-ir` (R2).

## Required Tests

- `effigy ir:build` and `effigy ir:check` both exit 0 with the shell model
  present, and `ir:check` still fails on drift (prove it: mutate a generated
  file, watch it fail, restore).
- The generated artifact carries the source-definition name and generator
  version in its header — spec 063's "Generated Artifact Contract".
- A test asserting the two web shells expose the **same** capability set and
  the **same** label text, derived from the scene rather than a hand-written
  list. A hand-listed expectation would pass while the shells drift, which is
  the failure this card exists to prevent.
- Changing one value in the Rust scene updates both web shells in one
  `ir:build` — spec 063's "One Rust definition change must update every
  expected target in one build."
- Theme, size, density, contrast and tab changes remain interactive in both
  web previews. Do not lose behaviour to gain sharing.

Do not weaken an existing test to make one of these pass.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `packages/contracts/ir/src/scenes.rs` in full before authoring. Its doc
  comments name the `SHELL-*` row each field serves — use them.
- **Run `effigy check:svelte`.** Not optional.
- `docs:callback-drift` is a new gate — run it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-035-shell-scene-rust-authoring-and-web`. Do
  not merge.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/fixtures/**` (new files only — do not edit
  `synthetic-model.json`)
- `packages/codegen/tests/**`
- `packages/svelte/preview/src/App.svelte`
- `packages/svelte/preview/src/components/DisplayControls.svelte`
- `packages/svelte/preview/src/router.ts`
- `packages/react/preview/src/gallery/App.tsx`
- `packages/react/preview/src/gallery/DisplayControls.tsx`
- `packages/{svelte,react}/preview/src/**/generated/**`
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md` (status line only)
- `docs/logs/2026-08/<DD>-g13-035-shell-scene-rust-authoring-and-web.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ci:rust`, `ir:build`, `ir:check`, `test:components`,
   `check:svelte`, `docs:lint`, `git diff --check`. All start green.
2. Read `scenes.rs` in full. Write the capability + label matrix for the two
   web shells into your log — the measured before-state this card's parity test
   will replace.
3. Author the shell scene in Rust (R1). Serialize it; confirm it validates.
4. Emit, and wire the Svelte shell to consume the generated artifact.
5. Mirror React exactly.
6. Prove R3: remove a control from the scene, rebuild, confirm it disappears
   from both shells, restore it. Record the result in your log.
7. Validate:
   ```sh
   effigy ir:build
   effigy ir:check
   effigy ci:rust
   effigy test:core
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:callback-drift
   effigy svelte:surface-audit
   git diff --check
   ```

## Acceptance Criteria

- [ ] The shell scene is authored in Rust, not hand-written JSON.
- [ ] `poodle-ir` gained no `[[bin]]` and no new schema field.
- [ ] Both web shells render their controls and labels from the scene; the
  step-6 removal test proves it.
- [ ] `ir:check` exits 0 clean and non-zero on drift.
- [ ] Both web previews keep every axis interactive.
- [ ] All step-7 commands exit 0; no baseline refreshed.
- [ ] `synthetic-model.json` is unchanged.

## Stop Conditions

- A shell capability has no field in Scene IR (R2 — schema work is not yours).
- Serializing the Rust-authored model cannot produce input the existing
  `load_and_validate` accepts without changing the loader's contract.
- Sharing a control costs interactivity in either web shell.
- The parity test can only be written against a hand-listed expectation.

Stop with exact paths, commands, and the smallest unresolved question.
