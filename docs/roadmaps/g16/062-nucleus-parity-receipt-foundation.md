# g16.062 — Nucleus Parity Receipt Foundation

Status: complete — merged in PR #170 at `d88a60e27`
Type: evidence infrastructure
Opened: 2026-09-02
Depends on: promoted Nucleus parity programme
Governing refs: `nucleus-gpui-parity-programme.md`,
`001-active-cohort-parity-evidence-ledger.md`,
`parity-evidence-ledger.md`, `../../contracts/001-working-rules.md`

## Goal

Freeze the 29-row Nucleus cohort manifest and make mounted parity claims depend
on receipts emitted by executed selectors. Preserve the current expected-test
map as planning input; stop treating a mapped test name as proof that it ran.

## Fixed Boundary

- One versioned manifest records the 29 rendered components, `IconProvider`
  prerequisite, scenario IDs, direct dependencies, and package/commit/lock
  resolution fields.
- One versioned execution receipt records component, scenario, proof level,
  runtime, command, exact commit, production-path observation, actions,
  assertions, outcome, and artifact paths.
- The actual runner emits receipts only after a successful observed mounted
  execution. Static imports, test names, specimen routes, and source references
  cannot emit receipts.
- The ledger checker compares expected entries with validated receipts.
  Expected-without-receipt remains missing; unmanifested receipts are review
  findings.
- `M1`, `A1`, and `V1` stay distinct. This card does not create `A2`, `M2`, or
  `V2`, run Nucleus, or change the existing Button visual denominator.

## Ordered Work

1. Add the fixed cohort manifest and schema validation.
2. Add receipt emit/validate support to the real mounted selector path.
3. Rename or classify `MOUNTED_BEHAVIOUR_TESTS` as expected coverage and make
   generated mounted status consume validated receipts.
4. Add counterexamples for a mapped-but-unrun test, an unmanifested receipt,
   wrong commit/runtime, direct-handler proof, and proof-level substitution.
5. Regenerate the ledger without silently upgrading any row. Record the old
   expected-map counts separately for traceability.

## Acceptance

- A successful real mounted run emits a deterministic receipt tied to exact
  source and observed production-path execution.
- A test name with no run cannot produce `mounted` status.
- A receipt with the wrong component, scenario, runtime, commit, or artifact
  fails validation.
- `A1` and `V1` cannot be inferred from `M1`.
- The fixed manifest reports 29 rendered targets plus one separate
  `IconProvider` prerequisite.
- Existing historical evidence remains visible without being upgraded.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Execution, not naming | add a plausible test name without running it | row remains missing |
| Production path | direct-call the component handler | receipt validation rejects |
| Exact identity | reuse receipt after source commit changes | commit check rejects |
| Proof levels stay separate | label M1 receipt as A1/V1 | schema/checker rejects |
| Cohort is fixed | include `IconProvider` as row 30 | denominator check fails |

## Writable Scope

Parity ledger generator/checker, mounted runner receipt support, fixed Nucleus
manifest and schemas, focused tests/fixtures, this card, one execution log,
generated ledger/front-door rows, and new papercuts. Do not edit component
behavior, Nucleus, lab code, accessibility authority, workflows, releases,
versions, Jetstream, or run windowed/native-visual selectors.

## Validation

Run focused receipt/schema tests, the relevant real headless mounted selector,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `effigy qa`, and
`git diff --check origin/main...HEAD`.

## Stop Conditions

Stop if the existing runner cannot prove production-path observation without a
new architecture decision, if receipts need product data, or if the work would
claim accessibility, visuals, or Nucleus adoption from mounted evidence.

## Continuation

Evidence log: `docs/logs/2026-09/20260902-g16-062-nucleus-parity-receipt-foundation.md`.

Compile small NP-1 through NP-5 cards from validated gaps. Do not dispatch a
tranche as one batch. `g16.066` launches first under the recorded native
harness merge order; the Nucleus child cards remain independently promotable.
