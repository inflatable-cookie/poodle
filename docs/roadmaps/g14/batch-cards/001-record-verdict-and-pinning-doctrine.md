# 001 Record The Verdict And Pinning Doctrine

Status: ready
Milestone: `g14.001`
Owner: Poodle core
Branch: `thread/g14-001-record-verdict-and-pinning-doctrine`
Depends on: none (runs alongside `g13-b053`)
Governing refs:
`docs/roadmaps/g13/020-consolidate-and-reassess.md` (recorded verdict),
`docs/roadmaps/g13/pilot-verdict-evidence.md`,
`docs/roadmaps/g14/001-record-verdict-and-pinning-doctrine.md`,
`docs/roadmaps/g14/README.md` (fixed inputs — do not re-decide)

## Goal

Promote the g13 verdict's durable outcomes and the pinning doctrine into
stable architecture and working rules, and write the normative specs for
the pinning line. Docs only — no code, no gates, no behaviour. No g14
execution card runs before this merges.

## Fixed By Ruling (recorded — do not re-decide)

- The g13.020 verdict: authority retired as a corpus mechanism, three
  pilot slices unwound, scene system kept, drift-gate estate is the
  corpus-wide authority.
- g14 fixed inputs: no revival of cross-language codegen, no expression
  vocabulary, no evaluator. Generated machine *interfaces* are the only
  generated surface. Svelte is the reference implementation.
- Pinning mechanisms, and nothing else: generated machine interfaces,
  differential machine traces, vector completeness gating, capability
  absence registry, specimen evidence gates.

## Deliverables

- Architecture 006 amended: the machine model gains the pinning contract —
  machine interfaces generated from one schema, differential traces as the
  cross-pair equivalence check, vector completeness as a gate. The
  dual-layer shape and the pure-machine model stay as they are.
- Architecture 001 amended: pair-wise authority (`poodle-core` for web,
  `poodle-render` for natives) stated as the durable shape; the
  cross-language-authority ambition recorded as closed, citing the
  verdict.
- Working rules 001 amended: capability absence is declared, never
  silent; a native hole is a failing gate, not a sweep finding. The
  §Shared Implementation and §Runtime Parity Authority sections are the
  places this lands.
- Spec `064-cross-runtime-machine-pinning.md` written: scope, mechanisms,
  boundaries, stop conditions. Normative, not provisional.
- Spec 063 split: the scene half promotes to
  `065-scene-authoring-and-specimen-fixtures.md` (fixture authority, four
  runtimes, no evaluator, no application framework); the component half
  retires. Spec 063's status records the retirement and points at both
  successors.
- `docs/specs/README.md` index updated for the new spec set.

## Acceptance

- [ ] Architecture 001/006 and working rules 001 amended; nothing
  provisional promoted into them.
- [ ] Spec 064 active; spec 065 active; spec 063 retired with the verdict
  cited.
- [ ] The boundary is stated so a future card cannot read around it:
  interfaces in, behaviour out, no evaluator, no expression vocabulary.
- [ ] `effigy docs:lint` and `git diff --check` pass.
- [ ] No code, task, or gate file touched.

## Stop Conditions

- An architecture doc already contradicts a fixed input — report the
  exact passage instead of silently amending around it.
- Promoting a point would require inventing behaviour the verdict did not
  record.

## Writable Paths

- `docs/architecture/001-poodle-system-shape.md`
- `docs/architecture/006-headless-core-and-machine-model.md`
- `docs/contracts/001-working-rules.md`
- `docs/specs/063-rust-authored-component-and-scene-ir.md`
- `docs/specs/064-cross-runtime-machine-pinning.md` (new)
- `docs/specs/065-scene-authoring-and-specimen-fixtures.md` (new)
- `docs/specs/README.md`
- `docs/logs/2026-08/14-g14-001-record-verdict-and-pinning-doctrine.md`
- `PAPERCUTS.md` (append only — see below)

## Steps

1. Baseline: `effigy docs:lint`, `git diff --check`. Green.
2. Read the governing refs in full, including the recorded `g13.020`
   verdict and the `g13.008` evidence.
3. Amend architecture 006, then 001, then working rules 001. Each change
   minimal and citing the verdict.
4. Write spec 064 from the g14 roadmap's mechanisms, not invented ones.
5. Split spec 063: promote the scene half into 065, retire the component
   half, record the retirement.
6. Update the specs index.
7. Validate: `effigy docs:lint`, `git diff --check`.
8. Write the batch log (glue-light, following the house style in
   `docs/logs/`): what each doc gained, what was retired, where the
   boundary is stated.

## Worker Rules

- Docs only. If a change needs code, it is out of scope — say so.
- Do not touch `generation-index.md` or `roadmaps/README.md` — front-door
  rollover is the orchestrator's.
- `PAPERCUTS.md` is shared append-only; append only if you hit a real
  hurdle, never reflow neighbours.
- Stage only writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g14-001-record-verdict-and-pinning-doctrine`.
  Do not merge.
