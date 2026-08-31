# Icon Morphing Research

Status: open
Captured: 2026-08-31
Source: https://www.morphicons.com/#how
Operator intent: determine whether Poodle should implement its own icon
morphing capability rather than adopt the referenced library blindly.

## Why Keep This

Icon morphing could make related state changes feel continuous, but it may
impose path-topology, asset-authoring, runtime, and accessibility constraints
that do not fit Poodle's current icon system. The useful product shape could be
a generic primitive, a curated set of morphable pairs, or a lower-level motion
utility. Research must decide; this note does not.

Do not copy code or assets, add a dependency, or commit to an API before the
technique, licence, and architecture have been reviewed.

## Later Research Questions

- How does Morphicons normalize and interpolate geometry, and what path point,
  winding, subpath, stroke, fill, and view-box constraints does it impose?
- Can arbitrary icons morph reliably, or must Poodle author and validate
  curated pairs against a shared topology?
- How would this fit Poodle's existing Lucide-based icon pipeline and avoid
  turning third-party icon internals into public contract authority?
- What timing, easing, reversal, interruption, cancellation, and rapid-toggle
  semantics are required?
- How should accessible names, state announcements, focus, reduced motion, and
  static fallbacks behave?
- What are the SSR, hydration, bundle-size, browser-support, and rendering-cost
  implications for Svelte and React?
- Can shared Rust composition and GPUI provide equivalent semantics and useful
  visual parity without a browser SVG engine? Jetstream remains deferred under
  the current working rules.
- What does the site's and library's licence permit, including derived assets,
  algorithms, attribution, and redistribution?

## Promotion Route

1. inspect the referenced technique, implementation, examples, licence, and
   browser/runtime assumptions;
2. audit Poodle's icon contracts, asset pipeline, consumers, and native limits;
3. build a bounded technical spike with a few semantically related icon pairs;
4. present generic-primitive, curated-pair, utility, and no-build options for
   operator decision;
5. promote an accepted shape into architecture/contracts before compiling
   implementation cards.

Keep this note open until the research is promoted, rejected, or superseded.
