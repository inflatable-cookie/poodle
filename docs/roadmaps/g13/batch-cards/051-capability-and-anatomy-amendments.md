# 051 Capability And Anatomy Amendments

Status: ready
Milestone: `g13.018` (**this card closes `g13.018`**)
Owner: Poodle core
Branch: `thread/g13-051-capability-and-anatomy-amendments`
Depends on: `g13-b050` (`7dbe54f9`), merged — `g13.017` is complete
Governing refs: `docs/roadmaps/g13/018-capability-and-anatomy-amendments.md`,
`docs/roadmaps/g13/pilot-verdict-evidence.md` (**§4 failed assumptions 2 and 3
are the two gaps this card fixes**),
`docs/specs/063-rust-authored-component-and-scene-ir.md` (the amended scope
section — both amendments are vocabulary, so both are in scope),
`docs/logs/2026-08/13-g13-049-text-input-slice-native-runtimes.md`
(the capability finding, measured)

## Goal

The pilot produced two expressiveness gaps, and they are the concrete reason
the verdict was **revise** rather than **reject**. Fix both.

Both are **vocabulary** — what a runtime has, and what a component is made of.
Neither is behaviour. The narrowed boundary from `g13.017` holds throughout:
no expression tree, no evaluator, no conditional-render construct.

## Current State — Measured

### Amendment 1 — capability absence cannot be said

```rust
pub struct CapabilityRequirement {
    pub capability: Capability,
    pub purpose: String,
}
```

Two fields. Ownership is prose inside `purpose`; **absence has no
representation at all**. Measured consequence, from `b049`: Jetstream has zero
`on_edit_key` / `on_select_range` / `on_edit_insert` references and no
`ime.rs`, renders a text field nobody can type into, and is declared
identically to GPUI, which implements the whole editing model
(`ime.rs` 218 lines, `input_text.rs` 574).

**The affordance already exists.** `RuntimeTarget` is defined in
`conformance.rs:20` with all four runtimes (`Svelte`, `React`, `Gpui`,
`Jetstream`). Reuse it. Do not define a second runtime enum.

### Amendment 2 — `PartKind::Repeated` is unused dead vocabulary

```rust
Repeated {
    over: Identifier,      // must be a List prop
    description: String,
}
```

Measured: **no model uses it.** The only mentions of `Repeated` in
`packages/codegen/src/models/` are `range_slider.rs`'s doc comment explaining
why it cannot be used. Its own doc comment in `parts.rs` names *"the two
RangeSlider thumbs"* as its motivating example — the case it cannot serve,
because it requires a `List` prop and yields identical instances with no
per-item identity.

What RangeSlider does instead: two distinct declared parts
(`control-lower` / `control-upper`), with `render/src/range_slider.rs` calling
`make_thumb()` twice (lines 235–236) and hard-coding
`RangeThumb::Lower` / `RangeThumb::Upper`.

Because nothing uses `Repeated`, **it can be replaced outright — there is no
migration**. That is the cheapest this amendment will ever be.

## Fixed By Ruling (do not re-decide)

### R1 — Both amendments are vocabulary. The `g13.017` boundary holds.

Declaring *that* a runtime lacks a capability is vocabulary. Declaring *what*
it does instead is behaviour and is out of scope. Declaring that an anatomy has
two identified instances is vocabulary. Declaring how a runtime lays them out
is behaviour.

Re-introducing an expression tree, an evaluator, or a conditional-render
construct **requires a new verdict, not a card** — spec 063 now says so. If an
amendment seems to need one, **stop**.

### R2 — Reuse `RuntimeTarget`. Do not invent a second runtime enum.

`conformance.rs:20` already has the four. A second list would be exactly the
two-homes problem the pilot's drift gating exists to prevent.

### R3 — Absence must be declared, not inferred.

A component declares, per capability, which runtimes provide it and which do
not. "Not listed" must not silently mean "absent" — the whole finding is that
silence is indistinguishable from working. Make the absent case explicit and
make it carry a reason, the way baselines in every other gate here do.

### R4 — The gate is the deliverable, not the type.

A type nobody checks changes nothing. Wire a gate that fails when:

- a declared absence stops being true — e.g. Jetstream gains edit handlers
  while still declared as lacking text editing; **or**
- a runtime is declared as providing a capability it has no trace of.

Static checking is fine and is how `docs:react-specimen-drift` works: that gate
exists because a runtime-only guard shipped a fatally broken preview with every
other gate green. Prove this one on **both** directions independently — plant
each violation, watch it fail, restore.

### R5 — Per-item identity, and RangeSlider stops hard-coding "two".

Replace `Repeated` with a construct that expresses a **fixed set of identified
instances** — the two thumbs, each with its own identity and its own declared
semantics. The count comes from the definition.

`render/src/range_slider.rs` must derive the pair from the definition rather
than calling `make_thumb()` twice. If the render still hard-codes the count
after the amendment, the amendment did not land.

**Pixels do not move.** A moved native baseline is a stop condition, not a
refresh — `b042`, `b046` and `b049` each refreshed nothing.

### R6 — Public API and artifacts.

No prop renamed in any runtime; no component behaviour changed. Generated
artifacts will change — this card adds vocabulary — but every change must be
attributable to a declared amendment, and web component artifacts must not
change in ways that alter the DOM.

## Scope

### In scope

- `packages/contracts/ir/src/capabilities.rs`, `parts.rs`, `validation.rs`.
- `packages/codegen/src/models/{range_slider,text_input}.rs` — declaring the
  new vocabulary.
- `packages/codegen/src/targets/**` — emitting it.
- `packages/render/src/range_slider.rs` — consuming the identified pair (R5).
- The new gate and its wiring.
- `docs/specs/063-*.md` — record both amendments as delivered.

### Out of scope — stop conditions if reached

- Implementing any capability in any runtime. Jetstream still gets no text
  editing; that absence is now *declared*, not fixed.
- Behaviour of any kind (R1).
- `packages/contracts/headless/**` and the vectors.
- Vocabulary coverage for other components — that is `g13.019`.
- Any component file, CSS, or contract outside what R5 names.
- Refreshing a baseline.

## Required Tests

- The TextInput definition states that Jetstream lacks text editing, and the
  gate fails if that stops being true.
- The gate fails, independently, when a runtime claims a capability it has no
  trace of.
- The RangeSlider definition expresses two identified thumbs; `range_slider.rs`
  derives the count from it. Assert the count is not literal in the render.
- `ir:build` / `ir:check` pass; `ir:check` still fails on a planted byte.
- One definition change still reaches all four runtimes.
- Existing `poodle-render` and component tests pass unedited; no baseline
  refreshed.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read the verdict evidence §4 first** — assumptions 2 and 3 are these gaps.
- Reuse `RuntimeTarget` (R2). Replace `Repeated` outright (no migration).
- A gate that cannot be made to fail is not a gate. Prove both directions.
- Run `effigy ci:web` (includes `test:web-pack-install`) and `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
  `packages/codegen/generated/**` is writable here — `ir:build` rewrites it and
  stale dumps fail `ir:check`, as `b050` found.
- Commit and push with
  `git push -u origin thread/g13-051-capability-and-anatomy-amendments`. Do not
  merge.
- `PAPERCUTS.md` is append-only and shared: do not reflow neighbours.

## Writable Paths

- `packages/contracts/ir/src/**`
- `packages/contracts/ir/tests/**`
- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/codegen/fixtures/**`
- `packages/codegen/generated/**`
- `packages/render/src/range_slider.rs`
- `packages/render/src/generated/**`
- `packages/{svelte,react}/components/src/generated/**`
- `packages/svelte/preview/scripts/**`
- `tasks/effigy.tasks.toml`
- `docs/specs/063-rust-authored-component-and-scene-ir.md`
- `docs/roadmaps/g13/018-capability-and-anatomy-amendments.md` (status only)
- `docs/logs/2026-08/<DD>-g13-051-capability-and-anatomy-amendments.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All green. Record artifact checksums.
2. Read the verdict evidence §4.
3. Amendment 1: extend `CapabilityRequirement` with per-runtime provision and
   explicit, reasoned absence, reusing `RuntimeTarget`.
4. Declare TextInput's real split: web delegates, GPUI implements, Jetstream
   absent.
5. Amendment 2: replace `Repeated` with identified instances; declare
   RangeSlider's two thumbs.
6. Wire `range_slider.rs` to derive the pair (R5).
7. Add the gate; prove both failure directions independently.
8. Diff artifacts against the recorded checksums; attribute every change.
9. Prove propagation still reaches all four runtimes.
10. Validate:
    ```sh
    effigy ir:build
    effigy ir:check
    effigy ci:rust
    effigy ci:web
    effigy test:core
    effigy test:components
    effigy test:parity
    effigy check:svelte
    effigy docs:lint
    git diff --check
    ```

## Acceptance Criteria

- [ ] Capability absence is declarable, carries a reason, and is not inferred
  from silence.
- [ ] TextInput declares the real three-way split across four runtimes.
- [ ] `Repeated` is gone; identified instances replace it.
- [ ] `range_slider.rs` no longer hard-codes the thumb count.
- [ ] The gate fails on each direction independently and passes clean.
- [ ] No prop renamed, no behaviour changed, no baseline refreshed.
- [ ] Spec 063 records both amendments as delivered.
- [ ] `g13.018` marked complete; all step-10 commands exit 0.

## Stop Conditions

- An amendment cannot be expressed without an expression tree or evaluator
  (R1) — that needs a new verdict.
- Deriving the thumb pair changes RangeSlider's rendering or geometry.
- A native visual baseline moves.
- Declaring absence would require implementing anything.

Stop with exact paths, commands, and the smallest unresolved question.
