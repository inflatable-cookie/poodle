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

## Catalogue Specimens

- Catalogue specimens are human-facing documentation. Their first job is to
  show what a component is for, what is available, and how it is normally
  composed.
- Keep `Examples` representative and curated. Do not replace it with an
  exhaustive case corpus or repeat size and density matrices already owned by
  dedicated tabs.
- Exhaustive fixtures, actions, and assertions belong in focused tests beside
  the component, not in the catalogue. The g14 pilot's shared case corpus and
  its projected `Conformance` tab are gone (`g14.008`, `g14.021`).
- A renderer-neutral specimen plan may share ordered tabs, sections, captions,
  and fixture references across runtimes. Runtime adapters still render real
  components and may own bounded presentation needed by their renderer.
- Axis tabs use the component's exact ordered public value domain. A prop named
  `size` does not imply the five-step `ControlSize` domain, and a prop named
  `density` does not justify a tab when its values have no observable effect.
  Every advertised axis value must render real evidence in every active
  runtime; an omitted, blank, collapsed, or fabricated row is a defect.
- Review specimens as documentation. A green test board does not make a
  specimen page useful, and an attractive specimen does not prove parity.

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

Poodle targets Svelte, React, GPUI, and Jetstream. The active completion cohort
is currently Svelte, React, and GPUI plus the renderer-neutral Rust declaration
and `poodle-node` output. Jetstream is a deferred backend integration until a
later admission runway proves its converter, input, accessibility, and preview
workflow against the same cases.

- **Svelte is the reference implementation.** Where runtimes disagree on what a
  component can do, Svelte is what the others are brought up to.
- A capability present in Svelte and absent from another active runtime is a
  **gap to port**, not an accepted delta. It remains a failing completion
  condition even when the absence is declared and explained.
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

### Every component ships in the active cohort

A component is not exempt from an active runtime because of where it is
typically used. A titlebar control, a desktop-only affordance, or a dev-tool
surface still implements Svelte, React, shared Rust composition, and GPUI.
"It's only used on the web" is not a reason to skip the Rust target.

Jetstream deferral is program-wide, not a per-component exception and not a
parity claim. Components must keep renderer-neutral specs, cases, and node
output so Jetstream can consume the same authority later. Reports must label
Jetstream deferred until its admission gate passes; they must not report it as
passing, complete, or an accepted absence.

Distinguish two things that sound alike:

- **Active-cohort component parity is required.** Every component has a
  contract, Svelte and React implementations, a `<Name>Spec`, a
  `poodle-render` implementation, and a GPUI specimen. Jetstream preview
  admission is deferred as one backend program rather than waived component by
  component.
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
