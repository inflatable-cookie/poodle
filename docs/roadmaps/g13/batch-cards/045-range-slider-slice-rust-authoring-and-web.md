# 045 RangeSlider Stateful Proof — Rust Authoring And The Two Web Runtimes

Status: ready
Milestone: `g13.006` (part 1 of 2 — **this card does not close `g13.006`**)
Owner: Poodle core
Branch: `thread/g13-045-range-slider-slice-rust-authoring-and-web`
Depends on: `g13-b041` (`ff391651`), `g13-b042` (`c5fa2f85`), both merged —
`g13.005` is complete
Governing refs: `docs/roadmaps/g13/006-range-slider-stateful-control-proof.md`,
`docs/specs/063-rust-authored-component-and-scene-ir.md`,
`docs/contracts/components/range-slider.md`,
`docs/roadmaps/g13/batch-cards/041-button-slice-rust-authoring-and-web.md`
(the shape this follows)

## Goal

`g13.005` proved the **easy** case. Button is stateless: props in, attributes
out. This is the card that tries to break the model — controlled state, gesture
effects, repeated anatomy, and geometry that depends on the value.

Card `046` does the natives. **`g13.006` closes there, not here.**

`g13.006` exists to produce a *result*, not a success. A model that cannot
express two thumbs is a finding the `g13.008` verdict needs; a card that
quietly reshapes the component until it fits tells the verdict nothing.

## Current State — Measured

| | |
|---|---|
| `RangeSlider.svelte` / `.tsx` | 176 / 185 lines |
| props | 18 |
| data attributes | 8 |
| `packages/core/src/slider.ts` (TS machine) | 344 lines |
| `packages/contracts/headless/src/slider.rs` (Rust machine) | 607 lines |
| `packages/render/src/range_slider.rs` | 497 lines |
| contract | 564 lines |

**The slider machine already exists twice** — hand-written in TS and in Rust —
and is **already pinned** by a shared conformance vector (`slider` is a key in
`packages/contracts/headless/vectors/machines.json`). That is not an accident
of history; `g13.006`'s own deliverables call for *"shared conformance vectors
where runtime machines remain hand-written."*

## Fixed By Ruling (do not re-decide)

### R1 — The IR declares the machine. It does not absorb it.

The roadmap is explicit: declarative transition/effect intent **where
portable**, and shared conformance vectors **where runtime machines remain
hand-written**. Both slider machines stay hand-written.

So do **not** port `slider.ts` or `slider.rs` into the IR, and do not invent a
state-machine encoding to make them generated. If the IR cannot express some
transition declaratively, that is the expected answer for this component — say
so and fall back to the vector.

What the definition *does* carry is everything Button's did — props, parts,
state attributes and their value domains, axes, recipe hooks, accessibility —
plus the value-dependent bits R2 names.

### R2 — The three things Button could not test.

Button was stateless, so `g13.005` proved nothing about these. Each needs an
explicit answer in the log, even if the answer is "the IR cannot express it":

1. **Repeated anatomy.** Two thumbs, from one definition. Button's parts were
   singular. Can the IR describe a part that occurs N times, or does the
   renderer hard-code "two"?
2. **Value-dependent geometry.** Unipolar and bipolar fill: which side of the
   origin the fill grows from, and how negative fill is expressed. This is the
   acceptance line *"negative/positive fill geometry and recipe roles remain
   exact."*
3. **Gesture effects.** Begin/move/end semantics and thumb selection. The
   contract already specifies them; the question is whether the definition can
   *declare* them or only name them for a hand-written machine to honour.

### R3 — The boundary stays where it is.

Pointer capture, hit-testing, keyboard input and ARIA value projection are
**adapter-owned**, per the roadmap and the standing architecture rule. The
definition may declare that they exist and what they mean. It must not
implement them, and no runtime may read machine state from drawing code.

### R4 — Public API and pixels unchanged.

All 18 props keep their names, types and defaults; the 8 data attributes keep
their values. This is re-plumbing, as `041` was. A prop rename is a **stop**.
If a visual baseline moves, **stop** — geometry is the thing most likely to
shift here, and a moved pixel is a real finding rather than a tidy-up.

### R5 — The conformance vector is the safety net. Use it.

`slider` is already pinned. Whatever the definition ends up declaring, the
existing vector must still pass against **both** machines, unedited. If a
change would require editing the vector, that is a behaviour change wearing a
refactor's clothes — stop and say so.

If the work reveals the vector is thin — that it would not have caught a real
divergence — say that too. It is useful evidence about the mechanism the
roadmap is relying on.

### R6 — Web only. `046` takes the natives.

Svelte first, React mirrors exactly. Do not touch `poodle-render`, either
adapter, or either native preview. Follow `042`'s route when it comes: the
artifact lives in the package that ships it, and `b003 R2` still bars the
previews and `poodle-render` from depending on `poodle-ir`.

### R7 — Count the hand-written exceptions, as `041` did.

Per runtime: what came from the definition, what stayed hand-written, why. For
a stateful control this inventory is the *main* output — `g13.008` is deciding
whether the model earns its keep, and the honest size of the hand-written
remainder is the number that decides it.

## Scope

### In scope

- `packages/codegen/src/models/range_slider.rs` and its fixture.
- Emitter work to carry whatever R2 needs beyond Button's vocabulary.
- Generated artifacts in both web component packages.
- `RangeSlider.svelte` / `.tsx` consuming them.
- `ir:build` / `ir:check` coverage, tests, and the R7 inventory.

### Out of scope — stop conditions if reached

- `poodle-render`, either adapter, either native preview (R6).
- `packages/core/src/slider.ts` and `packages/contracts/headless/src/slider.rs`
  — the machines stay hand-written (R1).
- `packages/contracts/headless/vectors/**` — the vector is a fixed target (R5).
- `poodle-ir` schema changes. If two thumbs need a field the IR lacks,
  **stop** — that is exactly the finding `g13.006` exists to produce.
- Button's artifacts, `synthetic-model.json`, the shell scene.
- Refreshing a visual baseline (R4).

## Required Tests

- `ir:build` / `ir:check` exit 0; `ir:check` fails on drift in the new
  artifacts (plant, watch, restore).
- A definition change moves the DOM in both web previews, as `041` proved for
  Button.
- The existing `slider` conformance vector passes unedited against the TS
  machine.
- All 18 props and 8 attributes unchanged — prove it, do not assert it.
- Two thumbs still render, drag, and honour detents; bipolar fill still grows
  from the origin. Existing RangeSlider tests pass unedited.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `041`'s log first for the authoring shape and its exception inventory.
- **A negative result is a result.** If the IR cannot express repeated anatomy
  or value-dependent geometry, write that down precisely and stop at that
  boundary rather than reshaping RangeSlider to fit.
- Run `effigy ci:web` (which now includes `test:web-pack-install`) and
  `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-045-range-slider-slice-rust-authoring-and-web`.
  Do not merge.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/fixtures/**` (new files only)
- `packages/codegen/tests/**`
- `packages/svelte/components/src/RangeSlider.svelte`
- `packages/react/components/src/RangeSlider.tsx`
- `packages/{svelte,react}/components/src/generated/**`
- `packages/{svelte,react}/components/test/RangeSlider*.test.*`
- `tasks/effigy.tasks.toml`
- `docs/contracts/components/range-slider.md` (only if the definition records
  something the contract does not — say so)
- `docs/logs/2026-08/<DD>-g13-045-range-slider-slice-rust-authoring-and-web.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `test:components`, `git diff --check`. All green.
2. Read `range-slider.md` and both machines. Write down the surface to
   preserve: 18 props, 8 attributes, and the geometry rules.
3. Author `range_slider.rs`. Stop and record the moment R2's three questions
   get an answer — including a negative one.
4. Emit; wire Svelte.
5. Mirror React exactly.
6. Prove the definition moves the DOM in both previews.
7. Confirm the `slider` vector passes unedited.
8. Write the R7 exception inventory. For this component it is the headline.
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
   effigy drift:recipes
   effigy svelte:surface-audit
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] R2's three questions each answered explicitly in the log.
- [ ] The machines are untouched and the `slider` vector passes unedited.
- [ ] 18 props, 8 attributes, no pixel moved.
- [ ] A definition change moves the DOM in both web previews.
- [ ] The hand-written exception inventory exists, per runtime.
- [ ] All step-9 commands exit 0; no baseline refreshed.

## Stop Conditions

- Repeated anatomy (two thumbs) needs a `poodle-ir` field that does not exist.
- Value-dependent fill geometry cannot be declared without a runtime-specific
  value path.
- The `slider` vector would need editing.
- A visual baseline moves.

Each of these is a **finding for `g13.008`**, not a failure. Stop with exact
paths, commands, and the smallest unresolved question.
