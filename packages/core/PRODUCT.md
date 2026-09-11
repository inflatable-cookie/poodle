# Product

<!-- impeccable:product-schema 1 -->

## Platform

adaptive

## Users

Primary users are product engineers adopting Poodle to build consistent
professional web and native applications. Design-system maintainers extend its
contracts, tokens, components, and renderer implementations as a supporting
audience.

## Product Purpose

Poodle is a contract-first design system for applications that need one UI
language across Svelte, React, GPUI, and Jetstream. It makes component meaning,
design tokens, themes, interaction rules, and accessibility expectations
portable across runtimes while leaving each renderer idiomatic.

Success means an application can choose its runtime package and receive the
same observable component semantics without learning the other implementations
or translating Poodle's design language by hand.

## Positioning

Poodle defines renderer-neutral component contracts and one generated semantic
token vocabulary, then implements those contracts through paired web and native
paths. Parity is judged by observable meaning, state, behavior, keyboard and
accessibility support, layout intent, and token use—not by shared implementation
or pixel identity.

## Operating Context

Product engineers consume published runtime packages, exact pre-1.0 versions,
component contracts, integration guides, and generated theme assets. Maintainers
work from normative component contracts and W3C DTCG token sources, exercise
Svelte and React previews, and verify native behavior through the shared Rust
renderer and its GPUI or Jetstream backend.

Poodle is evaluated through real component behavior and parity evidence.
Preview galleries help explore the system but do not by themselves prove
conformance.

## Capabilities and Constraints

- Poodle owns generalized tokens, primitives, reusable composites, and general
  workstation shells. Product-specific workflows, routing, persistence,
  services, and domain widgets remain in their applications.
- Component contracts are the authority for public inputs, states, events,
  accessibility behavior, layout, token usage, and allowed runtime differences.
- Web behavior and styles are shared through `@inflatable-cookie/poodle-core`;
  Svelte and React expose thin idiomatic shells.
- Native composition is shared through `poodle-render`; GPUI and Jetstream
  backends interpret renderer-neutral nodes rather than reimplementing
  components.
- The W3C DTCG token schema generates aligned CSS, TypeScript, and Rust
  artifacts. Theme, density, and control size are independent axes.
- Engine-backed editors wrap CodeMirror, TipTap, and ProseMirror behind narrow
  Poodle contracts. Engine objects and arbitrary extension surfaces are not
  public Poodle APIs.
- Underlay and other applications import published Poodle packages directly.
  Consumer-specific adapters and public domain vocabulary stay with the
  consumer.
- Poodle is pre-1.0. Breaking changes may ship in `0.x` minor releases; adopters
  pin exact versions and read release notes.
- Svelte and GPUI are preview surfaces, React is experimental, and Jetstream
  integration remains deferred until explicitly admitted.

## Brand Commitments

The product name is Poodle. Public writing is direct, technical, and explicit
about support status, intentional runtime differences, package boundaries, and
evidence. It does not overstate preview maturity or disguise unsupported
behavior.

## Evidence on Hand

- Product promise and scope: `README.md` and
  `docs/vision/001-poodle-vision.md`.
- Package and renderer ownership: `docs/architecture/001-poodle-system-shape.md`.
- Normative behavior: `docs/contracts/components/`.
- Canonical token sources: `packages/tokens/schema/`.
- Real Svelte, React, GPUI, and Jetstream preview surfaces under `packages/`.
- Roadmaps, execution logs, tests, and parity receipts record implementation
  and validation history. They are evidence, not substitutes for current
  component contracts.
- No testimonials, customer claims, adoption metrics, or performance benchmarks
  are established; future work must not fabricate them.

## Product Principles

1. Define observable behavior once; keep renderers idiomatic.
2. Share semantics and token meaning, not implementation accidents.
3. Admit only reusable, cross-product UI into Poodle.
4. Make runtime differences and maturity visible rather than silently falling
   back.
5. Treat interaction and accessibility evidence as part of the component, not
   gallery polish.

## Accessibility & Inclusion

Web surfaces target WCAG 2.2 AA. Native runtimes must provide equivalent
semantics and operability where their accessibility APIs support them. Every
component contract specifies its keyboard and accessibility behavior, and any
runtime limitation must be explicit.
