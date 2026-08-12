# 036 Shared Preview Shell Scene — GPUI And Jetstream

Status: ready
Milestone: `g13.004` (part 2 of 2 — **this card closes `g13.004`**)
Owner: Poodle core
Branch: `thread/g13-036-shell-scene-native-shells`
Depends on: `g13-b035` (`db4e4510`), merged
Governing refs: `docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md`,
`docs/roadmaps/g13/batch-cards/035-shell-scene-rust-authoring-and-web.md`,
`docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`
(**`R2` is the constraint that shapes this card**),
`docs/specs/063-rust-authored-component-and-scene-ir.md`

## Goal

`b035` authored the shell scene in Rust and put both web shells on it. This
card does the same for GPUI and Jetstream, and `g13.004` closes when it merges.

## Current State — Measured, Not Assumed

The scene authors five controls: `Theme`, `Size`, `Density`, `Contrast`,
`Search`.

| Shell | Matches the scene? |
|---|---|
| Svelte | yes — reads the generated artifact (`b035`) |
| React | yes — reads the generated artifact (`b035`) |
| Jetstream | labels all five already agree, but they are hand-written in `preview/src/shell.rs` (699 lines) |
| GPUI | three deltas, hand-written in `preview/src/main.rs`: `THEME` and `SEARCH` are upper-case, and contrast reads **`Neutral contrast`** |

GPUI's contrast is a continuous range (`app_state.rs:246`,
`CONTRAST_MIN`/`CONTRAST_MAX`) matching the web's, so every capability exists
in both natives. This is an ownership and wording problem, not a missing
control.

## Fixed By Ruling (do not re-decide)

### R1 — A generated Rust artifact. Not a crate dependency.

`b003 R2` freezes the dependency direction and forbids exactly the shortcut
this card invites:

> `poodle-specs`, `poodle-headless`, `poodle-render`, both adapters, and both
> previews must not depend on `poodle-ir` **during the pilot** … That inversion
> is a `g13.008`-gated decision.
>
> `poodle-codegen` must not be depended on by anything. It is a tool.

So the natives may not import `poodle_ir`, may not import the authoring
module, and may not deserialize the fixture through `poodle-ir` types.
**Do not add either crate to `packages/gpui/preview/Cargo.toml` or
`packages/jetstream/preview/Cargo.toml`.** If that looks like the only way
through, stop — you have found a real finding for `g13.008`, not a licence.

Emit a **self-contained** Rust artifact instead: plain data, no `use` of any
Poodle crate, the way the TypeScript artifact carries zero imports. Precedent
for the mechanism is `poodle-tokens`, which pulls generated Rust in with
`#[path = "../../../tokens/artifacts/rust/mod.rs"]`; `b003 R1` names it as the
shape g13 follows.

### R2 — A new emitter target. Do not repurpose the `.ts` one.

`targets/shell.rs` hard-codes `format!("{}.ts", …)`, and `b035`'s tests
byte-compare both committed web artifacts against its render. Changing that
target's output breaks the parity test that makes `b035` mean anything. Add a
sibling target; leave `shell-scene` alone.

Both targets read the same scene. One authored change must still move every
shell in one build — spec 063's "Generated Artifact Contract" — and now that
is four artifacts, not two.

### R3 — Labels come from the scene.

The three GPUI deltas go. Casing is presentation: GPUI may render `Theme`
upper-case through styling if that is its house look. **`Neutral contrast` is
not casing** — it is different text, and after this card the scene owns the
word. Change the scene if `Neutral contrast` is the better label; do not keep
a second copy of it in a shell.

### R4 — Hosts hold capability glue, not shell composition.

`b035` proved this by deleting a control from the scene and watching both web
shells drop it. Repeat that test with all four shells, and record the result.
A native that still renders a removed control is still composing.

The standing boundary is unchanged: GPUI and Jetstream keep node
interpretation, runtime input, lifecycle, text, accessibility projection and
engine drawing.

### R5 — Look at it. Do not infer.

Both natives are pixel-verifiable locally — GPUI takes `--screenshot`,
Jetstream has a headless `snap`. "It compiles" is not evidence that a control
renders. Capture both shells and say what you saw.

### R6 — No component migration.

`g13.005` is the first component vertical slice, and broad migration stays
locked until the maintainer records **adopt** at `g13.008`. Shell only.

## Scope

### In scope

- The new Rust emitter target in `packages/codegen/`.
- Generated Rust artifacts for both native previews.
- Both native shells consuming them.
- `ir:build` / `ir:check` extended to cover the two new artifacts.
- Tests and fixtures for the above.
- `docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md` — status to
  complete, since this card closes it.

### Out of scope — stop conditions if reached

- Adding `poodle-ir` or `poodle-codegen` to any preview manifest (R1).
- Editing `targets/shell.rs`'s output or `synthetic-model.json` (R2).
- Any component definition or specimen.
- `HistoryCenter` and its files.
- The two papercuts `b035` logged (Svelte `SHELL-08` URL persistence; the
  catalogue-landing grids ignoring their filtered prop). Real, but not this
  card.
- Refreshing visual baselines. If a native baseline moves because the shell
  now renders the scene's labels, **say so and stop** — classify the delta
  first; a baseline can be wrong rather than merely outdated.

## Required Tests

- `effigy ir:build` and `ir:check` exit 0 with four artifacts, and `ir:check`
  still fails on drift in a **Rust** artifact (prove it: plant a line, watch it
  fail, restore).
- The parity test extends to all four artifacts and stays **scene-derived** —
  each committed artifact must equal the scene's own render. A hand-listed
  expectation passes while the shells drift, which is the failure this card
  exists to prevent.
- Changing one value in the Rust scene updates all four artifacts in one
  `ir:build`.
- Neither native preview manifest gained a `poodle-ir` or `poodle-codegen`
  dependency — assert it, do not just avoid it.
- Theme, size, density, contrast and tab changes remain interactive in both
  native previews.

Do not weaken an existing test to make one of these pass.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `b035`'s log (`docs/logs/2026-08/12-g13-035-…`) before starting: it
  records the capability matrix, the emitter shape, and the removal test you
  are repeating.
- **Run `effigy check:svelte`** — the web shells must not regress.
- Run `effigy docs:callback-drift`.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-036-shell-scene-native-shells`. Do not merge.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/gpui/preview/src/**`
- `packages/jetstream/preview/src/**`
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md`
- `docs/logs/2026-08/<DD>-g13-036-shell-scene-native-shells.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All start green.
2. Record the capability + label matrix for both native shells — the
   before-state the parity test replaces.
3. Add the Rust emitter target (R2). Emit both artifacts.
4. Wire the Jetstream shell — it already agrees on all five labels, so it is
   the cleaner first cutover.
5. Wire the GPUI shell, including the three label deltas (R3).
6. Prove R4: remove a control from the scene, rebuild, confirm it disappears
   from all four shells, restore it. Record what you saw.
7. Prove R5: screenshot both native shells. Record what you saw.
8. Validate:
   ```sh
   effigy ir:build
   effigy ir:check
   effigy ci:rust
   effigy ci:web
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:callback-drift
   git diff --check
   ```

## Acceptance Criteria

- [ ] All four shells render their controls and labels from one scene.
- [ ] Neither native preview depends on `poodle-ir` or `poodle-codegen`.
- [ ] `targets/shell.rs` and `synthetic-model.json` are unchanged.
- [ ] `ir:check` exits 0 clean and non-zero on drift in a Rust artifact.
- [ ] The step-6 removal test drops the control from all four.
- [ ] Both natives screenshotted; both keep every axis interactive.
- [ ] `g13.004` marked complete.
- [ ] All step-8 commands exit 0; no baseline refreshed.

## Stop Conditions

- A self-contained Rust artifact cannot carry something a native shell needs
  without importing a Poodle crate (R1).
- A native shell's control cannot be driven from generated data without moving
  runtime state ownership out of the host (R4).
- A native visual baseline moves.

Stop with exact paths, commands, and the smallest unresolved question.
