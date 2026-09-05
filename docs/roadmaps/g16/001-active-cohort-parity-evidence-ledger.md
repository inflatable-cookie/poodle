# g16.001 — Active-Cohort Parity Evidence Ledger

Status: complete — operator-reviewed and merged in PR #75
Completed: 2026-08-25
Depends on: completed `g15`, published `v0.2.2`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`../../specs/025-parity-automation-and-harness-boundary.md`,
`../g15/release-baseline-roster.md`, `../g14/conformance-estate.md`

## Outcome

Replace Poodle's contradictory parity status surfaces with one current,
component-level evidence ledger for the active cohort: Svelte, React, shared
Rust composition, and GPUI. Record Jetstream once as program-deferred.

The ledger reports what the repository proves today. It does not declare
runtime parity from implementation presence, specimen construction, a borrowed
test, or another runtime's pass.

This is evidence recovery, not a third conformance architecture. It makes the
next runway measurable; it does not choose or build that runway.

## Problem

The g15 release roster closed structural coverage, focused web evidence,
native declarations, shared rendering, GPUI specimens, selected mounted
interactions, and one Button visual comparison. Those are different claim
levels.

Current reports blur them:

- `packages/gpui/cross-runtime-parity-report.json` identifies itself as
  `g09.018`, counts 96 components, and says mounted GPUI route evidence is
  missing;
- `packages/gpui/native-accessibility-proof.json` is a `g04.010` manual
  baseline that predates g14/g15 mounted evidence;
- `packages/jetstream/cross-runtime-parity-report.json` is a `g10.014`
  117-component report whose completion language conflicts with program
  deferral;
- generated Svelte and React parity reports embed the stale GPUI report;
- `g15/release-gap-register.md` still describes the retired fork-backed
  offscreen capture path, superseded by `g15.059`;
- the open Longhorn conformance-lab note assumes a true headless GPUI pixel
  sidecar that crates.io GPUI does not provide.

No current surface answers the basic question: which claims are structural,
which have focused behaviour evidence, and which are actually compared across
runtimes?

## Fixed Denominator And Runtime Posture

- Denominator: the current public Svelte component roster. It must resolve to
  175 components unless source has intentionally changed since g15.
- Portable native denominator: every component except the contract-approved
  web-only `MeterSurface`; expected count 174.
- Svelte is the semantic reference.
- React, shared Rust, and GPUI receive independent evidence cells. One runtime
  never borrows another runtime's result.
- Jetstream is one program-level `deferred` target. Shared specs, render nodes,
  or adapter compilation do not make its backend pass.
- Historical `docs/archive/parity/` audits and rejected g13/g14 evidence
  remain historical. Do not rewrite them as current results.

If the measured roster differs, stop before changing the denominator. Report
the source change and required planning decision.

## Evidence Model

Create one current ledger under this generation, backed by a deterministic
checker. Each component row records these claims separately:

| Claim | Required evidence |
| --- | --- |
| Contract | exact component contract path |
| Svelte surface | implementation, public export, specimen, focused test |
| React surface | implementation, public export, specimen, focused test |
| Shared Rust surface | exact spec and `poodle-render` path, or approved `n/a` |
| GPUI construction | exact specimen/route and current 174-route probe result |
| GPUI mounted behaviour | exact test path and test name; absent stays absent |
| Web accessibility | exact automated or manual evidence; Svelte axe does not transfer to React |
| GPUI accessibility | exact semantics/focus/keyboard/announcement evidence; spec-only and mounted proof remain distinct |
| Web visual | exact Svelte↔React fixture/sweep evidence and execution posture |
| GPUI visual | exact compared fixture set and capture posture; Button-only remains Button-only |
| Known deltas | contract/register reference, status, and runtime reason |

Use a small closed vocabulary that cannot imply more than it proves. At
minimum distinguish:

- `present` — implementation or structural surface exists;
- `focused` — named owner-local test proves a bounded claim;
- `mounted` — named test drives the real runtime tree;
- `compared` — named cross-runtime comparison and denominator exist;
- `manual` — review is required and no automated pass is claimed;
- `missing` — required active-cohort evidence is absent;
- `not-applicable` — contract-approved exclusion;
- `deferred` — program-level target is outside the active cohort.

Do not collapse these into one `complete` flag.

## Delivery

### 1. Build the current ledger

- Add `docs/roadmaps/g16/parity-evidence-ledger.md` with summary counts,
  evidence definitions, one row per component, global Jetstream posture, and
  an explicit limitations section.
- Derive component identity and structural paths from live source/catalogue
  authorities. Do not duplicate a hand-maintained denominator.
- Add the smallest deterministic checker under `scripts/` or the existing
  preview audit surface. It must fail on missing, extra, or duplicate rows and
  unresolved paths/test names.
- Explicit evidence claims may be hand-maintained where semantics cannot be
  inferred safely. The checker still derives the roster and proves every row
  participates; no manually registered component may disappear silently.
- Summary counts derive from ledger rows. Do not hand-maintain totals.

### 2. Repair active report truth

- Update or retire active parity/accessibility artifacts reachable from
  `report:parity`, `report:accessibility`, `docs:check`, and current docs front
  doors when they contradict the ledger.
- Generated artifacts remain generated. Change their source or generator; do
  not hand-edit output only.
- Remove g09/g10 component-count and mounted-preview claims from current
  reports.
- Make GPUI evidence say exactly:
  - 174/174 portable specimen routes construct headlessly;
  - mounted behaviour is the bounded named regression set, not the roster;
  - the accepted three-runtime visual inventory is Button-only;
  - current GPUI pixels require the operator-approved, non-activating windowed
    diagnostic and are absent from default QA/CI;
  - broad native accessibility and assistive-technology parity are not proved.
- Make Jetstream evidence say exactly:
  - shared Rust composition and the in-repo adapter remain maintained;
  - the sibling converter, input, accessibility, preview, and visual programme
    is deferred;
  - no historical full/partial component count is a current parity result.

### 3. Repair current planning references

- Correct the stale GPUI offscreen-capture row in
  `g15/release-gap-register.md` with an explicit supersession note pointing to
  `g15.059`–`g15.061`. Preserve the historical g15 result.
- Update the Longhorn conformance-lab triage note: webviews remain controllable
  without focus; crates.io GPUI currently offers a non-activating windowed
  diagnostic, not true headless pixels. Keep the lab open unless this card's
  evidence resolves its ownership question.
- Update task comments or current front doors only where old g16 numbering or
  old capture language would misstate the shipped boundary.

### 4. Return the measured next decision

- Add one closeout section grouping components by the next missing evidence
  class: semantic/interface, mounted behaviour, accessibility, web visual, or
  GPUI visual.
- Identify candidate primitive families from measured gaps only.
- Do not create `g16.002`, a fixture schema, runtime adapters, a comparator, or
  a rollout plan. The orchestrator and operator choose the next lane after
  reviewing the ledger.

## Closeout — 2026-08-25

The live denominator stayed at 175 public Svelte components and 174 portable
native components. `MeterSurface` is the single explicit native
`not-applicable` row. The checked-in ledger, generated reports, and
`effigy check:parity-evidence-ledger` now share that denominator.

Measured evidence classes:

- Semantic/interface: Svelte and React each have 175 focused surface rows;
  shared Rust has 174 present rows and one native exclusion; GPUI has 174
  focused construction routes and one native exclusion.
- Mounted behaviour: 29 component rows carry mounted evidence across 33 named
  regression tests; 145 rows remain honestly missing and one is not applicable.
- Accessibility: the Svelte axe sweep covers 175 rows; React has no equivalent
  axe sweep; GPUI accessibility is manual for 174 portable rows and does not
  claim broad native or assistive-technology parity.
- Visual: the web inventory has one compared Button row, 169 focused rows, and
  five manual skips. GPUI comparison is also Button-only with 18 fixtures and
  operator-approved non-activating windowed capture; it is absent from default
  QA/CI.
- Jetstream remains program-level `deferred`; shared Rust and the in-repo
  adapter do not admit its backend.

Validation:

- `effigy test:parity-evidence-ledger`: 4 tests passed.
- `effigy report:parity`, `effigy report:accessibility`, `effigy docs:lint`,
  `effigy docs:check`, and `effigy check:parity-evidence-ledger`: passed.
- `effigy probe:gpui-specimens`: passed; `effigy regressions:native`: 70
  passed; `effigy ci:web`, `effigy ci:native`, and `effigy qa`: passed.
- `effigy test:visual-fixtures`: passed after the bounded authority-path test
  repair; the test now reads and sanctions the live `window_capture` loader.
  The fixture inventory and thresholds were unchanged.
- `git diff --check`: passed.

No `g16.002` is compiled. The next lane requires operator review of the
measured missing cells.

## Acceptance

- [x] The ledger contains exactly the live public component denominator once,
      with the one approved native `not-applicable` component explicit.
- [x] Every evidence cell names a resolvable source, test, artifact, or an
      honest `missing` / `manual` / `not-applicable` posture.
- [x] Summary counts are derived and a planted missing, duplicate, extra, or
      unresolved evidence reference fails the checker.
- [x] No report claims all-component behavioural, accessibility, or visual
      parity from roster presence or specimen construction.
- [x] Current web reports no longer embed the g09 GPUI 96-component baseline.
- [x] Current Jetstream reports say program-deferred and do not carry the old
      117-component parity result as current truth.
- [x] GPUI reporting distinguishes 174-route headless construction, bounded
      mounted regressions, Button-only comparison, and windowed pixel capture.
- [x] The g15 capture ledger and Longhorn lab note agree with the crates.io
      GPUI boundary shipped in v0.2.2.
- [x] No component contract, public API, runtime behaviour, specimen content,
      visual threshold, or workflow changes.
- [x] One August execution log records inventory method, artifact dispositions,
      before/after claims, validation, and unresolved evidence gaps.

## Writable Scope

- `docs/roadmaps/g16/parity-evidence-ledger.md`
- this card, `docs/roadmaps/g16/README.md`, generation/front-door status, and
  one August execution log
- the smallest deterministic evidence-ledger checker and its focused tests
- current parity/accessibility report sources, generators, and generated
  artifacts proven stale by this card
- `docs/roadmaps/g15/release-gap-register.md` for the bounded capture
  supersession correction
- `docs/roadmaps/g16/visual-lab-unblock-runway.md` for the shipped GPUI
  capture boundary
- `tasks/effigy.tasks.toml` only to expose the checker or correct stale task
  comments; no selector semantics may be weakened
- `PAPERCUTS.md` for new execution friction only

Do not edit component contracts, package APIs, component/runtime
implementations, specimens, visual fixtures or thresholds, release/version
surfaces, downstream repositories, `.github/workflows/`, or the sibling
Jetstream repository.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused tests for the ledger checker and changed report generators;
- `effigy probe:gpui-specimens`;
- `effigy regressions:native`;
- `effigy test:visual-fixtures`;
- `effigy docs:check`;
- `effigy ci:web`;
- `effigy ci:native`;
- `effigy qa` once after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything must remain headless. Do not run `*-windowed`, native visual,
Jetstream preview/QA, or release selectors.

## Stop Conditions

- The live denominator differs from 175/174 without an already-authorised
  component change.
- Producing the ledger requires a universal component schema, shared case
  corpus, normalized observation model, generated runtime adapter, or new
  component authority.
- A status can only be made green by inferring evidence from another runtime,
  weakening a gate, hiding a component, or treating a specimen as behaviour
  proof.
- Current evidence contradicts a component contract or exposes a runtime bug.
  Record the exact gap and return it for a bounded repair card; do not absorb
  the repair here.
- Repair requires component/runtime code, a workflow edit, a windowed run, a
  sibling Jetstream checkout, or a release mutation.

## Continuation

The completed ledger and evidence-class summary were operator-reviewed with PR
#75. The operator and orchestrator now choose the first primitive family and
evidence layer before compiling `g16.002`; no later card is implied yet.
