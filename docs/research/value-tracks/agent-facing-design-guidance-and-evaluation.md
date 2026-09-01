# Value track: agent-facing design guidance and composition-quality evaluation

Status: research complete; awaiting orchestrator review
Created: 2026-09-01
Updated: 2026-09-01
Priority: decision support for a bounded promotion pilot

## Executive summary

Vercel's method is useful to Poodle as a separation of concerns, not as a template to copy. The transferable pattern is:

1. a task-facing judgment layer that tells an agent how to frame a user's job and choose a composition;
2. bounded mechanics that make the approved visual system easy to use;
3. deterministic checks for known mechanical failures; and
4. a matched first-attempt review loop that decides which corrections belong in which layer.

Poodle already has most of layers 2 and 3, plus a human specimen-review practice. Its material gap is a maintained, agent-readable routing layer for composition judgment and a matched evaluation that tests whether that layer improves first attempts. The gap is not a missing universal prompt, token system, or renderer-conformance suite.

Recommendation:

- Contributor-local skill: **adapt, behind a promotion gate**. A small repo-local router could point agents to contracts, token sources, specimen rules, and the relevant Effigy checks. It must remain subordinate to those sources and must not restate them.
- Consumer-facing `design.md`: **hold** until an operator decides that Poodle needs a public composition guide, names its owner, and defines its release/version boundary. A public guide would be a different audience and authority surface from the contributor skill.
- Composition-quality evaluation: **adapt as a finite manual pilot; hold a productized harness**. Reuse existing Poodle evidence where it answers the question, but keep composition quality separate from semantic parity, renderer parity, and accessibility.

This dossier recommends a decision path. It does not authorize a skill, public guide, eval harness, implementation card, component change, contract change, or canonical promotion.

## Scope and evidence discipline

The research question is whether the method described in Vercel's “How our agents build on-brand pages with design.md” improves how agents compose, review, and learn Poodle interfaces.

This dossier distinguishes:

- **[LF] Local fact** — verified in the Poodle checkout at the captured revision.
- **[SF] Source fact** — verified in an external primary source or official documentation.
- **[SAC] Source-author claim** — a claim made by a source author that is reported, not independently established for Poodle.
- **[WI] Worker inference** — an interpretation or recommendation from the evidence.

The local baseline is Poodle `01e22f646d7f681dfd49f8966f86c97381d639d7`, the same revision as `origin/main` when this dossier was captured on 2026-09-01. Local links below therefore identify the exact source revision by this baseline; external live pages are marked with their capture date because their contents can change.

The source pass did not copy proprietary prompts, brand rules, or source text. Vercel's public artifacts were used to understand structure, mechanics, and governance. The Eve template was used as a governance precedent, not as a proposal for a Slack corpus agent.

## Method and source inventory

### Primary external sources

| Source | Evidence type and capture | What it contributes | Boundary |
| --- | --- | --- | --- |
| [How our agents build on-brand pages with design.md](https://vercel.com/blog/how-our-agents-build-on-brand-pages-with-design-md) | [SF] Official Vercel article, published 2026-08-31; checked 2026-09-01 | Describes the guidance/mechanics/check/evaluation loop, fixed scenarios, blind comparisons, correction routing, and reported outcome | [SAC] The reported run counts and failure reduction are Vercel's evidence, not Poodle validation |
| [Vercel `design.md`](https://vercel.com/design.md) | [SF] Public live artifact; checked 2026-09-01 | Shows how one public file can state reader job, evidence, composition, visual primitives, responsive behavior, and revision rules. At capture it was 39,519 bytes and 369 lines. | Mutable public content; not a Poodle authority or a source to reproduce |
| [Teaching agents product design at Vercel](https://vercel.com/blog/teaching-agents-product-design-at-vercel) | [SF] Official Vercel article, published 2026-06-25; checked 2026-09-01 | Describes the repo-local `product-design` skill, source routing, modes, exemplars, linters, and review feedback loop | [SAC] Vercel's operating model is not evidence that the same surface is needed in Poodle |
| [Vercel public brand stylesheet](https://vercel.com/geist/vercel-brand.css) | [SF] Public live stylesheet; HTTP 200 and CSS metadata checked 2026-09-01 | Shows the bounded-mechanics layer: namespaced classes, tokens, and composable report primitives | Mutable artifact; Poodle already has a canonical token/core-style system |
| [Vercel Labs Eve design template](https://github.com/vercel-labs/eve-design-template/tree/7f8e5a62b02cb3407e063fc98c56c83dabbd95f4) | [SF] Public repository, pinned to `7f8e5a62b02cb3407e063fc98c56c83dabbd95f4`; checked 2026-09-01 | Gives a concrete source snapshot, manifest, owner approval, precedence, refresh, and runtime isolation model | Governance precedent only; no Slack corpus agent, private corpus, or Eve runtime is proposed |
| [Demystifying evals for AI agents](https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents) | [SF] Official Anthropic engineering article, published 2026-01-09; checked 2026-09-01 | Defines tasks, trials, graders, transcripts, harnesses, suites, calibration, holdouts, and capability-versus-regression evaluation | General eval guidance; any Poodle numeric decision rule is deferred to the operator-approved pilot brief |

### Poodle sources inspected

| Source | [LF] Relevant current evidence |
| --- | --- |
| [`docs/architecture/product-guardrails.md`](../../architecture/product-guardrails.md) | Shared tokens, primitives, composites, workstation shells, and cross-runtime contracts are in scope. Product-specific widgets and behavior remain with owning products. Preview and screenshot evidence is not proof of design quality. |
| [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md) and [`docs/contracts/components/README.md`](../../contracts/components/README.md) | Contracts are normative for semantics, states, behavior, accessibility, layout, tokens, runtime parity, and known deltas. Catalogue specimens are teaching surfaces, not exhaustive test corpora. |
| [`docs/architecture/001-poodle-system-shape.md`](../../architecture/001-poodle-system-shape.md) and [`docs/architecture/003-component-docs-ia-and-implementation-substrates.md`](../../architecture/003-component-docs-ia-and-implementation-substrates.md) | Poodle maintains one contract across Svelte, React, shared Rust composition, and GPUI; Svelte is the reference visual-proof surface; review follows contract, shared behavior, runtime interaction/accessibility, tokens/layout, then evidence. |
| [`docs/architecture/002-token-system-and-package-layout.md`](../../architecture/002-token-system-and-package-layout.md) and [`packages/core/src/styles/README.md`](../../../packages/core/src/styles/README.md) | DTCG tokens and shared core styles are the canonical mechanics. Generated outputs are not hand-authored parallel systems. |
| [`docs/roadmaps/g15/specimen-catalogue-audit.md`](../../roadmaps/g15/specimen-catalogue-audit.md), [`docs/roadmaps/g15/specimen-plan-outline.md`](../../roadmaps/g15/specimen-plan-outline.md), and [`docs/roadmaps/g15/027-screen-clear-human-review.md`](../../roadmaps/g15/027-screen-clear-human-review.md) | Poodle has a bounded catalogue audit and a human screen-clear review method. The audit separates mechanical screening from human quality judgment and routes semantic blockers out of specimen-only repair. |
| [`packages/svelte/preview/src/catalog.ts`](../../../packages/svelte/preview/src/catalog.ts), [`parity.ts`](../../../packages/svelte/preview/src/parity.ts), and accessibility reporting | Preview surfaces expose adoption bars, stateful examples, token provenance, accessibility posture, automated/manual coverage, and review routes. They are evidence-planning surfaces, not a matched generated-composition evaluation. |
| [`test/visual/README.md`](../../../test/visual/README.md) and [`test/visual/fixtures/README.md`](../../../test/visual/fixtures/README.md) | The visual gate measures paired Svelte/React renderer output. Fixtures are data-only identities. Neither establishes composition quality or a general design score. |
| [`tasks/effigy.tasks.toml`](../../../tasks/effigy.tasks.toml) and the repo-local Effigy skill | Existing selectors cover contract drift, catalogue shape, preview checks, native construction, accessibility/report generation, and visual comparison. The native specimen probe is construction/axis evidence; the visual gate is renderer evidence. |

### Research method

The method was deliberately bounded:

1. read the handoff, local authority, current worktree state, and the existing value-track style;
2. inspect the named Vercel sources and the linked public stylesheet;
3. pin the Eve repository to its current public commit and read its source/approval workflow;
4. inspect Anthropic's primary eval guidance for trial, grader, holdout, and calibration requirements;
5. map each external construct to an existing Poodle source, evidence layer, gap, and owner boundary; and
6. produce one dossier with candidate verdicts and a finite pilot shape.

## Current Poodle audit

### Existing authority and evidence layers

| Layer | Current Poodle authority/evidence | What it answers today | Remaining question |
| --- | --- | --- | --- |
| Contract judgment | Component contracts, product guardrails, working rules | What a component means, which states/behaviors/accessibility/layout/token obligations are valid, and where product ownership stops | How should an agent choose and combine valid primitives for a user job without duplicating contract prose? |
| Composition teaching | Specimen catalogue IA, `catalog.ts` adoption bars, g15 audit and screen-clear review | Whether a human-facing specimen shows normal use, meaningful variants/states, composition, sizes/densities, captions, and readable layout | How should a contributor agent find this judgment quickly, and which recurring composition corrections are general enough to teach? |
| Shared mechanics | DTCG token schema, generated token outputs, core styles, shared Rust composition, runtime adapters | How approved semantics and tokens become reusable web/native behavior and styles | Whether a proposed agent-facing layer can route to the mechanics without inventing aliases or a second style vocabulary |
| Deterministic enforcement | Effigy selectors, contract/spec/callback/value-domain drift checks, catalogue checks, preview/report writers, native construction probe, visual comparator | Whether known shape, contract, report, construction, or renderer-parity conditions hold | Which composition failures can be encoded safely, and which still require human judgment? |
| Human review | Bounded g15 screen-clear cards and operator review of changed pages | Whether a specimen is understandable and useful in its intended human teaching surface; whether an observed issue is a real blocker | Whether guidance changes first attempts across independent prompts rather than only improving already-authored specimens |

[WI] The missing layer is a routing and learning surface between “contract is valid” and “this is a useful composition for the reader.” It should not become a new normative contract.

### Bounded audit evidence

The current catalogue audit is a useful source of candidate composition defects, but it is not a generated-agent evaluation.

- **[LF] Coverage:** the audit records 175 entries: 174 portable entries plus web-only `MeterSurface`.
- **[LF] Human judgment:** 56 pages received the screen-clear human review; the audit leaves 108 entries in a curation tranche. This is a bounded review result, not a fully human-graded catalogue.
- **[LF] Composition signals:** the audit records 53 overloaded or long `Examples` sections, six narrow-overflow cases, 52 blank Svelte captions, and pages whose interaction probes initially looked broken but required source/gesture interpretation.
- **[LF] Coverage signals:** it records missing or misleading size/density evidence, divergent caption idioms, audio-heavy matrices, and paired-route gaps. These are evidence/teaching problems, not all composition-quality failures.
- **[LF] Semantic boundary:** six human-review cards exposed semantic blockers, including nested-layer or native-semantics issues. Those were routed as blockers rather than “repair the specimen” work.
- **[LF] Heuristic boundary:** an interaction probe nominated 20 pages, but source checking confirmed 14 genuinely unwired pages and six non-defects caused by clipboard behavior, hover, right-click, terminal state, or navigation. A probe is a triage signal, not a quality verdict.

[WI] These observations are a strong seed list for a pilot's defect taxonomy. They do not establish prevalence, model lift, or a causal relationship with any guidance file.

### What current checks do not establish

The current system has deliberate evidence boundaries:

- **[LF] Semantic/behavioral parity** is a contract and interaction question. It is not the same as visual similarity.
- **[LF] Renderer parity** is a Svelte/React visual comparison question. It does not establish that the composition is useful, well-prioritized, or correctly chosen.
- **[LF] Accessibility** has explicit automated/manual/blocked statuses and runtime notes. An accessible page can still be poorly composed; a visually strong page can fail accessibility.
- **[LF] Composition quality** currently appears in specimen guidance and human review, but there is no matched with/without-guidance generated-output suite, first-attempt ledger, blind A/B protocol, or holdout decision.
- **[LF] React build** is present, but the local papercut record notes there is no equivalent React type-check selector. That is a mechanics/validation gap, not evidence for or against a design guide.

[WI] Reusing an existing visual gate as the primary composition score would collapse four distinct questions and produce misleading results.

## What the Vercel method actually provides

The article describes a three-part system: judgment in `design.md`, bounded mechanics in a stylesheet, and an evaluation loop. For comparison with Poodle's existing evidence model, this dossier splits the mechanics and checks into four layers.

### 1. Judgment: task-facing prose

[SF] Vercel distinguishes a repo-local `product-design` skill from a public `design.md` used when an agent is outside the repository. The repo-local skill routes to product judgment, interface-quality guidance, resilience, surfaces, copy, patterns, exemplars, and tooling. The article presents `design.md` as one public file for a defined output audience; it does not establish that the file itself is compact, concise, or short. The captured file was 39,519 bytes and 369 lines.

[SF] The public file focuses on reader job, evidence, caveats, composition, visual hierarchy, responsive behavior, and revision. It also tells the agent to use the public stylesheet's exact primitives rather than inventing names.

[SAC] Vercel says the guidance was built from fixed scenarios, repeated corrections, and reruns. Its article reports more than 200 development runs.

[WI] The transferable property is not “put more design rules in a prompt.” It is “put only recurring, observable judgment in a maintained layer and route the rest to executable or canonical sources.”

### 2. Mechanics: bounded primitives

[SF] The public stylesheet exposes a namespaced class vocabulary and tokenized report primitives. It removes recurring typography, spacing, layout, and evidence decisions from the model's free-form generation.

[WI] Poodle already has the corresponding canonical layer in DTCG tokens, core styles, component contracts, and shared composition. A new stylesheet-like vocabulary or `design.md` token table would be duplication and would violate current ownership rules.

### 3. Deterministic checks: known failures only

[SF] Vercel routes mechanical failures to deterministic checks. The article's final comparison reports a known-failure count, not a holistic design-quality score.

[SAC] In Vercel's reported comparison, three desktop scenarios were each rendered twice with and without `design.md`, using Codex GPT-5.5, for six pages. The encoded known-failure count was 39 with guidance and 91 without, reported as 57% fewer failures. Vercel also states that the sample was too small to establish quality or reliability and that every page in both groups still had at least one shipping-blocking failure.

[WI] The numerical result supports running a Poodle pilot; it does not support importing the percentage as a target or claiming that guidance makes an interface shippable.

### 4. Evaluation loop: fixed first attempts and correction routing

[SF] Vercel freezes prompts, mock inputs, render settings, model configuration, and scenario definitions; stores outputs and reviewer feedback; compares blind A/B first attempts; and lands each accepted correction in the narrowest layer that can own it. A new page type becomes a future evaluation scenario.

[SF] Anthropic's eval guidance separates task, trial, grader, transcript, outcome, harness, and suite. It recommends multiple trials for nondeterministic systems, code graders for objective conditions, human calibration for nuanced judgments, balanced/holdout tasks, and clean isolated environments.

[WI] Poodle can borrow the protocol shape without adopting a model judge, a corpus agent, a broad platform, or a new permanent test suite. The first credible experiment can be manual and finite.

## Gap map: Vercel construct to Poodle boundary

| Construct | Poodle equivalent | Fit | Gap or constraint |
| --- | --- | --- | --- |
| Repo-local design skill | Repo-local Effigy routing plus contracts, architecture, specimens, and package docs | **Adapt** | No composition router exists. Any new skill must link to canonical sources, not clone them. |
| Public `design.md` | Package READMEs, catalogue docs, contracts, and consumer-facing usage guidance | **Hold/adapt** | Audience, public ownership, release coupling, and source-of-truth status are unresolved. |
| Bounded public stylesheet | DTCG schema, generated tokens, core styles, shared behavior, adapters | **Existing** | Do not create a parallel class/token system. |
| Fixed recurring scenarios | g15 specimen families and bounded review cards | **Adapt** | Existing scenarios are authored specimen review, not matched agent first attempts. |
| Deterministic known-failure checks | Effigy contract/drift/catalogue/visual/report selectors | **Adapt** | Existing checks answer specific mechanics. A composition check needs a stable defect definition before code. |
| Blind A/B first attempts | No current equivalent | **Pilot-required** | Need fixed prompts/fixtures, model/version metadata, independent trials, blind reviewers, and a holdout. |
| Correction routed to prose/mechanics/check/harness | Existing ownership rules and blocker routing | **Adapt** | Need a pilot ledger that records the route and keeps one-off/model-specific issues out of canonical guidance. |
| Weekly review corpus | Poodle PR/review history and bounded audit findings | **Hold/adapt** | Use an owner-controlled review cadence. Do not build or copy a Slack corpus agent. |
| Public branded output rules | Poodle's generalized component and token contracts | **Reject as transfer** | Poodle's scope is reusable UI infrastructure, not Vercel brand replication. |

## Candidate comparison

The four verbs are evaluated independently for each candidate. “Adopt” means accept the construct with only local naming; “adapt” means retain the operating idea but change its scope; “hold” means no promotion until an operator decision or pilot evidence; “reject” means it conflicts with Poodle's boundaries.

### Candidate A: repo-local contributor skill

| Option | Assessment |
| --- | --- |
| Adopt | **No.** A direct Vercel-shaped skill would duplicate Poodle contracts, token rules, and specimen guidance. |
| Adapt | **Yes, conditionally.** A small skill could route a contributor agent by task: contract first; then shared mechanics; then specimen/review guidance; then the narrow Effigy selector. It could expose shape, implement, review, copy, and harden modes only if each mode points to an existing authority. |
| Hold | **Until owner and source map are named.** The skill must have a maintainer, update cadence, and a drift check for links/claims. |
| Reject | **Reject a generic all-purpose design prompt, copied brand rules, or a second component contract.** |

**Verdict: adapt after a finite pilot or operator-approved promotion brief.** The immediate output should be a routing map, not a large guidance corpus.

### Candidate B: consumer-facing `design.md`

| Option | Assessment |
| --- | --- |
| Adopt | **No.** A public file cannot silently become authority over component semantics, tokens, accessibility, or runtime parity. |
| Adapt | **Potentially.** If consumers have a real composition problem, a public guide could state reader/job, composition, state, density, token, and accessibility expectations while linking to versioned package contracts and examples. |
| Hold | **Yes now.** Poodle has not made the public-audience, distribution, versioning, owner, or release-cadence decision. |
| Reject | **Reject Vercel-specific brand guidance, public class names, and an unversioned parallel token vocabulary.** |

**Verdict: hold.** Do not create a public `design.md` in this lane. A consumer guide should follow an explicit operator decision and package/documentation ownership review.

### Candidate C: matched composition-quality evaluation

| Option | Assessment |
| --- | --- |
| Adopt | **No, not the Vercel numbers or harness shape wholesale.** The six-page comparison is a useful example, not a Poodle threshold. |
| Adapt | **Yes, as a finite manual pilot.** Use three in-set scenarios, one holdout, fixed inputs, independent first attempts, blind human review, separate deterministic tracks, and a small predefined defect ledger. |
| Hold | **Yes for permanent automation.** Do not add selectors, model-judge infrastructure, or a corpus until a pilot shows a repeatable signal and a named owner accepts the maintenance cost. |
| Reject | **Reject pixel difference as the composition score, an exhaustive specimen corpus, and a broad agent-evaluation platform.** |

**Verdict: adapt the protocol; hold implementation.** The pilot is the smallest next evidence step.

## Bounded pilot and evaluation rubric

This is a proposed pilot shape, not an implementation card. `[WI]` scenario briefs and the numeric decision rule require operator approval before use.

### Preconditions

The pilot should not start until:

1. an operator names the candidate under test: contributor-local guidance, consumer guidance, or both;
2. one owner accepts responsibility for the guidance source and one reviewer owns the pilot record;
3. the guidance is reduced to a versioned draft with links to canonical Poodle sources and no duplicated contract/token tables;
4. the target runtime is fixed. The recommended first target is the Svelte web reference-proof surface. Results must not be generalized to React or GPUI without a separate evidence track;
5. the Poodle commit, package versions, model/version, tool settings, prompt, fixtures, viewport, theme, density, and output retention rule are frozen; and
6. the operator accepts the proposed rubric and the pilot brief's numeric aggregation and decision rule before seeing the results.

### Scenario briefs

Use generalized Poodle primitives and host-neutral mock data. Keep DAW-specific widgets and product-owned behavior out of the sample.

| Scenario | Composition question | Required surface evidence |
| --- | --- | --- |
| Form and validation | Can the agent make the primary task and recovery path obvious without over-composing the form? | Field grouping, labels/help/error, default/invalid/pending states, primary/secondary action hierarchy, narrow layout |
| Browse and detail | Can the agent establish browse-to-detail hierarchy and useful empty/loading/no-results states? | Filter/search, list or table, detail entry point, state copy, density choice, responsive behavior |
| Workstation shell | Can the agent compose a reusable shell without inventing product-specific chrome? | Header/navigation/content hierarchy, tabs or split view where justified, resize or collapse behavior, status/readout placement, narrow layout |
| Hidden holdout: picker or review surface | Does the guidance transfer to a related surface that was not used to phrase the guidance? | Same evidence classes, but a different primitive family and a predeclared “do not over-compose” constraint |

[WI] The exact prompts and fixtures belong in the next operator-approved pilot brief. They should describe a user job, evidence, constraints, and expected output—not prescribe a screenshot or copy a source prompt.

### Matched conditions and trials

- Run the same prompt, mock data, tool access, model/version, temperature or equivalent settings, theme, density, and viewport with and without the candidate guidance.
- Capture first attempts only. No rerolls, manual patching, or selective regeneration before grading.
- Use two independent trials per condition for each of the three in-set scenarios. This produces 12 first attempts. Run the hidden holdout under both conditions for two independent trials, producing four additional attempts.
- Randomize the A/B labels and blind reviewers to condition. Preserve the mapping privately until scoring is complete.
- Record transcripts or action traces where behavior matters. Screenshots alone cannot establish semantics, interaction, or accessibility.
- Keep the pilot in a disposable, clean environment. Do not add generated outputs to the canonical Poodle catalogue or source tree.

### Separate evidence tracks

Do not collapse these into one “quality” number.

| Track | Grader | Result |
| --- | --- | --- |
| Semantic/behavioral | Contract-focused checks plus targeted manual interaction | Pass/fail and defect route; contract violations are blockers, not composition points |
| Renderer parity | Existing paired Svelte/React visual gate, only when the same approved surface is rendered in both runtimes | Renderer evidence/diff; not a composition score |
| Accessibility | Existing automated reports where applicable plus keyboard/focus/label/manual review | Separate status; a11y failure cannot be hidden by a good composition score |
| Composition | Two blind human reviewers using the rubric below | Per-dimension 1–5 scores, notes, and correction category |
| Mechanical known failures | Existing or predeclared deterministic checks only | Count of named failures; no post-hoc checks added to improve the result |

### Blind human composition rubric

Score each dimension from 1 to 5. Use “not applicable” only when the scenario explicitly does not exercise the dimension. Reviewers should write the observed correction, not a taste statement.

| Dimension | 1 | 3 | 5 |
| --- | --- | --- | --- |
| Reader job and hierarchy | The primary task is unclear or several entries compete | The task is discoverable but hierarchy or next action is uneven | The reader can identify purpose, priority, evidence, and next action quickly |
| Component fit and composition | Components are nested, duplicated, or chosen without a user-job reason | Most pieces fit, with one unnecessary or weak composition choice | Each primitive has a clear role and the composition is restrained and reusable |
| State and content clarity | Important states or evidence are absent, misleading, or filler | Main path is clear but one meaningful state or explanation is weak | Default, recovery, and decision-relevant states are explicit and useful |
| Density and responsive composition | The layout overflows, collapses, or loses priority at the fixed narrow viewport | It remains usable with local crowding or excess space | Density, spacing, and narrow behavior preserve the task and hierarchy |
| Copy and explanation | Labels/captions describe implementation or add noise | Copy is serviceable but some meaning is implicit or repetitive | Copy explains user meaning, evidence, caveats, and action without ceremony |
| Restraint and consistency | The page invents a parallel visual language or adds ornamental structure | Poodle mechanics are mostly used, with some inconsistent choices | The page uses the provided mechanics coherently and adds only job-required structure |

The last dimension is about use of the approved Poodle system, not resemblance to Vercel branding. Token and API compliance still belongs to deterministic or contract checks where possible.

### Numeric decision rule deferred

No numerical pass/fail threshold or aggregate score is adopted in this dossier. The trial counts above define data collection only; they do not define a promotion result. The operator-approved pilot brief must set the decision rule before any run is graded, including:

- the per-output score calculation across the six rubric dimensions;
- whether a dimension marked `not applicable` is excluded from that output's denominator, and the minimum number of applicable dimensions required for an output score;
- how the two reviewer scores combine for an output, including disagreement handling and rounding;
- how independent trials combine within a scenario, how scenarios combine across the in-set sample, and how the holdout is evaluated;
- the exact defect-count denominator and treatment of duplicate findings;
- the allowed semantic/behavioral, accessibility, renderer, and responsive regressions; and
- the numeric promotion, hold, and kill thresholds.

Until that brief is approved, the results may be described only as collected observations and reviewer corrections. They must not be called a lift, regression, pass, fail, or falsifiable threshold result. Non-numeric stop conditions remain: a guidance conflict with canonical authority, inability to complete the blind rubric, or a pilot that requires an unapproved path, app-specific widget, unbounded corpus, or general-purpose platform keeps the work held.

### Correction ledger and routing rule

Every material defect should be recorded against the exact run, scenario, condition, and reviewer note, then routed once:

| Observed correction | Correct destination |
| --- | --- |
| Meaning, state, event, accessibility, or API is wrong | Component contract or owning implementation; stop the composition promotion path |
| Reusable token, layout, or behavior mechanism is missing | Canonical token/core-style/shared behavior or adapter owner |
| Stable, objectively detectable failure | Deterministic check, if the check can be expressed without encoding taste |
| Repeated user-job, hierarchy, state-selection, or restraint judgment | Candidate contributor guidance, with links to the relevant Poodle source |
| False positive, missing fixture, broken capture, or bad comparison | Pilot harness/evidence record |
| One model, one prompt, or one accidental generation artifact | Keep in pilot notes; do not promote |
| Product-specific widget or owning-product behavior | Keep outside Poodle |

[SF] This narrowest-layer routing mirrors the Vercel article's described loop. [WI] Poodle should require recurrence across independent scenarios before a correction enters guidance; a single attractive example is not enough.

## Governance and source of truth

### Authority order

| Question | Source of truth | Role of a future guidance/eval surface |
| --- | --- | --- |
| What does a component mean and how does it behave? | Poodle component contract and approved architecture/spec | Link and route; never override or silently restate |
| What do tokens and shared mechanics mean? | DTCG schema, generated outputs, core styles, shared behavior, and adapters | Name the route; never invent aliases, fallback values, or a parallel class system |
| How should a contributor work in this repository? | Repo instructions, Effigy selectors, canonical docs, and a future local skill if approved | Make the shortest correct path discoverable |
| How should a specimen teach a human? | Specimen plan, catalogue adoption bars, and bounded review records | Feed recurring composition lessons back only after review |
| How good was a generated first attempt? | Immutable pilot manifest, outputs, transcripts, separate grader results, and reviewer notes | Evidence for a promotion decision; not canonical component truth |
| What may consumers read publicly? | Operator-approved package/docs ownership and release/version policy | A future consumer guide may summarize and link; it cannot silently become a second contract |

### Minimum governance if anything is promoted

[WI] A promoted local skill or public guide should carry:

- named owner and backup owner;
- exact source links for every normative-looking rule;
- audience and supported task/surface boundary;
- Poodle revision or package release boundary;
- update trigger and review cadence;
- a link/drift check appropriate to the owning repository; and
- an explicit statement that contracts, tokens, accessibility rules, and runtime parity remain authoritative elsewhere.

[SF] Eve's public template provides a useful governance precedent: immutable dated source snapshots, a manifest with identity/origin/owner/priority/status, explicit precedence, owner approval, and refresh by superseding rather than mutating an approved snapshot. [WI] Poodle should borrow those controls only for a future guidance/eval record; it should not import Eve's corpus or runtime model.

### Pilot record

If approved, each run should preserve an immutable manifest containing:

- Poodle commit and package/runtime versions;
- guidance identifier, revision, and digest;
- prompt and fixture identifiers/digests;
- model/version and generation settings;
- viewport, theme, density, and tool/network posture;
- condition and randomized blind label;
- output, transcript/action trace, deterministic results, and screenshots where relevant;
- reviewer notes and correction routes; and
- decision status, owner, and approval date.

The record should contain no secrets, private customer data, unlicensed third-party corpus, or mutable “latest” source reference. Old runs remain evidence; a later guidance version creates a new comparison.

### Cadence and drift

The first round should be finite. If promoted, rerun the fixed pilot suite when the guidance or a relevant contract/mechanic changes, and review the correction ledger at a named release checkpoint. Add scenarios only for a new recurring page type or a repeated defect class; do not grow an exhaustive universal corpus.

The existing Poodle checks should continue to own contract, token, catalogue, report, and renderer drift. A future guidance check may detect stale links or missing source references, but it must not encode subjective composition as a silent pass/fail alias.

## Risks, non-goals, and rejected transfers

### Risks

- **Source-author overclaim:** Vercel's 39-versus-91 result is a small, known-failure comparison. It does not prove Poodle lift, reliability, or shippability.
- **Guidance drift:** a guidance file can become a shadow contract or fossilize one team's taste unless every rule has a source, owner, and expiry/update path.
- **Prompt overfitting:** three scenarios can reward wording or a model quirk. The holdout and independent trials are required to expose this.
- **Metric collapse:** a pixel pass, accessibility pass, or contract pass cannot stand in for composition quality. Combining them hides the failure mode that needs repair.
- **Reviewer variance:** blind human review is necessary for nuanced composition, but it needs calibration, anchored scores, and correction notes.
- **Public-boundary confusion:** a contributor skill and a consumer `design.md` have different audiences, packaging, and update authority. One should not be published as the other.
- **Maintenance cost:** a permanent model-evaluation harness can become a new product surface before Poodle has evidence that the guidance is valuable.
- **Runtime generalization:** a Svelte pilot does not establish React or GPUI behavior, accessibility, or visual parity.

### Non-goals

- No app-specific DAW widgets or product-owned behavior.
- No duplication of component contracts, token schema, or runtime parity rules.
- No universal scene/corpus or exhaustive specimen benchmark.
- No Slack-corpus agent, private knowledge ingestion, or broad agent platform.
- No Vercel brand/style reproduction or copied proprietary prompt/style text.
- No model-as-judge as the first or sole composition grader.
- No use of the existing visual gate as a design-quality score.
- No roadmap implementation card or canonical architecture/contract change from this research lane.

## Exact recommendation

**Adapt Vercel's separation of judgment, mechanics, deterministic checks, and review into a staged Poodle experiment.**

The staged disposition is:

1. **Contributor-local skill — adapt, then promote only if owned and source-routed.** [WI] Keep a future skill compact and task-specific. Its first job is to tell an agent where to look and which existing check to run, not to define new design truth.
2. **Consumer-facing `design.md` — hold.** Require an explicit public-audience, ownership, versioning, and release decision before writing one.
3. **Composition-quality evaluation — adapt the protocol, hold the harness.** Run one finite matched pilot with a predeclared operator-approved decision rule and separate evidence tracks. Do not add permanent tooling unless that rule records a passing result and an owner accepts the cost.

This recommendation is intentionally narrower than “adopt Vercel's system.” It preserves Poodle's contract-first architecture and treats generated-interface quality as an evidence question.

## Operator decisions required

The orchestrator should decide:

1. Is a repo-local contributor skill a desired Poodle surface, and who owns its updates?
2. Is there a real consumer-facing composition problem that warrants a public `design.md`? If yes, which package/release owns it?
3. Is the first pilot limited to Svelte web, or is a cross-runtime question explicitly funded as a separate track?
4. Who approves the scenario briefs, rubric, thresholds, and final promotion decision?
5. What retention, privacy, and review policy governs generated outputs, transcripts, and screenshots?
6. Which existing review/roadmap checkpoint is the right place to rerun the fixed pilot after a promoted guidance or contract change?

Until these decisions are made, the correct disposition is **hold implementation and canonical promotion**.

## Smallest useful next artifact

[WI] If the orchestrator elects to continue, the smallest useful next artifact is a one-page, owner-approved pilot brief containing:

- the three in-set prompts and one hidden holdout;
- the exact guidance draft and canonical source map;
- fixed model/runtime/render settings;
- the separate evidence tracks and blind rubric;
- the predeclared numeric thresholds and non-numeric kill conditions; and
- the owner, reviewer, retention rule, and decision date.

That brief should be created only after the operator decisions above. It should not become an implementation card until the approved decision rule records a passing result and the relevant authority owner accepts promotion.

## Follow-up disposition

Research is complete and awaits orchestrator review. This lane changed only this dossier. No contributor skill, consumer guide, eval harness, component, contract, token source, catalogue specimen, roadmap item, or canonical architecture file was created or promoted.
