# 001 Poodle Vision

Status: active
Updated: 2026-11-27

## Purpose

Poodle is a generalized design system for teams that need the same UI language
across Svelte, React, and GPUI applications.

It turns component meaning, design tokens, themes, interaction rules, and
accessibility expectations into explicit contracts. Each renderer stays
idiomatic while operators receive consistent behavior and presentation.

## Core Promise

Every Poodle component should:

- have one renderer-neutral contract
- use one generated semantic token vocabulary
- preserve the same observable inputs, states, events, and behavior across
  supported runtimes
- state intentional runtime differences
- remain reusable outside the application that first needed it

Poodle does not pursue byte-for-byte or implementation-level sameness. Parity
means semantic behavior, accessible operation, layout intent, and token use
before rendering mechanics.

## What Poodle Owns

- W3C DTCG design-token sources and CSS, TypeScript, and Rust artifacts
- portable theme, density, control-size, and appearance-Recipe rules
- framework-neutral component contracts
- foundational controls and layout primitives
- reusable application composites
- general workstation and professional-tool shells
- Svelte and React web implementations
- a shared Rust renderer with a GPUI backend
- documentation, parity rules, examples, and migration guidance

## What Poodle Does Not Own

- product-specific workflows, routing, persistence, or service calls
- Loophole-specific DAW surfaces such as transport, timeline, mixer,
  automation, or plugin panels
- Underlay's application-facing APIs and templates
- Bits Svelte APIs as public Poodle contracts
- renderer-only shortcuts that would silently change component semantics

Applications should build domain surfaces from Poodle components rather than
moving domain vocabulary into the design system.

## Ecosystem Boundaries

### Underlay

Underlay and the applications built on it import Poodle's published packages
directly. Underlay may add its own templates or bridges on top, and it owns
them; Poodle does not carry an Underlay-facing adapter layer. Operator decision
2026-09-02, recorded in architecture 001.

### Loophole and other products

Product-specific components live with the product. They may compose Poodle
tokens and components, but Poodle accepts them only when their semantics are
general across products.

### Web renderers

Svelte and React share framework-free behavior, styles, tokens, and contracts.
Bits Svelte may support the Svelte implementation internally, but it does not
define public naming or behavior.

### Native renderers

`poodle-render` produces renderer-neutral node trees, and the GPUI backend
interprets them using native runtime capabilities. Renderer-specific input,
lifecycle, and drawing remain in the backend.

## Direction

- **GPUI parity is the goal.** Operator ruling 2026-09-02: missing parity is
  what blocks committing applications to GPUI, and real parity lets many
  applications switch. Do not recommend slowing native work. The first parity
  application was Nucleus; its fixed component cohort and receipts live in
  [`docs/evidence/nucleus/`](../evidence/nucleus/README.md). Operator note
  2026-09-29: that Nucleus has been archived and is being rebuilt from
  scratch, with no Poodle in it yet. Operator ruling the same day: the
  receipts stay as frozen history and no longer pin runtime source, so native
  fixes in `packages/gpui`, `packages/render` and `packages/contracts` need no
  evidence repin. GPUI parity stays the goal; the rebuilt Nucleus can anchor
  new evidence once it adopts Poodle.
- **The fleet and its agents come first; public adoption is the long-term
  aim.** Operator ruling 2026-11-25. Choices serve the operator's
  applications and the agents that build them now, and should not close off
  public use later.
- **Poodle offers three entry points.** Operator ruling 2026-11-25: a
  behaviour layer, a style framework, and opinionated components built on
  both. A consumer or an agent can enter at any layer, so when the library
  lacks a component the agent drops down one layer and stays in the system.
  The redesigned Poodle look is built as the first style, not as the only
  one.
- **Poodle keeps its own behaviour core.** Operator ruling 2026-11-25. The
  core is framework-free and has a matched Rust crate with shared test
  vectors; DOM-bound libraries such as Vercel's UI SDK cannot give the Rust
  renderers that. Poodle borrows ideas from them and does not rebase onto
  them.
- **Rendering style and colour theme are independent axes.** Operator ruling
  2026-11-25: the style language is portable alongside theming, so a consumer
  application can switch rendering style and colour theme separately, for
  example from a settings picker. A style is a set of token modes plus a
  small, fixed set of treatment switches: control fill, surface separation
  and the selected state. Token modes alone change radius, weight, elevation and motion
  but leave every component built the same way, so a second style would not
  be a second look. Each renderer implements each switch value once, and the
  switches live in the token sources, not in a preview application.
- **The redesign removes noise.** Operator ruling 2026-11-25. Use subtle
  fills, fewer and softer borders, minimal labelling, and less accent colour.
  On 2026-11-26 Tom made the experimental `quiet` look the web default and
  retired both the previous look and the experimental style name. The
  appearance change ships with the 0.5 release. Fallback cleanup and
  wider type-role adoption follow.
- **A theme is a few settings, and every named theme is a preset.** Operator
  ruling 2026-11-25, a clean break that replaces authored palettes. A theme
  is a scheme (light or dark), a base colour and how far it blends into
  black or white, an accent colour and how far it blends into the neutrals,
  and a contrast amount. Base and accent are picked the same way, from the
  same range of colours. At a blend of zero the colour has no effect on any
  neutral. Every colour role is derived from those settings by one function
  that holds text contrast; Nord, Solarized and the other named themes are
  saved combinations of the settings, and none authors its own palette. A
  consumer can offer the presets, or the settings themselves, at run time.
  A theme defines colour only, and no component rule selects on a theme;
  the three Iceberg overrides that did are removed (operator ruling the
  same day).
- **The design sweep lands on web first.** Operator ruling 2026-11-25.
  Soundcheck is the test consumer for a second style; Loophole is the
  long-term case. The route for the Rust renderers is recorded below and in
  [Q-004](questions.md#q-004--what-is-the-route-for-the-rust-renderers).
- **Poodle has one native backend, GPUI.** The 2026-11-27 ruling requires no
  GPUI fork and one wgpu revision across the GPUI stack and its host. See
  [system shape](architecture/001-poodle-system-shape.md).
- **The node tree is the seam between Poodle and a native runtime.**
  Operator ruling 2026-11-25. Poodle owns everything up to the node tree:
  the contracts, tokens, styles, theme resolution, behaviour and the shared
  renderer that builds every component once as nodes. A backend is a leaf
  that turns nodes into a runtime's elements; nothing depends on a backend,
  and a backend implements no component of its own.
  - The GPUI backend stays in this repository as a first-class one.
  - Poodle ships a conformance kit: node-tree fixtures with expected
    results that hold the GPUI backend to parity. Shared specimens stay for
    GPUI.
  - The lab hosts runtime previews and may pin each one to an individual
    Poodle commit, so that work in progress can be tested.
  - No repository is created per adapter.
- **Svelte is the parity authority.** When runtimes disagree, the others are
  brought up to Svelte; see
  [working rules](contracts/working-rules.md#runtime-parity-authority).
- **Adoption follows releases, not merges.** A change reaches a consumer only
  when a published Poodle release carries it. Underlay applications receive
  Poodle through Underlay foundation releases. Consumers pin published
  versions; they don't link Poodle's primary checkout, whose `main` and
  prepared `dist` move with every merge (operator ruling 2026-10-05). See
  [release](contracts/release.md).

## Success Criteria

Poodle succeeds when:

- an operator can choose a runtime and begin without learning repository
  history
- one token change generates aligned web and native artifacts
- one contract can be verified across every supported renderer
- a new renderer can reuse contracts and tokens without redefining them
- applications keep their public APIs and domain logic outside Poodle
- runtime differences are deliberate, bounded, and visible
- packages can evolve through pre-1.0 preview into a documented public release

## Failure Modes

- becoming a component dump for one application
- treating a preview gallery as proof of behavioral parity
- duplicating native component recipes in each backend
- duplicating web state or CSS in each framework
- forcing downstream teams to translate themes by hand
- exposing implementation dependencies as Poodle's public API
- hiding release status or unsupported behavior from operators
