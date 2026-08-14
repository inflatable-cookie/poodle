# g14.001 Delivery Review

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Delivery: PR #10, commits `1ce3710e` and replacement `0d041108`
Verdict: do not merge; replacement proof remains unsound

## What The Delivery Proved

- One Button case corpus can project matching specimen groups and fixtures.
- Generated Rust declarations can replace the hand-written `ButtonSpec`
  declaration surface while keeping derived Rust methods beside it.
- Svelte, React, shared Rust composition, GPUI conversion, and Jetstream can
  emit reports with one broad observation shape.
- The planted failures show that generated-artifact drift and several direct
  output differences can be made legible.

These are useful results. The branch remains the evidence source.

## Merge Blockers

1. The TypeScript interface is still duplicated. `defineComponentInterface`
   returns the broad `ComponentInterface` type, while `ButtonPortableProps`
   and `ButtonPortableEvents` are written out by hand. A schema rename is not
   mechanically the web type authority.
2. The case corpus is not interface-bound. Fixture props and regions are
   `Record<string, ...>`; parts, states, events, and axes are free strings;
   `componentCase` validates no item against the Button interface. Typos can
   serialize and be ignored by every adapter.
3. The shared native observer is Button code under a generic name. It embeds
   Button's six part IDs, icon names, label search, and node-kind rules. The
   next profile must edit shared mechanism, firing the generic-runner stop
   condition.
4. Required active-runtime evidence can pass as vacuous. Native token-role
   assertions are always emitted as `vacuous`; GPUI focus is unobserved. The
   comparator only fails when every runtime is vacuous, so Svelte or React can
   hide a missing GPUI observation.
5. GPUI activation does not traverse the GPUI listener. The runner calls
   `to_gpui`, discards the element, then invokes the original node's
   `interaction.on_activate` callback. Jetstream proves its own input route,
   not GPUI's.
6. The delivery implements and requires Jetstream in the completion selector,
   contradicting the active-cohort decision. `conformance:complete` therefore
   needs the sibling repository and reports Jetstream passing instead of
   program-deferred.
7. The reported mechanism is 5,145 lines against 844 replaced lines. That is
   a required reassessment under spec 066, not a number to defer until rollout.

## Ruling

`g14.001` remains open. `g14.002` remains blocked. Reuse the good case/specimen
projection and Rust declaration work only where it survives the stricter
acceptance in the revised card. Do not stack another observer or gate beside
the failing mechanism.

The licence review under `g14.016` is independent and may proceed in the
orchestrator thread.

## Replacement Re-review

The replacement closes useful parts of the first review: TypeScript types are
now projected from the interface value, cases are closed over that interface,
GPUI uses a real window/input path, and Jetstream is deferred. The focused
selectors pass: 19 cases across Svelte, React, and GPUI, plus the packed web
consumer board.

The green completion result is not yet evidence of parity:

1. **The portable interface still permits cross-language shape drift.**
   `size`, `density`, and `defaultPressed` are nullable in the TypeScript
   projection, but generated Rust emits non-optional fields. Explicit
   `defaultPressed={false}` enters toggle mode on web; Rust collapses it into
   the same `false` used for absence, so the behavior cannot agree.
2. **Concrete normalized-output drift passes completion.** The web `label`
   part resolves to the label span with no role or name. GPUI's `root-label`
   resolution returns the root Button node, so that same part reports role
   `button`, name `Run`, and focusability. `conformance:compare` never compares
   observations; it only aggregates failures from fields a case happened to
   assert.
3. **The shared observers remain Button-specific.** The web runner hardcodes
   `.poodle-button__icon` and `.poodle-button__label`; the native observer has
   `NodeKind::Button` branches for label and role. This directly fails the
   card's generic-runner acceptance and stop condition.
4. **Required evidence is narrowed to what the implementation already exposes.**
   Icon names are deliberately recorded as `null` on web and omitted from
   assertions. Resolved channels are recorded but not compared. The governing
   architecture requires icon identity and says an unobservable required field
   is incomplete, not a reason to shrink the intersection.
5. **GPUI execution is not standing enforcement.** `docs:check` and `ci:web`
   run the web cases; `ci:native` only compiles the GPUI runner. An inert GPUI
   listener therefore passes every standing gate even though a manual
   `conformance:complete` run would catch it.
6. **The cost report is incomplete and its log is stale.** The command now
   reports 4,373 mechanism lines, not 4,308, while counting only one of four
   committed JSON artifacts. The omitted interface fixture, case fixture, and
   GPUI interface copy add 2,002 lines. Hypothetical reuse across the estate is
   not concrete amortization proof.
7. **The branch is not structurally clean.** Committed `<<<<<<< HEAD` markers
   remain in `PAPERCUTS.md` and `conformance-estate.md`; `git diff --check`
   fails. The batch log's clean-diff claim and the PR description are stale.

`g14.001` remains open and `g14.002` remains blocked. The next pass must correct
the proof mechanism, not add assertions that merely conceal the observation
and type mismatches.
