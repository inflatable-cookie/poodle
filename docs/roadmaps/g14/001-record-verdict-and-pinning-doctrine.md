# g14.001 Record The Verdict And Pinning Doctrine In Architecture

Status: planned
Owner: Poodle maintainer
Depends on: g13 closeout, `g13.020` verdict recorded
Governing refs: `../g13/pilot-verdict-evidence.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../architecture/006-headless-core-and-machine-model.md`,
`../../contracts/001-working-rules.md`

## Objective

Promote the g13 verdict's durable outcomes and the pinning doctrine into
stable architecture and working rules, and write the normative spec for the
pinning line. No g14 execution card runs before this.

## Deliverables

- Architecture 006 amended: the machine model gains the pinning contract —
  machine interfaces generated from one schema, differential traces as the
  cross-pair equivalence check, vector completeness as a gate.
- Architecture 001 amended: pair-wise authority (`poodle-core` for web,
  `poodle-render` for natives) stated as the durable shape; the
  cross-language-authority ambition recorded as closed.
- Working rules 001 amended: capability absence is declared, never silent;
  a native hole is a failing gate, not a sweep finding.
- Spec `064-cross-runtime-machine-pinning.md` written: scope, mechanisms,
  boundaries, stop conditions.
- Spec 063 retired or folded: vocabulary-scope outcome recorded per
  `g13.020`, including the disposition of `poodle-ir`/`poodle-codegen`.

## Acceptance

- [ ] Architecture 001/006 and working rules 001 amended; nothing
  provisional promoted into them.
- [ ] Spec 064 active; spec 063 retired or folded with the verdict
  recorded.
- [ ] The boundary is stated so a future card cannot read around it:
  interfaces in, behaviour out, no evaluator, no expression vocabulary.
- [ ] `docs:lint` and `git diff --check` pass.

## Next

`g14.002` freezes the baseline the pinning line is measured against.
