# g16.020 — Component Continuation Audit

Status: complete — audit register delivered; board baselines recorded
Opened: 2026-08-28
Depends on: completed `g16.001`, merged `g16.018`; does not depend on
`g16.019` merge
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`../../specs/025-parity-automation-and-harness-boundary.md`,
`parity-evidence-ledger.md`

## Goal

Catalog the component work completed through g15 and g16, account for the
remaining public Svelte roster, and return a dependable continuation map for
component delivery alongside the separate drag-and-drop programme.

This is a read-only audit of implementation and evidence. It creates no
component fixes, parity claims, conformance framework, or ready implementation
cards.

## Why This Lane Exists

The release and g16 mounted-evidence work landed many component repairs in a
long serial run. The evidence ledger says which claims exist, but it does not
answer three planning questions:

- which explicit non-claims and deferred component gaps remain after the work;
- which gaps share a substrate or interface decision and should be grouped;
  and
- which components are genuinely ready for bounded follow-up rather than
  merely missing a mounted, accessibility, or visual evidence cell.

Without a continuation audit, the next sequence risks returning to
one-component/one-cell selection, repeating completed work, or mixing the new
drag-and-drop substrate into unrelated component cards.

## Fixed Denominator

- Start from the live public Svelte roster and the checked g16 parity ledger.
- Expected denominator: 175 public Svelte components, 174 portable native
  components, with MeterSurface the current native `not-applicable` row.
- Treat Jetstream once as program-deferred. Do not create 175 per-component
  Jetstream tasks.
- If the live denominator differs, stop and report the source change. Do not
  silently rewrite this audit, the ledger, or the expected count.

PR #94 / `g16.019` is an active external input. Main remains authoritative:

- if PR #94 is unmerged at audit closeout, record Select as an active
  changes-requested lane and keep main's ledger totals;
- if it merges before closeout, update the audit from the merged result; and
- never copy PR #94's provisional 47 / 127 totals into current evidence while
  main still reports 46 / 128.

## Audit Questions

For every component, determine:

1. What g15/g16 work materially changed or certified it?
2. What current contract, implementation, specimen, focused test, mounted
   behavior, accessibility, and visual evidence does the live ledger name?
3. What explicit non-claims, stop conditions, triage notes, consumer feedback,
   or current contract gaps remain?
4. Is there a known implementation task, an evidence-only gap, a blocked
   decision, a cross-cutting programme dependency, or no currently identified
   work?
5. If work is known, what substrate and neighboring components should share a
   bounded delivery lane?

Do not infer a defect from a missing evidence cell. `missing mounted proof`,
`known behavior gap`, and `not audited deeply enough` are different results.

## Required Classifications

Create `component-continuation-register.md` under this generation. Account for
every live component exactly once with compact fields equivalent to:

| Field | Meaning |
| --- | --- |
| Component | canonical public roster name |
| Family | current catalogue/contract family |
| Recent work | exact g15/g16 card, PR, or `none identified` |
| Current evidence | pointer to the live ledger row, not copied cells |
| Continuation class | closed, evidence-only, known repair, decision-blocked, programme-owned, or unknown |
| Remaining work | terse fact with exact source, or `none currently identified` |
| Dependency | shared substrate/decision/programme, when any |
| Candidate lane | bounded family/lane name, not a ready card |
| Confidence | observed, inferred, or unknown |

The register may use grouped tables or a checked generated view, but every
component must be mechanically accounted for once. Do not build another
component schema or duplicate the ledger's evidence matrix.

## Historical Reconciliation

Read the current g15 and g16 roadmaps and August logs. Extract:

- completed component/interface/specimen work that should not reappear;
- explicit non-claims that remain relevant;
- review findings that were fixed versus deliberately deferred;
- open triage decisions such as NumberInput's native value model;
- consumer-reported gaps already resolved on main; and
- cross-cutting work that belongs to an existing authority rather than a new
  component card.

Historical claims never override live contracts or source. When a log and the
current implementation disagree, record the drift and stop before classifying
it as ready work.

## Parallel Programme Boundary

Architecture 011 and spec 069 govern dependable drag-and-drop. Classify these
components as `programme-owned` where their remaining work is migration to that
substrate:

- Tabs;
- EditableList;
- Tree;
- ModelCatalogueEditor;
- OrderBy;
- BlockEditor; and
- DockRegion.

The audit may identify non-drag work in those components separately. It must
not design another drag system, modify architecture 011/spec 069, or scatter
their migration across unrelated candidate lanes.

Other cross-cutting programmes remain distinct, including:

- editing value-model decisions for NumberInput and related controls;
- broad React accessibility evidence;
- GPUI accessibility;
- Svelte/React and native visual comparison;
- motion/transition research; and
- deferred Jetstream admission.

## Candidate Continuation Map

After the roster register, produce a small family-level continuation map. Each
candidate lane must state:

- component set;
- exact observed gap;
- shared dependency or governing contract;
- whether operator intent or a breaking pre-1.0 decision is required;
- expected evidence level gained;
- likely file-overlap boundary with other lanes;
- why it is a coherent batch; and
- why it is not ready, or what would make it ready.

Return no more than eight candidate lanes. Rank by leverage and dependency,
not by the number of missing ledger cells. Identify at least:

- the next component foundation candidate independent of drag-and-drop;
- the next decision-blocked editing candidate;
- the drag-and-drop migration family as one programme-owned lane;
- accessibility and visual evidence as separate programme choices; and
- components with no currently identified implementation work.

The orchestrator will turn accepted candidates into roadmap cards. The audit
worker does not mark them ready or choose the next implementation.

## Writable Scope

- this card;
- `component-continuation-register.md` under `docs/roadmaps/g16/`;
- one August audit log under `docs/logs/2026-08/`; and
- `PAPERCUTS.md` only for new execution friction.

Do not edit:

- `docs/roadmaps/g16/README.md`, `docs/roadmaps/README.md`, or the generation
  index; the orchestrator owns front-door reconciliation with PR #94;
- the parity evidence ledger, its checker, or generated reports;
- architecture 011 or spec 069;
- component contracts, implementations, specimens, tests, runtime adapters,
  tokens, package/public APIs, workflows, releases, or downstream repositories;
  or
- sibling Longhorn, Loophole, Underlay, or Jetstream repositories.

## Method

Use deterministic inventory before prose:

1. derive the live roster and ledger rows;
2. index g15/g16 cards and logs by named component;
3. index explicit non-claims and open triage references;
4. inspect live contracts/source only to resolve a named contradiction or
   classify a candidate; do not perform an unbounded quality audit of every
   implementation file;
5. account for every component once; and
6. derive the family-level continuation map from the register.

Use `rg` for exact component/card references and Effigy graph queries for
ownership or impact questions. Separate observed facts from inference. Mark
unknown rather than filling a gap with plausible work.

## Acceptance

- [x] Exactly the live public Svelte roster is accounted for once, with the
      portable native denominator and MeterSurface exclusion explicit.
- [x] Every component points to the live ledger rather than copying its
      evidence cells into a competing matrix.
- [x] Completed g15/g16 work, still-valid non-claims, open decisions, and
      programme dependencies are distinguished.
- [x] Missing evidence is not mislabeled as an implementation defect.
- [x] Drag-dependent components point to architecture 011/spec 069 as one
      programme; no second drag-and-drop design appears.
- [x] The continuation map contains no more than eight coherent candidate
      lanes with dependency, authority, evidence, and readiness stated.
- [x] Select and ledger totals reflect merged main, with unmerged PR #94
      described only as pending external work.
- [x] No implementation, contract, specimen, ledger, generated report,
      workflow, release, or sibling-repository file changes.
- [x] One August log records inventory method, counts, unresolved unknowns,
      candidate lanes, validation, and explicit non-claims.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- deterministic roster/accounting check chosen by the worker without adding a
  new permanent schema or checker;
- `effigy check:parity-evidence-ledger`;
- `effigy docs:lint`;
- `effigy docs:check` once after the coherent audit batch;
- `effigy qa` once at final handoff; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The live component denominator differs from 175/174.
- A classification requires changing a contract, resolving operator intent,
  or deciding a breaking migration.
- A historical claim contradicts live source and cannot be classified as
  drift without deeper implementation work.
- The audit would need a new universal component schema, generated runtime
  adapter, conformance corpus, or competing evidence authority.
- A finding needs implementation repair. Record the exact candidate and leave
  it for orchestrator promotion.
- PR #94 changes the roster, authority, or audit method rather than only
  Select evidence and closeout state.

## Worker Closeout

The audit output is `component-continuation-register.md`; the evidence and
method log is `../../logs/2026-08/20260828-g16-020-component-continuation-audit.md`.
The register proves 175 Svelte exports, 174 portable native components, one
MeterSurface native exclusion, and 175 unique ledger rows. It keeps the live
46 / 128 mounted total and records PR #94 only as pending external work.

The handoff's literal base SHA is invalid; short `69118d831` resolves to
`69118d83122e976d256af6033e57d1c8e6b1a9de`, the actual ancestor of this
worktree's HEAD. The friction is recorded in `PAPERCUTS.md` and the audit log.

Validation passed for the deterministic roster proof, parity-evidence ledger,
and docs lint. `effigy docs:check` stopped at the existing React preview token
package-alias failure; final `effigy qa` stopped at the existing
lucide-static 1.35.0 versus manifest 1.31.0 mismatch. Both failures are
recorded in the audit log and existing root papercuts. The final committed
range passed `git diff --check origin/main...HEAD`.

## Continuation

Return the completed register, candidate continuation map, audit log, exact
validation, and PR URL to the orchestrator. Do not merge. The orchestrator
reconciles the result with PR #94, the drag-and-drop programme, and the g16
front doors before selecting implementation cards.
