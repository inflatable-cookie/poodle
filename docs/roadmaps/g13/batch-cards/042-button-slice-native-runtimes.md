# 042 Button Vertical Slice — GPUI And Jetstream Through `poodle-render`

Status: ready
Milestone: `g13.005` (part 2 of 2 — **this card closes `g13.005`**)
Owner: Poodle core
Branch: `thread/g13-042-button-slice-native-runtimes`
Depends on: `g13-b041` (`ff391651`), merged
Governing refs: `docs/roadmaps/g13/005-button-component-vertical-slice.md`,
`docs/roadmaps/g13/batch-cards/041-button-slice-rust-authoring-and-web.md`,
`docs/roadmaps/g13/batch-cards/036-shell-scene-native-shells.md` (the
generated-Rust route this follows),
`docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`
(`R2`)

## Goal

`b041` authored Button's definition in Rust and put both web runtimes on it.
This card does the natives, through `poodle-render` — and `g13.005` closes when
it merges.

`g13.005`'s headline acceptance becomes checkable for the first time here:
**one definition change visible in all four previews.**

## Current State

- `button-ts` emits the web artifact into
  `packages/{svelte,react}/components/src/generated/button/`.
- `shell-rust` (from `b036`) already emits a **self-contained** Rust artifact —
  plain data, no `use` of any Poodle crate — into each native preview. That is
  the route to copy.
- Emitter targets today: `docs`, `json`, `schema`, `registry`, `conformance`,
  `typescript`, `shell-scene`, `shell-rust`, `button-ts`.
- `packages/render/src/button.rs` is 621 lines and hard-codes its vocabulary
  (23 mentions of variant/tone/attribute names).

## Fixed By Ruling (do not re-decide)

### R1 — A self-contained generated Rust artifact. No new crate dependency.

`b003 R2` forbids `poodle-render` depending on `poodle-ir` during the pilot,
and forbids anything at all depending on `poodle-codegen`. So `poodle-render`
may not import the IR, the authoring module, or the fixture's types.

Emit plain data with no `use` of any Poodle crate, exactly as `shell-rust`
does. **Do not add `poodle-ir` or `poodle-codegen` to
`packages/render/Cargo.toml`.** If that looks like the only way through, stop —
that is a finding for `g13.008`, not a licence.

### R1a — The artifact lives in the package that ships it.

`b041` shipped a regression by emitting a component's artifact into the preview
package: every gate passed while the published tarball could not resolve the
import. `poodle-render` is the consumer here, so the artifact belongs under
`packages/render/src/generated/`.

Not a native preview, and not `packages/codegen` — a crate cannot depend on the
tool that generates it.

### R2 — A sibling target. Do not edit `button-ts` or `shell-rust`.

Add `button-rust` beside them. `button-ts`'s output is byte-compared by
`b041`'s tests and `shell-rust`'s by `b036`'s; changing either breaks the
tests that make those cards mean anything.

Sharing a helper across targets is fine and has precedent — `b036` and `b041`
both widened a `shell.rs` helper to `pub(crate)` rather than duplicating it,
and both were accepted because the emitted bytes did not move. Do the same if
it helps, and prove the bytes did not move.

### R3 — The artifact must drive what renders.

Same ruling as `b041` R2, and now the full acceptance. A generated struct that
nothing reads proves nothing. `render/src/button.rs` must take its vocabulary —
variants, tones, the state-attribute names and their value domains — from the
generated definition instead of its own literals.

**The proof, and it is mandatory:** change one value in `button.rs`'s
definition, run `ir:build`, and show it in **all four** previews — Svelte,
React, GPUI, Jetstream. Restore. Record what you saw in each.

That single sentence is what `g13.005` has been building toward. If it cannot
be demonstrated, say so plainly; a partial result honestly reported is worth
more to `g13.008` than a claim.

### R4 — Pixels and public API unchanged.

`g13.005`: *"public APIs and current pixels remain contract-equivalent."*
`ButtonSpec` keeps its fields, `poodle-render` keeps its function signatures,
and the natives render what they render today.

**A moving native visual baseline is a stop condition, not a refresh.**
Classify the delta first and report it — a baseline can be wrong rather than
merely outdated.

### R5 — Look at both natives. "It compiles" is not evidence.

Both are pixel-verifiable locally: GPUI takes `--screenshot`, Jetstream has a
headless `snap`. Capture Button in each and say what you saw.

**Environment note, from `b036`:** `jetstream-poodle` is a sibling-repo path
dep (`../../../../jetstream/…`) that resolves from the main checkout but not
from a worktree. A `poodle-wt/poodle` symlink exists and points at the main
repo; build Jetstream through that path. This is in `PAPERCUTS.md`.

### R6 — Count the hand-written exceptions, per runtime.

`b041` did this for Svelte and React. Extend the same inventory to GPUI and
Jetstream: what came from the definition, what stayed hand-written, why. Spec
063's acceptance is *"zero or explicitly justified"*, and this inventory plus
`b041`'s is the evidence `g13.008` weighs.

## Scope

### In scope

- The `button-rust` target and its artifact under `packages/render/src/generated/`.
- `packages/render/src/button.rs` consuming it.
- `ir:build` / `ir:check` covering the new artifact.
- Tests, including the four-runtime proof.
- `docs/roadmaps/g13/005-button-component-vertical-slice.md` — status to
  complete.

### Out of scope — stop conditions if reached

- `poodle-ir` schema changes (R1). A missing field is `g13.002`'s and a
  `g13.008` finding.
- `button-ts`, `shell-rust`, `shell-scene`, `synthetic-model.json` output (R2).
- Any component other than Button.
- The 15-component native registration gap — unrelated, and `g13.014`'s.
- Refreshing a visual baseline (R4).

## Required Tests

- `ir:build` / `ir:check` exit 0 with the Rust artifact, and `ir:check` fails
  on drift in it (plant, watch, restore).
- **The four-runtime proof** (R3), recorded per runtime.
- One definition change updates every artifact in one `ir:build`.
- `packages/render/Cargo.toml` gained no `poodle-ir` or `poodle-codegen`
  dependency — assert it, do not merely avoid it.
- Existing `poodle-render` Button tests pass unedited.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `b036`'s and `b041`'s logs first: `b036` for the self-contained Rust
  route and the Jetstream symlink, `b041` for the authoring shape and its
  exception inventory.
- Run `effigy ci:web` **and** `ci:rust`. `test:web-pack-install` is now inside
  `ci:web`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-042-button-slice-native-runtimes`. Do not
  merge.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/render/src/button.rs`
- `packages/render/src/generated/**`
- `packages/render/src/lib.rs` (only to declare the generated module)
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g13/005-button-component-vertical-slice.md`
- `docs/logs/2026-08/<DD>-g13-042-button-slice-native-runtimes.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All green.
2. Read `b036`'s log for the self-contained Rust route.
3. Add the `button-rust` target; emit into `packages/render/src/generated/`.
4. Wire `render/src/button.rs` to consume it (R3).
5. Prove R3 across all four previews. Record each.
6. Prove R5: screenshot Button in GPUI and Jetstream. Record what you saw.
7. Write the R6 exception inventory for both natives.
8. Validate:
   ```sh
   effigy ir:build
   effigy ir:check
   effigy ci:rust
   effigy ci:web
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   git diff --check
   ```

## Acceptance Criteria

- [ ] One definition change is visible in **all four** previews, demonstrated.
- [ ] `poodle-render` depends on neither `poodle-ir` nor `poodle-codegen`.
- [ ] The artifact sits in `packages/render/src/generated/`.
- [ ] `button-ts` and `shell-rust` output byte-identical.
- [ ] Both natives screenshotted; no baseline refreshed.
- [ ] Exception inventory covers GPUI and Jetstream.
- [ ] `g13.005` marked complete.
- [ ] All step-8 commands exit 0.

## Stop Conditions

- The vocabulary cannot reach `poodle-render` without a crate dependency or a
  schema change.
- A native visual baseline moves.
- Jetstream cannot be built (see R5's symlink note before concluding this).
- The four-runtime proof fails for one runtime — report which and why rather
  than reshaping the claim.

Stop with exact paths, commands, and the smallest unresolved question.
