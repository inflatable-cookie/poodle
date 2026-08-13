# 001 Working Rules

Status: active
Updated: 2026-08-14
Owner: Poodle core
Depends on: [Product Guardrails](../architecture/product-guardrails.md)

## Authority

- Architecture defines stable ownership and runtime boundaries.
- Specs define repository-wide normative rules.
- Component contracts define public component semantics.
- Roadmaps define active sequencing.
- Logs record completed work and point-in-time evidence.

Execution status does not belong in contracts. When two documents conflict,
prefer the narrower current authority and repair the stale document.

## Contract-First Changes

- Update a component contract before changing observable inputs, defaults,
  states, events, keyboard behavior, accessibility, layout intent, or token use.
- Keep contracts renderer-neutral. Put framework and engine details in runtime
  notes only when they affect parity.
- Document intentional runtime differences and their reason.
- Do not infer parity from a preview specimen alone; validate contract behavior
  and relevant interaction evidence.

## Shared Implementation

- Put framework-free web state and interaction logic in `poodle-core`.
- Put shared web component styles in `poodle-core/styles`.
- Keep Svelte and React shells idiomatic and thin.
- Put shared native component composition in `poodle-render`.
- Keep GPUI and Jetstream backends limited to runtime interpretation, input,
  lifecycle, and drawing concerns.
- Extend the shared node vocabulary only for reusable rendering capabilities.
- Capability absence is declared with a reason, never inferred from a runtime
  being silent. A declared absence records debt; it does not count as parity
  or component completion.

## Runtime Parity Authority

Poodle has four runtimes: Svelte, React, GPUI, and Jetstream.

- **Svelte is the reference implementation.** Where runtimes disagree on what a
  component can do, Svelte is what the others are brought up to.
- A capability present in Svelte and absent elsewhere is a **gap to port**, not
  an accepted delta. It remains a failing completion condition even when the
  absence is declared and explained.
- A capability present in another runtime and absent from Svelte is a
  **candidate for inclusion**, not an automatic port. Evaluate it, then either
  add it to Svelte and the contract, or record why it stays runtime-specific.
- The exception is genuinely runtime-owned behavior — focus, IME, portals,
  measurement, pointer capture, text systems, accessibility projection. Those
  are adapter capabilities and are expected to differ in mechanism while
  matching in observable result.
- Porting a capability includes documenting it. An undocumented capability is
  not "in Svelte and missing elsewhere"; it is drift on every side. The
  contract's props table is part of the port.
- Web-native attributes (for example `autocomplete`, `spellcheck`, `autofocus`)
  belong to the web runtimes and stay excluded from the portable Rust spec.
  Imperative escape hatches (for example `focus()`) are documented as methods,
  not props, and are expected in both web runtimes.

Contracts remain the semantic authority. This rule decides what *should* be
true when a contract is silent and the runtimes disagree; it does not let an
implementation override a contract that already speaks.

### Every component ships in all four runtimes

A component is not exempt from a runtime because of where it is typically used.
A titlebar control, a desktop-only affordance, a dev-tool surface — all four
runtimes still implement it. "It's only used on the web" is not a reason to
skip GPUI and Jetstream; the runtimes are targets, not use cases.

Distinguish two things that sound alike:

- **Component parity is required.** Every component has a contract, a Svelte
  and React implementation, a `<Name>Spec`, a `poodle-render` implementation,
  and a specimen in both native previews.
- **Web-platform prop parity is not.** Native attributes like `autocomplete`,
  `autofocus` and `spellcheck`, imperative escape hatches, and DOM-node props
  stay web-only and out of the portable spec. `WEB_ONLY_PROPS` in
  `contract-spec-drift.ts` is the sanctioned register for these, and each entry
  carries its reason.

A capability that genuinely cannot cross — a CSS selector, a DOM element
reference — is a documented delta with its rationale, not a silent omission.
Where a web capability has no native equivalent, the native target implements
the *observable result* by its own means, or the contract records why it
cannot.

## Component Ownership

- Poodle owns reusable primitives, composites, and general workstation shells.
- Applications own routing, persistence, data fetching, authorization, domain
  vocabulary, and workflow orchestration.
- App-specific DAW or product widgets remain in their owning repositories.
- Underlay integrations preserve Underlay-owned public APIs behind adapters and
  token bridges.

## Tokens and Presentation

- Change token meaning in the canonical DTCG schema and regenerate every
  target.
- Components consume semantic tokens rather than hardcoded theme values.
- Theme, density, control size, and contrast remain independent axes.
- Use `typography="inherit"` when an inline text-like primitive should follow
  parent typography. Shell geometry should scale proportionally where the
  runtime supports it.

## Svelte Surface

- Prefer Svelte 5 runes for new or substantially changed internals.
- Prefer callback props and snippets for new public composition surfaces.
- Add compatibility aliases only for a documented downstream migration need.
- Treat Bits Svelte as an implementation detail, never as contract authority.

## Accessibility

- Non-interactive layout primitives remain accessibility-neutral by default.
- Semantic regions, labels, focus behavior, keyboard operation, dismissal, and
  announcements must be explicit in the contract.
- Native implementations preserve equivalent semantics where runtime support
  exists and document unsupported capabilities where it does not.
- An adapter may translate API shape but must not silently drop accessibility
  behavior.

## Validation

Use Effigy as the command surface. Run the narrow checks relevant to a batch,
then the broader repository or docs gate before handoff. Generated evidence
must describe the current implementation and must not be edited by hand.
