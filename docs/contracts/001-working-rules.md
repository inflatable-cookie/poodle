# 001 Working Rules

Status: active
Updated: 2026-08-09
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
