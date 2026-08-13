# 048 TextInput Environment Boundary — Rust Authoring And The Two Web Runtimes

Status: ready
Milestone: `g13.007` (part 1 of 2 — **this card does not close `g13.007`**)
Owner: Poodle core
Branch: `thread/g13-048-text-input-slice-rust-authoring-and-web`
Depends on: `g13-b045` (`db4e587a`), `g13-b046` (`9621d119`), both merged —
`g13.006` is complete
Governing refs: `docs/roadmaps/g13/007-text-input-environment-boundary-proof.md`,
`docs/specs/063-rust-authored-component-and-scene-ir.md`,
`docs/contracts/components/text-input.md`,
`docs/logs/2026-08/13-g13-045-range-slider-slice-rust-authoring-and-web.md`,
`docs/logs/2026-08/13-g13-046-range-slider-slice-native-runtimes.md`
(**read both — this card is the third data point in their series**)

## Goal

`g13.005` proved the stateless case. `g13.006` proved the stateful one and
returned two negative findings. This is the last slice before `g13.008` decides
the IR's fate, and it is the one aimed squarely at the model's weakest claim:
that a Rust-authored definition can sit above **environment-owned** behaviour —
focus, selection, IME composition, clipboard, measurement — without generating
lifecycle code or weakening input semantics.

Card `049` does the natives. **`g13.007` closes there, not here.**

As with `045`: this milestone exists to produce a *result*, not a success. A
model that cannot express the environment boundary is exactly the finding
`g13.008` needs.

## Current State — Measured

| | |
|---|---|
| `TextInput.svelte` / `.tsx` | 631 / 553 lines |
| props | **49** |
| documented data attributes | **3** (`data-size`, `data-density`, `data-validation-state`) |
| `packages/contracts/headless/src/text_input.rs` | 837 lines |
| `packages/render/src/text_input.rs` | 625 lines |
| contract | 712 lines |

### The ratio is the reason this card exists

| Component | Props | Documented data attributes | Props per attribute |
|---|---|---|---|
| Button | 34 | 11 | ~3 |
| RangeSlider | 18 | 4 | ~5 |
| **TextInput** | **49** | **3** | **~16** |

Button's surface was mostly *vocabulary* — names and value domains that map to
attributes, which is precisely what the IR absorbed well. TextInput's is not.
Its props are overwhelmingly behavioural and environmental: `debounce`,
`validate`, `validationContext`, `validationDebounce`, `validateOnBlur`,
`autocomplete`, `inputMode`, `enterKeyHint`, `spellcheck`, `autocapitalize`,
`autocorrect`, `pattern`, `maxLength`, `resize`.

Counting method, so the next card can reproduce it: props are the entries of
the Svelte `Props` interface; attributes are the distinct `` `data-*` ``
tokens in the contract. A literal `data-` grep of the component sources
undercounts (Button and RangeSlider build attributes programmatically), so the
contract is the authority for this table.

### The environment split is already asymmetric — measured

Occurrence counts of environment terms in each implementation:

| Source | composition/IME | selection/caret | focus | clipboard |
|---|---|---|---|---|
| `TextInput.svelte` | 20 | **0** | 13 | **0** |
| `TextInput.tsx` | 18 | **0** | 15 | **0** |
| `contracts/headless/src/text_input.rs` | 6 | **67** | 1 | 18 |
| `render/src/text_input.rs` | 1 | **60** | 16 | 3 |

The web runtimes handle composition and touch selection **not at all** — the
DOM `<input>` owns selection, so there is nothing to write. The Rust side is
the mirror image: it implements the editing model itself. This is not a tidiness
problem; it is the shape of the boundary the milestone is named after.

### There is no TypeScript text machine, and that is not an oversight

`packages/core/src/text-input.ts` **does not exist**. `packages/core/src/input.ts`
is helper functions only — `validationStatusToState`, `parseNumberish`,
`clampNullable`, `slugify` — not a machine. `b047` classified
`contracts/headless/src/text_input.rs` as *correctly different* (an editing
model, not a behaviour machine) and baselined it in
`docs/machine-shape-drift` as `rs:text_input`.

So the two sides are asymmetric **by design**: the browser supplies the editing
model, and native has to implement one. Do not create a TS machine to make the
table look symmetrical — that is the failure mode `b047 R1` exists to prevent.

## Fixed By Ruling (do not re-decide)

### R1 — Follow `045`'s route exactly, with `b041`'s packaging correction.

Rust-authored definition, emitters as siblings, self-contained generated
artifacts. **Artifacts live in the package that ships them** —
`packages/{svelte,react}/components/src/generated/` — never in a preview
package. `b041` shipped that regression because a card said otherwise, and
`test:web-pack-install` (now inside `ci:web`) is what catches it.

`b003 R2` still holds: no `poodle-ir` or `poodle-codegen` dependency in the
previews or `poodle-render`. Nothing depends on `poodle-codegen`.

Add sibling targets. Do not change `button-ts`, `button-rust`,
`range-slider-ts`, `range-slider-rust` or `shell-*` output — their tests
byte-compare it. Sharing a helper is fine with proof the bytes did not move.

### R2 — Declare the capability boundary. Never implement it.

The deliverable is a **typed capability boundary** for focus, selection,
composition/IME, clipboard, measurement and native text editing. The definition
may declare that a capability exists, what it means, and which runtime owns it.
It must not implement it, and no runtime may read machine state from drawing
code.

The standing architecture rule is unchanged: runtime adapters retain
focus/IME/portals/measurement/pointer-capture/lifecycle/input/hit-testing/
accessibility; drawing consumes serializable VisualState only.

**A capability that cannot be typed is a finding, not a blocker to route
around.** Record it and continue.

### R3 — Answer the asymmetry question explicitly. It is the card's headline.

The measured table above shows web and native own opposite halves of the
editing model. Answer, in the log:

1. Can **one** declaration serve both, with each runtime honouring the half it
   owns — or does the boundary have to be declared per-runtime?
2. For each of the six capabilities, name the owner per runtime and say whether
   the definition can express that ownership or merely note it in prose.
3. `045`/`046` found the IR carries *vocabulary* but not *behaviour*. With 49
   props mapping to 3 attributes, how much of TextInput's surface does the
   definition actually reach?

Question 3 is the number `g13.008` turns on. A small honest number is worth far
more than a large defended one.

### R4 — Public API and pixels unchanged.

All 49 props keep their names, types and defaults. The 3 data attributes keep
their values. A prop rename is a **stop**. If a visual baseline moves, **stop**
and classify the delta rather than refreshing it — `b042` correctly called a
GPUI delta *stale, not moved*; `b046` refreshed nothing.

### R5 — The editing model stays hand-written. Do not port it, do not mirror it.

`contracts/headless/src/text_input.rs` is not going into the IR, and no TS
counterpart gets invented (see Current State). If the IR cannot declare some
editing transition, that is the expected answer for this component — say so.

### R6 — Web only. `049` takes the natives.

Svelte first, React mirrors exactly. Do not touch `poodle-render`, either
adapter, or either native preview.

### R7 — The exception inventory is the headline output, and this one decides.

Per runtime: what came from the definition, what stayed hand-written, why.
`g13.008` is next, and this is the last slice feeding it. State the size of the
hand-written remainder plainly, and compare it to Button's and RangeSlider's so
the trend across the three is visible in one place.

## Scope

### In scope

- `packages/codegen/src/models/text_input.rs` and its fixture.
- Emitter work for whatever R2's capability boundary needs beyond existing
  vocabulary.
- Generated artifacts in both web component packages.
- `TextInput.svelte` / `.tsx` consuming them.
- `ir:build` / `ir:check` coverage, tests, and the R7 inventory.

### Out of scope — stop conditions if reached

- `poodle-render`, either adapter, either native preview (R6).
- `packages/contracts/headless/src/text_input.rs` (R5).
- Creating `packages/core/src/text-input.ts`.
- `packages/contracts/headless/vectors/**` — `b047` just pinned all 21
  machines; the vectors are a fixed target.
- `poodle-ir` schema changes. If a capability needs a field the IR lacks,
  **stop** — that is `g13.007`'s finding.
- Button's / RangeSlider's artifacts, `synthetic-model.json`, the shell scene.
- Refreshing a visual baseline (R4).
- `NumberInput`, `CodeInput`, `EmbedInput` and any other entry component.

## Required Tests

- `ir:build` / `ir:check` exit 0; `ir:check` fails on drift in the new
  artifacts (plant, watch, restore).
- A definition change moves the DOM in **both** web previews.
- All 49 props and 3 attributes unchanged — prove it, do not assert it.
- IME composition still works: a composition sequence must not fire
  intermediate `onValueChange`. Test it in both runtimes; this is the
  acceptance line *"IME composition and selection behavior remain
  runtime-native and correct."*
- Selection is untouched by the definition — the DOM still owns it.
- `packages/{svelte,react}/components/package.json` gained no dependency on
  `poodle-ir`/`poodle-codegen`; `test:web-pack-install` passes.
- Existing TextInput tests pass unedited.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read `045`'s and `046`'s logs before starting.** R3 continues their
  question; you cannot answer it without them.
- **A negative result is a result.** If the definition cannot express the
  capability boundary, write that down precisely and stop at that boundary
  rather than reshaping TextInput to fit.
- Do not invent symmetry between the runtimes. The asymmetry is the subject.
- Run `effigy ci:web` (includes `test:web-pack-install`) and `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-048-text-input-slice-rust-authoring-and-web`.
  Do not merge.
- `PAPERCUTS.md` is append-only and shared: do not reflow neighbours.

## Writable Paths

- `packages/codegen/src/**`
- `packages/codegen/fixtures/**` (new files only)
- `packages/codegen/tests/**`
- `packages/svelte/components/src/TextInput.svelte`
- `packages/react/components/src/TextInput.tsx`
- `packages/{svelte,react}/components/src/generated/**`
- `packages/{svelte,react}/components/test/TextInput*.test.*`
- `tasks/effigy.tasks.toml`
- `docs/contracts/components/text-input.md` (only if the definition records
  something the contract does not — say so)
- `docs/logs/2026-08/<DD>-g13-048-text-input-slice-rust-authoring-and-web.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `test:components`, `git diff --check`. All green.
2. Read `045`'s and `046`'s logs.
3. Read `text-input.md` and both Rust sources. Write down the surface to
   preserve: 49 props, 3 attributes, and the validation/debounce semantics.
4. Author `text_input.rs`. Stop and record the moment R2's capability boundary
   gets an answer — including a negative one.
5. Emit; wire Svelte. Mirror React exactly.
6. Prove the definition moves the DOM in both previews.
7. Prove IME composition still behaves (Required Tests).
8. Answer R3's three questions.
9. Write the R7 exception inventory with the three-component comparison.
10. Validate:
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
    effigy docs:machine-shape-drift
    effigy drift:recipes
    effigy svelte:surface-audit
    effigy ci:web
    git diff --check
    ```

## Acceptance Criteria

- [ ] R2's capability boundary is typed, or its failure to be typed is recorded
  per capability.
- [ ] R3's three questions each answered explicitly in the log.
- [ ] The Rust editing model is untouched; no TS machine was created.
- [ ] 49 props, 3 attributes, no pixel moved.
- [ ] A definition change moves the DOM in both web previews.
- [ ] IME composition proven intact in both runtimes.
- [ ] The exception inventory exists per runtime, with the Button /
  RangeSlider / TextInput comparison.
- [ ] All step-10 commands exit 0; no baseline refreshed.

## Stop Conditions

- A capability cannot be typed without a `poodle-ir` field that does not exist.
- Declaring the boundary would require generating lifecycle code.
- Honouring the definition changes composition, selection or focus behaviour.
- A visual baseline moves.

Each is a **finding for `g13.008`**, not a failure. Stop with exact paths,
commands, and the smallest unresolved question.
