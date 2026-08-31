# transitions.dev Motion Learning for Poodle

Status: open
Captured: 2026-08-20
Sources:

- https://transitions.dev/
- https://github.com/Jakubantalik/transitions.dev

Operator intent: research every transition catalogued by transitions.dev,
extract its durable motion learning, and scope each useful pattern against
appropriate Poodle components.

## Why Keep This

Poodle has many interactive primitives and composites whose motion currently
emerges component by component. The linked catalogue may offer useful motion
patterns, principles, critique methods, timing/easing guidance, and repeatable
review techniques. Poodle needs a complete inventory and explicit component
mapping, not a sample of attractive effects or a bulk decorative application.

The repository has not yet been inspected. This note records the lead and the
questions to answer; it does not authorize a dependency, copied implementation,
new component API, or roadmap work.

## Later Research Questions

- What transitions does the current site list, how are they grouped, and what
  interaction or state change is each intended to explain?
- What durable motion principles do the site, repository, and any associated
  skills teach?
- Which ideas are general design guidance, and which are tied to a particular
  framework, browser API, or implementation?
- What does the repository's licence permit? Separate learnable principles
  from code or assets that would require attribution or cannot be reused.
- For every listed transition, which Poodle primitives or composites are a
  semantic match, a possible match, or an explicit non-match, and why?
- Which Poodle transitions currently feel abrupt, inconsistent, decorative,
  or mechanically timed?
- Can the useful vocabulary become shared duration, easing, distance, and
  choreography tokens rather than per-component constants?
- Which observable motion semantics belong in component contracts, and which
  painter/runtime mechanisms stay implementation-owned?
- How should reduced-motion, interruption, focus movement, dismissal,
  pointer/keyboard parity, and nested overlays constrain every pattern?
- Which ideas can match across Svelte, React, shared Rust composition, and
  GPUI despite different animation engines? Jetstream remains deferred under
  the current working rules.
- What small representative components would make a good motion pilot before
  any library-wide migration?

## Promotion Route

When this returns to the active conversation:

1. inventory every transition on the site and inspect the repository, licence,
   examples, associated skills, and stated design rationale;
2. audit Poodle's existing motion tokens, CSS transitions, overlay/disclosure
   behavior, and native animation capabilities;
3. produce a catalogue-to-component matrix with suitability, constraints, and
   explicit exclusions for every listed transition;
4. propose a small set of principles and candidate pilots for operator review;
5. promote accepted results into motion architecture/tokens and relevant
   component contracts;
6. only then compile implementation cards and migration order.

Keep this note open until that research is promoted, explicitly rejected, or
superseded by a broader Poodle motion-system decision.
