# 004 Machine Interface Authority

Status: ready
Milestone: `g14.004`
Owner: Poodle core
Branch: `thread/g14-004-machine-interface-authority`
Depends on: `g14.002` (merged), `g14.001` (merged)
Governing refs: `docs/specs/064-cross-runtime-machine-pinning.md` (mechanism
1), `docs/roadmaps/g14/004-machine-interface-authority.md`,
`docs/architecture/006-headless-core-and-machine-model.md` (Pinning
Contract), `../g11/002-headless-machine-spec-format-and-pilot-contracts.md`,
`packages/contracts/headless/vectors/machines.json`

## Goal

Make the machine contract — states, events, effects, context types — one
schema that generates the TypeScript and Rust type declarations. Interface
only: no transitions, guards, or derivation. An event mismatch becomes a
compile error in both languages; b047's shape drift dies structurally.

## Fixed By Ruling (recorded — do not re-decide)

- **R1 — Generator home is `poodle-codegen`.** It already owns the TS
  emitter, the drift-check machinery, and the `ir:build`/`ir:check`
  discipline. Machine-interface targets join the shell scene as its second
  use. No new tool.
- **R2 — The interface schema is a new sibling, not `machines.json`.**
  `machines.json` is the vectors' file; both existing harnesses read it
  and both must keep running unedited. New:
  `packages/contracts/headless/machine-interfaces.json`.
- **R3 — Pilot machines first, 1:1, no renames:** `hover`, `menu`,
  `modal`, `popover` (the four canonical). Generated declarations replace
  the existing hand-written types; no public export renamed (b047 R3 —
  consumers are file-linked).
- **R4 — Interface only.** The schema holds types and names. If a machine
  shape cannot be expressed without transitions/guards/derivation, it is
  a reported finding, never absorbed by widening the schema into
  behaviour (spec 064 boundary).

## Deliverables

- `machine-interfaces.json` with the four pilot machines' interfaces,
  declared from the existing implementations, not invented.
- Emitter targets generating TS declarations into
  `packages/core/src/generated/machines/` and Rust declarations into
  `packages/contracts/headless/src/generated/machines/`, deterministic and
  drift-gated (`ir:build`/`ir:check`).
- The four pilot machines' hand-written type declarations replaced by the
  generated ones; both runtimes compile.
- A gate that fails when a machine's interface is not generated from the
  schema.
- The LOC comparison: generated interfaces vs the declarations they
  replaced.

## Acceptance

- [ ] The four pilots compile in both runtimes from generated interfaces
  with no public export renamed.
- [ ] A planted interface divergence fails the gate on the authored side,
  naming the machine.
- [ ] Generated interfaces are smaller than the declarations they
  replace.
- [ ] `effigy ci:rust`, `effigy test:core`, `effigy test:components`,
  `effigy ci:web`, `effigy docs:lint`, `git diff --check` all exit 0.

## Stop Conditions

- A pilot machine's shape cannot be expressed as pure interface — report
  the exact machine and what it needs; do not widen the schema.
- Replacing a declaration would rename a public export — stop and report;
  the additive-conformance route (b047 R3) applies instead.

## Writable Paths

- `packages/contracts/headless/machine-interfaces.json` (new)
- `packages/contracts/headless/src/{hover,menu,modal,popover}.rs`
- `packages/core/src/{hover,menu,modal,popover}.ts`
- `packages/core/src/generated/machines/**` (new, generated)
- `packages/contracts/headless/src/generated/machines/**` (new, generated)
- `packages/codegen/**` (new machine-interface targets only; shell targets
  untouched)
- `tasks/effigy.tasks.toml`
- `docs/logs/2026-08/14-g14-004-machine-interface-authority.md`
- `PAPERCUTS.md` (append only)

## Steps

1. Reset per the Thread Reuse Protocol; baseline
   `effigy test:core`, `effigy ci:rust`, `effigy ci:web`, `git diff --check`.
2. Read spec 064 mechanism 1 and the four machines' existing
   declarations in both runtimes.
3. Author `machine-interfaces.json` from the implementations (types and
   names only); add the codegen targets; emit the four pairs of
   declarations.
4. Swap the hand-written types for the generated ones in the eight
   modules (4 TS + 4 Rust); run core and rust tests after each.
5. Add the gate (interface not generated → fail); plant-and-restore proof
   on one machine.
6. Validate the acceptance gate list; report the LOC delta.
7. Write the batch log; push with
   `git push -u origin thread/g14-004-machine-interface-authority`. Do not
   merge.
