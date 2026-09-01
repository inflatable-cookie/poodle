# Contributor Design-Guidance Pilot Decision

Status: ready for orchestrator review; implementation held
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Source: `docs/handoffs/20260901-121256-design-guidance-pilot-planning.md`
Evidence: `docs/research/value-tracks/agent-facing-design-guidance-and-evaluation.md`
Promotion owner: Poodle Northstar orchestrator

This packet records the operator-approved shape of one finite manual pilot. It
is planning evidence, not authority to create guidance, prompts, fixtures,
generated pages, evaluation tooling, selectors, public documentation, or an
implementation card.

## Settled Decisions

### Candidate, audience, and ownership

- Test one condition only: a compact contributor-local router for agents
  working inside the Poodle repository. Human contributors may read it;
  consumers are outside its audience and authority.
- The router exposes three task routes:
  - `frame` — user job, product boundary, composition authority, and specimens;
  - `build` — canonical components, tokens, shared mechanics, and runtime docs;
  - `check` — separate semantic, accessibility, mechanical, and composition
    review.
- The router links to existing Poodle authority and narrow Effigy checks. It
  adds no independent design rules and does not restate contracts, token
  tables, or mechanics.
- Poodle core owns guidance maintenance.
- The Poodle Northstar orchestrator owns the pilot brief, evidence record,
  randomized condition mapping, and promotion verdict.
- Two blind human reviewers score the outputs.
- A consumer-facing `design.md` and a permanent or productized evaluation
  harness remain held.

### Scenario and trial shape

- Target Svelte only. The pilot makes no React, GPUI, active-cohort, renderer
  parity, or cross-runtime claim.
- Freeze three in-set scenarios:
  - form and validation;
  - browse and detail;
  - reusable workstation shell.
- Use a picker surface as the hidden transfer scenario. Keep its exact prompt
  and fixtures out of router authoring; reveal them only after the router
  digest is frozen.
- Run two independent trials per condition and scenario: 16 first attempts.
- Use host-neutral synthetic data. Keep app-specific DAW widgets and
  product-owned behavior out of every scenario.

### Matched conditions and first-attempt boundary

- The router attachment is the only condition difference. Keep the prompt,
  model and settings, tools, network posture, repository and package state,
  viewport, theme, density, and time budget identical.
- One first attempt is one fresh agent run. The agent may inspect, build, run
  the shared checks, and self-correct inside that run. Capture its first final
  submission. No operator feedback, reroll, manual patch, or selective
  regeneration follows before grading.
- Digest the router, prompts, fixtures, named checks, rubric, thresholds, and
  execution settings before any output exists.
- The Northstar owner randomizes conditions. Generators and reviewers see
  neutral run identifiers until all scores and notes lock.
- Settings drift, leaked condition labels, broken capture, or a rerolled output
  invalidates both conditions and both trials for that scenario. Freeze a new
  pilot revision and rerun the complete matched scenario block; never replace
  one output selectively.

### Separate evidence tracks

- Semantic and behavioral evidence is pass/fail against the scenario-required
  contracts and targeted interaction.
- Accessibility evidence is separate automated and manual keyboard, focus,
  naming, and role review.
- Composition uses the six-dimension blind human rubric from the dossier:
  reader job and hierarchy; component fit and composition; state and content
  clarity; density and responsive composition; copy and explanation; restraint
  and consistency.
- Mechanical evidence uses only named checks frozen before generation.
- Renderer parity is not evaluated in this Svelte-only pilot.
- Guided outputs must contain zero scenario-required semantic, behavioral, or
  accessibility blockers. Route those failures outside the composition score.
- At the fixed narrow viewport, only predeclared hard failures block: overflow,
  an unreachable primary action, or lost required evidence. Crowding and weak
  priority remain composition-rubric deductions.

### Reviewer calibration and scoring

- Before pilot grading, both reviewers independently score one strong and one
  weak non-pilot anchor while blind to condition. They discuss rationale and
  must reach agreement within one point per dimension. The anchors never enter
  pilot aggregates.
- Each output score is the equal-weight mean of every applicable score across
  both reviewers and all six dimensions. Round only the final output score to
  two decimal places.
- `N/A` is valid only when the frozen scenario brief predeclares the dimension
  for both conditions. At least five dimensions must remain applicable or the
  scenario is not gradeable.
- A reviewer gap of two or more points on any dimension triggers one blind
  rationale-and-rescore round. If the gap remains, that scenario is
  inconclusive and cannot support promotion.
- Average the two outputs within each scenario and condition. Give each of the
  three in-set scenarios equal weight in the condition aggregate.
- Keep the hidden holdout outside the in-set aggregate. Evaluate it as a
  separate transfer gate.

### Mechanical counting

- Freeze the named checks and their applicability per scenario before
  generation.
- Count each applicable check once per output. The denominator is all
  applicable check-output opportunities.
- Repeated occurrences of the same failed check remain diagnostic detail and
  do not inflate the verdict.

### Predeclared decision rule

Promotion requires every gate below:

- guided in-set score at least `4.00`;
- in-set lift at least `0.50`;
- no individual in-set scenario regression greater than `0.25`;
- guided holdout score at least `3.75`;
- holdout lift at least `0.25`;
- guided mechanical failure-opportunity rate no higher than control;
- no blocker class introduced only under guidance;
- zero guided semantic, behavioral, or accessibility blockers; and
- no inconclusive reviewer result or invalid matched block.

Record `reject` when the router conflicts with canonical Poodle authority or
when the guided in-set aggregate or holdout regresses by `0.25` or more.
Record `revise` for any other missed promotion gate. A revised candidate needs
a newly frozen matched rerun before promotion. The owner cannot override these
thresholds after seeing results.

### Correction routing and recurrence

- Route a contract, behavior, API, or accessibility error to its owning
  contract or implementation and stop the composition-promotion path.
- Route a missing reusable token, layout, or behavior mechanism to the
  canonical shared owner.
- Route a stable, objectively detectable failure to a deterministic check only
  when it can be expressed without encoding taste.
- Add a composition correction to maintained guidance only when the same class
  recurs across two independent scenarios. The holdout may count as one.
- Keep a single-run or single-scenario artifact in the correction ledger.
- Keep product-specific behavior outside Poodle.

### Manifest, privacy, and retention

- The immutable manifest records the Poodle commit and package versions;
  provider and exact exposed model/version; date and generation settings;
  tool and network posture; viewport, theme, and density; and router, prompt,
  fixture, check, and rubric identifiers and digests.
- Run in an isolated environment with no credentials, customer data, private
  corpus, or ambient session context. Freeze the network posture.
- Contamination invalidates the affected matched scenario block. Destroy its
  contaminated artifacts.
- Keep sanitized raw outputs, transcripts or action traces, and screenshots
  outside canonical source through the verdict plus a 90-day audit window,
  then delete them.
- Retain the compact manifest, digests, scores, reviewer notes, correction
  ledger, and verdict as the durable pilot record.
- If promoted guidance or a linked canonical source changes materially, rerun
  the fixed pilot at the next named Poodle release checkpoint. No unchanged
  every-release rerun is required.

## Recommendations

- Keep the router smaller than the sources it routes to. Its value is choosing
  the shortest correct path for the current job.
- Materialize exact prompts, fixtures, checks, and the source-routed router only
  in a separately promoted pilot brief. Poodle core approves the router and
  source map before its digest locks; the Northstar owner approves the complete
  run manifest before generation.
- Treat a passing pilot as evidence to promote the router, not evidence that
  any generated page is shippable or that the method generalizes to another
  runtime or model.

## Evidence

- Poodle already owns normative component semantics, shared mechanics, tokens,
  specimen teaching rules, and deterministic checks. The research dossier
  identifies discoverable composition routing as the missing layer.
- The g15 catalogue audit shows that mechanical screening and human teaching
  judgment catch different failures. Neither substitutes for a matched
  first-attempt comparison.
- The research dossier supports a finite Svelte-only manual pilot with fixed
  conditions, independent first attempts, blind review, a hidden holdout, and
  separate evidence tracks. The operator has now settled its candidate,
  ownership, scenario shape, arithmetic, blocker rules, thresholds, and
  retention boundary.

## Alternatives Held Or Rejected

- Hold a public `design.md` until audience, package or release ownership,
  versioning, and maintenance are separately decided.
- Reject copied Vercel guidance, a general design prompt, duplicated contract
  or token rules, and a parallel class system.
- Hold model judging, corpus ingestion, permanent selectors, and a general
  evaluation platform.
- Reject pixel difference or one aggregate quality number as the verdict.
- Reject broadening a Svelte result into a React, GPUI, or active-cohort claim.

## Unresolved Questions

None inside this planning boundary. Exact prompts, fixtures, named checks, and
the router text are future materialization work governed by the settled rules,
not choices this packet resolves or authorizes.

## Non-Goals

- No public guide, broad design prompt, app-specific widget, catalogue
  replacement, token or class system, runtime implementation, release
  coupling, permanent harness, model judge, or cross-runtime claim.
- No generated pilot output or canonical Poodle source change in this lane.

## Proposed Canonical Destinations

These are promotion recommendations, not execution authority:

- a Poodle-core-owned repository-local agent skill for the three-route router;
- one bounded roadmap card or equivalent operator-approved pilot brief for the
  finite run, exact frozen manifest, validation, and stop conditions;
- the normal completion log for the compact verdict and retained evidence;
- the narrow existing contract, implementation, token, check, or guidance
  owner for each accepted correction.

The orchestrator chooses exact paths against current `main` after intake. Do
not create a permanent evaluation spec or harness by default.

## Promotion Boundary

The orchestrator reviews and merges this packet as intake, then reconciles it
with current `main`, chooses canonical destinations, and promotes settled
meaning in a separate batch. No pilot implementation or guidance surface
becomes ready from this packet alone.
