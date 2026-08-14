# g14.001 Delivery Review

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Delivery: PR #10, commit `1ce3710e`
Verdict: do not merge; replacement pass required

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
