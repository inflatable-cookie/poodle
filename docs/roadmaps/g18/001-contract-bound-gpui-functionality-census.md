# 001 — Contract-bound GPUI functionality census

Status: complete
Owner: Poodle core
Created: 2026-09-09
Depends on: none
Governing refs: `README.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/003-native-accessibility.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`../../specs/025-parity-automation-and-harness-boundary.md`,
`../../evidence/nucleus/parity-evidence-ledger.md`

## Outcome

Publish a reproducible capability-level census for all 175 portable GPUI
components. Each row says which contracted behaviour is proved by mounted GPUI
execution, which is missing, and which is held by a current platform boundary.
Existing evidence is admitted only when the executed scenario proves the named
claim.

The result becomes the sole compilation input for bounded g18 repair tranches.
It does not claim that 175 constructing routes, 57 expected-test components, or
one passing test makes the catalogue fully functional.

## Ready-State Rubric

- [x] The denominator and present evidence gaps are generated and current.
- [x] Contract and evidence authority are explicit.
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] Overclaim, stale-test, and unsupported-exception cases have review oracles.
- [x] The task selects no product behaviour or new public API.
- [x] No successor auto-starts; Chatterbox compiles repair work from the result.

## Decisions

- The census is capability-level. At minimum it separates semantic inputs and
  state, emitted events and timing, pointer interaction, keyboard and focus,
  accessibility projection, and visual/token/layout evidence when the contract
  requires each axis.
- A mounted receipt names the exact component, contract claims, scenario,
  selector, test, source commit, dependency resolution, and observed outcome.
- Existing Nucleus M1/A1/V1 receipts remain valid and unchanged.
- The 44 non-Nucleus rows currently carrying expected-only tests are candidates
  for admission, not automatic passes. The implementation must execute and
  inspect each named test before emitting evidence.
- Platform A2 is distinct from Poodle node-tree semantics. The current upstream
  GPUI publication hold cannot erase component-level keyboard, focus, role,
  state, or label gaps that Poodle can prove below the platform tree.
- No shared executable case corpus or generated portable component interface is
  introduced; the rejected g14 architecture stays rejected.

## Dispatch manifest

- **State:** ready; one serial lane; no concurrent sibling or automatic successor.
- **Completion:** a checked-in schema, generator/checker, generated census, and
  validated receipts for every existing execution claim that survives the
  oracle; all unproved capabilities remain explicit gaps; the roadmap summary
  counts can be regenerated from the artifact.
- **Owned mutable paths:** `scripts/` census/receipt generator and focused tests;
  `docs/evidence/gpui/`; `docs/evidence/nucleus/` generator integration only;
  `packages/gpui/cross-runtime-parity-report.json` and its generator/checker;
  `packages/gpui/preview/tests/headless_regressions.rs` only for receipt emission
  and planted oracle cases; Effigy manifest entries required for the new check;
  one execution log; `PAPERCUTS.md` append-only.
- **Reserved closeout surfaces:** `docs/roadmaps/g18/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`, and this
  task's final status/evidence block.
- **Worker:** high-reasoning implementation worker; evidence schema, Rust/TypeScript
  harness integration, and adversarial completeness review.
- **Excluded:** component API changes; speculative component repairs; web visual
  repair; windowed capture; A2 platform proof; Nucleus application journeys;
  Jetstream admission; release work; accepting observed deltas.
- **Escalation:** Chatterbox owns evidence-policy or component-grouping decisions;
  operator owns any new conformance architecture or windowed execution.

## Work

1. Derive the exact 176 public / 175 portable denominator from current catalogue
   authority and refuse missing, extra, or duplicate rows.
2. Review each portable component contract and record its required claims in
   the closed capability axes above, retaining exact contract references and
   explicit not-applicable reasons. Keep this manifest machine-readable and
   checker-enforced; do not infer semantics from heading text alone.
3. Define a closed mounted-receipt schema and deterministic generator/checker.
   Reuse Nucleus receipts without rewriting them.
4. Execute and inspect the retained expected-test map. Admit a receipt only when
   the named test exists, ran on the exact source/dependency identity, mounts the
   production renderer and GPUI node backend, and observes the claimed capability.
5. Generate the census and cross-runtime summary from admitted receipts. Keep
   unproved claims missing even when source, a route, or a test name exists.
6. Group missing capabilities by shared substrate and dependency so Chatterbox
   can compile the smallest high-leverage repair tranche without embedding a
   second roadmap in the evidence artifact.
7. Record counts, refusals, exact validation, and limitations in one execution log.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Denominator is complete | one catalogue component omitted or duplicated | generator/check fails closed |
| Evidence is executed | a retained expected test is renamed, ignored, or never run | no receipt; census remains missing |
| Evidence is claim-bound | a pointer-only test marks keyboard/focus complete | schema/check rejects the unsupported capability |
| Runtime path is real | a renderer unit test bypasses the GPUI node backend | receipt admission fails |
| Holds stay narrow | the A2 platform gate marks keyboard or node semantics not-applicable | checker rejects the widened exception |
| Existing evidence survives | generalization rewrites or weakens Nucleus M1/A1/V1 | existing receipt tests and byte comparison fail |
| Report cannot overclaim | construction count is presented as functional completion | generated-report assertion fails |
| Rejected architecture stays rejected | implementation adds a shared portable case corpus or generated component API | exact diff review rejects the change |

## Stop conditions

- Stop if contract language cannot be mapped to the closed axes without a new
  semantic or policy decision.
- Stop rather than treating source inspection, test names, or another runtime's
  result as mounted GPUI evidence.
- Stop if receipt emission would require weakening existing Nucleus validation.
- Stop if the work expands into component behaviour repair; report the exact
  failing rows for Chatterbox tranche compilation.
- Do not run a windowed selector without fresh operator approval.

## Evidence

Starting posture: 175/175 construction; 29/175 receipt-backed mounted rows;
146 missing mounted rows; 44 of the missing rows carry retained expected-test
entries and 102 do not. Final evidence must record the reviewed exact head, PR,
merge, admitted receipt count, remaining capability counts, refusals, and the
validation actually run.

## Shipped result (merged)

PR #235 merged as `8185a9758f146e901499a8a8704a41202f99ca9e` on 2026-09-09 after independent exact-head review of `e88cd75c9e2b68fd47e597180d20c6736a80bc31` ([review comment #5605510026](https://github.com/inflatable-cookie/poodle/pull/235#issuecomment-5605510026)).
Census: 176 public / 175 portable rows; 73/175 rows with at least one admitted capability, 24/175 fully admitted; 65 new mounted receipts (`poodle.g18-gpui-mounted-receipt.v1`); 224 in-census refusals; 11 missing-capability substrate groups for tranche compilation. Nucleus M1/A1/V1 receipts unchanged and byte-identical.
Validation on merged main `8185a9758f146e901499a8a8704a41202f99ca9e`: `bun scripts/gpui-functionality-census.ts --check` match, census tests 17 pass / 0 fail, ledger tests 9 pass / 0 fail, nucleus-parity-receipts tests 17 pass / 0 fail, `effigy docs:lint` pass, `git diff --check` clean. No windowed selector was run. No Jetstream claim admitted.

## Next task

Return to Chatterbox. Compile the first bounded functional repair tranche from
the accepted missing-capability groups; do not auto-start it.
