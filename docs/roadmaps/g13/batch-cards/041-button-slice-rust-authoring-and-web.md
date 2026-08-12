# 041 Button Vertical Slice — Rust Authoring And The Two Web Runtimes

Status: ready
Milestone: `g13.005` (part 1 of 2 — **this card does not close `g13.005`**)
Owner: Poodle core
Branch: `thread/g13-041-button-slice-rust-authoring-and-web`
Depends on: `g13-b035` (`db4e4510`), `g13-b036` (`97da7195`), both merged
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`,
`docs/roadmaps/g13/005-button-component-vertical-slice.md`,
`docs/roadmaps/g13/batch-cards/035-shell-scene-rust-authoring-and-web.md`
(the shape this follows), `docs/contracts/components/button.md`

## Goal

`g13.004` proved the scene path: one Rust-authored definition driving all four
shells. `g13.005` is the harder half — a **component**. This card authors
Button's definition in Rust and puts the two web runtimes on it.

Card `042` does the natives through `poodle-render`. **`g13.005` closes there,
not here.**

## Why This Is Split

`b035`/`b036` split `g13.004` the same way and both ran clean. `b038` did not
split, bundled discovery with a sweep, and died on its deadline. The risk here
is concentrated in what "generated" means for a component; once two runtimes
consume it, the natives are a second pass.

## Current State — Measured

| | |
|---|---|
| `Button.svelte` / `Button.tsx` | 220 / 164 lines |
| props on the Svelte surface | 34 |
| data attributes its DOM emits | 11 — `variant`, `tone`, `size`, `density`, `fit`, `loading`, `pressed`, `truncate`, `icon-only`, `has-leading`, `has-trailing` |
| recipe-hook uses in `button.css` | 82 |
| contract | 584 lines |
| **web components consuming generated TS today** | **none — all hand-written** |

`ComponentDefinition` in `poodle-ir` already models what this needs: `props`,
`controlled_state`, `events`, `parts`, `attributes`, `axes`, `tokens`,
`recipe_hooks`, `accessibility`, `capabilities`, `keyboard`.
`packages/codegen/fixtures/synthetic-model.json` carries three synthetic
components (`badge`, `gauge`, `search-field`); Button is the first **real** one.

## Fixed By Ruling (do not re-decide)

### R1 — Author in Rust, beside the shell scene.

`packages/codegen/src/models/button.rs`, reachable from the existing bin, the
same placement and the same pilot-scoped caveat `b035 R1` established. Not
`poodle-ir` — `b003 R1` fixed that crate as lib-only pure data, and an authored
instance is content, not schema. No new crate. No macros.

Serialize to its own fixture, as the shell model does. **Do not touch
`synthetic-model.json`** — `b022`/`b025`'s emitter tests pin it.

### R2 — The artifact must drive the DOM, not just describe types.

This is the ruling that decides whether the card proves anything.

`g13.005`'s acceptance is *"one definition change is visible in all four
previews."* A generated `type ButtonProps` cannot satisfy that — changing a
type changes nothing you can see. The existing `ts.rs` target emits exactly
that and no more, which is why it is not enough on its own.

So the generated artifact must also carry the **rendered vocabulary**: the
component's parts, its state attributes and their value domains, and its recipe
hooks. Button's Svelte and React must read those rather than hard-coding the
eleven attribute names and their values inline.

The test is concrete: **rename a state attribute in `button.rs`, run
`ir:build`, and both web previews must emit the new name in their DOM** with no
hand edit. If a change to the definition cannot move the DOM, the slice has not
been proved and that is a finding, not a smaller success.

### R3 — Public API and pixels unchanged. This is re-plumbing.

Every one of the 34 props keeps its name, type and default. The DOM keeps its
eleven attributes and their current values. `button.css` is untouched — the 82
recipe hooks are already the styling seam and this card does not restyle
anything.

A prop rename, a dropped prop, or a changed default is a **stop**, not a
tidy-up. `g13.005` says *"public APIs and current pixels remain
contract-equivalent"*, and the pilot's verdict depends on that being true
rather than approximately true.

If a visual baseline moves, **stop**. Nothing here should move a pixel.

### R4 — Count the hand-written exceptions. Out loud.

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* Some of Button will not be generated — event
wiring, the DOM element itself, framework idiom. That is expected and fine.

What is not fine is leaving it uncounted. The log must state, for each runtime:
what came from the definition, what stayed hand-written, and why. That
inventory is the evidence `g13.008` weighs; a card that generates a little and
says nothing about the rest tells the verdict nothing.

### R5 — Web only. `042` takes the natives.

Svelte first, React mirrors exactly. Do not touch `poodle-render`, either
adapter, or either native preview. `b003 R2` still forbids the previews
depending on `poodle-ir` or `poodle-codegen`; `042` will use the same
self-contained generated-Rust route `b036` established.

## Scope

### In scope

- `packages/codegen/src/models/button.rs` and its fixture.
- Whatever the emitter needs to carry the rendered vocabulary (R2) — extending
  `ts.rs`, or a sibling target if that keeps `ts.rs`'s existing output stable.
- Generated artifacts under `generated/` in both web packages.
- `Button.svelte` and `Button.tsx` consuming them.
- `ir:build` / `ir:check` covering the new artifacts.
- Tests.

### Out of scope — stop conditions if reached

- `poodle-render`, either adapter, either native preview (R5).
- `poodle-ir` schema changes. If Button needs a field the IR lacks, **stop** —
  that is `g13.002`'s, and it is a real finding for `g13.008`.
- `synthetic-model.json`, `targets/shell.rs`, and the shell scene artifacts.
- `button.css` and any visual change (R3).
- Any other component.

## Required Tests

- `ir:build` and `ir:check` exit 0 with Button's artifacts, and `ir:check`
  fails on drift in them (plant, watch, restore).
- **The R2 proof**: change a state attribute's name in `button.rs`, rebuild,
  and show both web previews emitting it. Restore. Record what you saw.
- One definition change updates both web artifacts in one `ir:build`.
- The public prop surface is byte-identical before and after — prove it, do not
  assert it. `svelte:surface-audit` and `docs:contract-drift` both still pass.
- Every existing Button test passes unchanged. Do not edit one to fit.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `b035`'s log first — it records the authoring shape, the emitter
  structure and the removal-test method you are repeating.
- **Run `effigy check:svelte`**, plus `docs:contract-drift`, `drift:recipes`,
  `svelte:surface-audit`.
- Verify every governing-ref path resolves before relying on it; `b040`'s card
  cited one that did not exist.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-041-button-slice-rust-authoring-and-web`. Do
  not merge.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/fixtures/**` (new files only)
- `packages/codegen/tests/**`
- `packages/svelte/components/src/Button.svelte`
- `packages/react/components/src/Button.tsx`
- `packages/{svelte,react}/components/test/Button*.test.*`
- `packages/{svelte,react}/preview/src/**/generated/**`
- `tasks/effigy.tasks.toml`
- `docs/contracts/components/button.md` (only if the definition records
  something the contract does not — say so)
- `docs/roadmaps/g13/005-button-component-vertical-slice.md` (status line only)
- `docs/logs/2026-08/<DD>-g13-041-button-slice-rust-authoring-and-web.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `test:components`, `git diff --check`. All green.
2. Read `button.md` and `Button.svelte` and write down the surface you must
   preserve exactly: 34 props, 11 attributes and their value domains.
3. Author `button.rs`. Serialize; confirm it validates.
4. Emit. Decide and record how the rendered vocabulary reaches the runtimes
   (R2).
5. Wire Svelte to consume it.
6. Mirror React exactly.
7. Prove R2: rename an attribute in the definition, rebuild, observe both
   previews, restore. Record it.
8. Write the R4 exception inventory.
9. Validate:
   ```sh
   effigy ir:build
   effigy ir:check
   effigy ci:rust
   effigy test:core
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:callback-drift
   effigy docs:focus-ring-drift
   effigy drift:recipes
   effigy svelte:surface-audit
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] Button's definition is Rust-authored; `poodle-ir` gained no field and no
  `[[bin]]`.
- [ ] A definition change moves the DOM in both web previews (R2), shown.
- [ ] All 34 props unchanged; all 11 attributes unchanged; no pixel moved.
- [ ] The hand-written exception inventory exists, per runtime, with reasons.
- [ ] `synthetic-model.json` and `targets/shell.rs` untouched.
- [ ] All step-9 commands exit 0; no baseline refreshed.

## Stop Conditions

- Button needs a `poodle-ir` field that does not exist.
- The rendered vocabulary cannot reach a runtime without a schema change or a
  new preview dependency.
- Preserving all 34 props exactly conflicts with generating them.
- A visual baseline moves.

Stop with exact paths, commands, and the smallest unresolved question.
