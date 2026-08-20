# transitions.dev Motion Learning for Poodle

Status: open
Captured: 2026-08-20
Source: https://github.com/Jakubantalik/transitions.dev
Operator intent: extract the repository's learning about better UI transition
animation and apply it where useful in Poodle.

## Why Keep This

Poodle has many interactive primitives and composites whose motion currently
emerges component by component. The linked repository reportedly contains
skills focused on designing better UI transitions. It may offer useful
principles, critique methods, timing/easing guidance, or repeatable review
techniques that Poodle can turn into a coherent motion language instead of
copying isolated effects.

The repository has not yet been inspected. This note records the lead and the
questions to answer; it does not authorize a dependency, copied implementation,
new component API, or roadmap work.

## Later Research Questions

- What durable motion principles do the repository and its skills teach?
- Which ideas are general design guidance, and which are tied to a particular
  framework, browser API, or implementation?
- What does the repository's licence permit? Separate learnable principles
  from code or assets that would require attribution or cannot be reused.
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

1. run a bounded external-evidence research pass over the repository, its
   licence, skills, examples, and stated design rationale;
2. audit Poodle's existing motion tokens, CSS transitions, overlay/disclosure
   behavior, and native animation capabilities;
3. propose a small set of principles and candidate pilots for operator review;
4. promote accepted results into motion architecture/tokens and relevant
   component contracts;
5. only then compile implementation cards and migration order.

Keep this note open until that research is promoted, explicitly rejected, or
superseded by a broader Poodle motion-system decision.
