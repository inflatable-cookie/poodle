# 062 Headless Core And Dual-Layer Strategy

Status: promoted — program complete (g11.002–007); durable outcomes live in architecture 006/007 and the component contracts. Retained as decision history.
Updated: 2026-07-10
Depends on: `021-public-package-api-stability-and-parity-debt-baseline.md`, `026-appearance-recipes-and-downstream-override-strategy.md`, `028-primitive-baseline-and-bits-aligned-surface.md`

## Purpose

Define the target architecture for splitting Poodle into a framework-agnostic
headless behavior layer and per-framework styled layers, without breaking the
17 local consumer apps that link `@inflatable-cookie/poodle-svelte` by `file:` path.

This is the master spec for the g11 headless-core runway (`g11.002` through
`g11.007`). Promote settled outcomes into `docs/architecture/` and
`docs/contracts/` as each phase closes; do not treat this file as permanent
architecture.

## Current Reality (evidence, 2026-07-10)

- `@inflatable-cookie/poodle-svelte` has **no Bits dependency**. External deps are `svelte`,
  `marked`, and `@inflatable-cookie/poodle-core/tokens`. Focus, portal, and dismissal behavior
  already live in `packages/svelte/components/src/internal` and sibling
  modules. Spec `028`'s Ownership Rule ("Bits remains an implementation
  substrate") is stale and superseded in part by this spec.
- 17 consumer packages across 10 roots link Poodle by `file:` to
  `packages/svelte/components/src` (source-consumed, no build step, no version
  buffer). Any src change lands in every consumer on next dev build.
- g11.001 modernized the full Svelte surface to runes/callback-first/snippet
  composition and closed the consumer rollout. The consumer matrix, wave
  process, and validation baseline in
  `docs/roadmaps/g11/001-svelte-modernization-and-consumer-rollout.md` are the
  reusable rollout machinery for this program.

## Decision: State-Machine Core, Not A Compiler

The headless layer is written once as plain framework-free TypeScript, in the
Zag.js pattern:

- per component: a state machine (states, events, transitions, guards,
  effects) plus prop getters (`getRootProps()`, `getTriggerProps()`, ...) that
  return plain attribute/handler maps including ARIA and
  `data-scope` / `data-part` / `data-state` attributes
- per framework: a thin adapter that subscribes to the machine and spreads the
  prop-getter output. All logic lives in core; adapters are glue

**Mitosis is rejected for the headless core.** Reasons recorded so the
decision is not relitigated casually: static-JSX lowest-common-denominator
input is hostile to imperative, ref/effect-heavy headless logic (focus traps,
dismissable layers, positioning, typeahead); debugging compiled per-framework
output multiplies surface; project is 0.x with single-digit npm dependents.
Mitosis may be revisited for *styled shells only* via the bounded spike in
`g11.007`.

## Decision: Package Shape

- `packages/core` → `@inflatable-cookie/poodle-core` (working name; confirm in `g11.003`):
  framework-free TS. Machines + shared machinery. Zero Svelte/React/Vue
  imports, zero DOM-framework assumptions beyond standard DOM APIs.
- Shared machinery modules inside core, built before component machines:
  focus management (trap, roving tabindex, restore), dismissable-layer stack,
  anchor positioning (wrap Floating UI; do not rewrite), presence/animation
  states, typeahead, id/aria wiring.
- `@inflatable-cookie/poodle-svelte` keeps its exact public surface and becomes the Svelte
  adapter + styled layer over core.

## Decision: Design Flexibility Via Slot Recipes

Extends spec `026` from strategy to product surface:

- every anatomy part emits `data-scope="tabs" data-part="trigger"
  data-state="active"` from the core prop getters
- the styled layer becomes slot recipes: per-part style maps keyed on
  state/size/variant, resolved from semantic tokens
- Poodle ships the default recipe per component; a consuming app overrides
  recipe slots per component without touching behavior and without rebuilding
  the suite
- token purity, treatment roles, and the cross-runtime/web-only lanes from
  spec `026` still govern what a recipe may contain

## Decision: Rust Mirror Path

The machine model is the cross-runtime contract. A pure transition function
`(state, event) -> state` plus a declarative part/attribute description ports
to Rust directly. `poodle-specs` already carries spec structs; `g11.006`
decides port-by-hand vs codegen from a machine-spec source of truth after the
TS machine shape has stabilized in `g11.004`.

## Consumer Compatibility Rule (hard constraint)

Because consumers are source-linked with no version buffer:

1. **Core-extraction waves are interface-invariant.** Swapping a component's
   internals onto core machines must not change its public props, callbacks,
   snippets, or markup semantics that consumers rely on. Proof per wave: the
   g11.001 validation matrix (typecheck across the validated consumer roots)
   plus targeted preview/demo checks.
2. **Interface changes ride the g11.001 wave process.** If a component's
   public surface must change, that change follows the existing rollout
   discipline: Underlay first, then root-by-root consumer migration in the
   same wave, with the consumer matrix updated as evidence.
3. **No long-lived dual surfaces.** Temporary shims follow g11.001 rules:
   exception, not default, removed within one wave.
4. **Recipes are additive.** The recipe layer ships with defaults that
   reproduce current visuals exactly; adopting overrides is opt-in per app.

## Resolved: Machine-Spec Authoring Format (g11.002)

Recommendation: **contract-markdown is the spec; TS is the implementation;
no codegen layer yet.**

- The `Behavior Machine` section (template + pilots in `checkbox.md`,
  `popover.md`, `tabs.md`) is the authoritative behavioral spec: context,
  states, events, transitions, effects-as-named-intents, part attribute
  output, machinery dependencies.
- TS machines in core implement that section directly. Transitions pure;
  effects emitted as data and executed by adapters — this discipline is what
  keeps the Rust port mechanical.
- Declarative-data + codegen (JSON/TOML → TS + Rust skeletons) is deferred,
  not rejected: the pilot writing showed the interesting content is guards
  and effects, which codegen cannot generate anyway. Revisit in `g11.006`
  with the real TS corpus; the conformance test vectors planned there are the
  cross-runtime sync mechanism either way.
- Pilot learning: sub-machines (Tabs drag/tooltip) and environment events
  (URL popstate, overflow measurement) fit the format cleanly; effects tables
  force cleanup rules to be explicit, which the old prose contracts left
  implicit.

## Resolved: Shared Machinery Inventory (g11.002)

Build list for `g11.003`, from the pilot specs:

- roving tabindex: wrapping, disabled-skipping index navigation
  (exists as `findNextEnabledIndex`/`firstEnabledIndex` in
  `packages/svelte/components/src/internal.ts`)
- focus: focusable-element query (`getFocusableElements`), initial-focus
  strategies, restore-to-trigger
- dismissable-layer stack: document-level escape + outside-pointerdown,
  innermost-first for nested overlays (current per-component listeners don't
  stack correctly — known improvement, must not change single-overlay
  behavior)
- anchor positioning: Floating UI wrapper; current Popover CSS anchoring is a
  documented delta until swap
- id wiring: instance-scoped ids for trigger/surface and tab/panel pairs
  (replaces per-component module counters)
- presence: not needed by pilots; deferred until an animated overlay wave
  needs it
- typeahead: not needed by pilots; expected at Select/Menu waves

## Resolved: Package Shape And Adapter Model (g11.003)

- Package: `packages/core` → `@inflatable-cookie/poodle-core`. Source-consumed
  (`exports` → `./src/index.ts`) like the sibling Svelte packages. Token
  types stay out of core; core is behavior-only.
- Core runtime: **pure functions, no interpreter/store.** Per component: a
  `*Transition(state?, context, event) → { state?, context, effects[] }`
  function plus `*Parts(...)` attribute getters. Adapters hold reactive state
  (Svelte 5 runes) and execute effect intents. Callbacks are effects
  (`emitValueChange`, `emitOpenChange`, ...). ~15-line adapters per
  component; direct blueprint for the Rust port.
- Dependency wiring: `@inflatable-cookie/poodle-svelte` depends on `@inflatable-cookie/poodle-core` via
  `workspace:*`. Verified against a real consumer (`soundcheck`): consumer
  `bun install` over the `file:` link succeeds, and imports resolve through
  `components/node_modules/@inflatable-cookie/poodle-core` → live workspace symlink.
  **Do not use `file:../../core` for the internal dep** — bun snapshots
  `file:` deps into its store, which shadowed live core source during
  development (stale-copy bug, caught in g11.003).
- Floating UI: **rejected (g11.004 wave 2).** The in-house
  `resolveOverlayPosition` (collision-aware flip candidates scored by
  viewport overflow then anchor overlap, viewport clamp) was promoted into
  core as `position.ts`, parameterized by viewport so it is pure and
  Rust-portable. Rationale: zero new dependencies, pixel-identical for the
  components already using it, and the Rust runtimes can mirror it directly
  — Floating UI would have been web-only. Tooltip/Menu/IconButton use it via
  a window-bound wrapper; HoverCard swapped its bespoke math onto it (flip
  near edges is a recorded improvement delta); Popover deliberately keeps
  CSS anchoring as a documented delta; ContextMenu's pointer anchoring stays
  adapter-side.
- Tests: `bun test` in `packages/core` (`bun:test`, no new dev deps beyond
  `@types/bun`). Machine tests double as the seed conformance vectors for
  `g11.006`.
- Machine event convention learned from a live regression: keyboard events
  carry the originating element index (`fromIndex`) and the machine prefers
  it over tracked context state (Tabs contract updated). Adapters must feed
  event-site facts into events instead of relying on mirrored state.

## Open Questions (resolve in the runway, record answers here)
- ~~Recipe override delivery mechanism~~ **Resolved (g11.005):** CSS
  custom-property contracts in a dedicated read-only namespace
  (`--poodle-recipe-<component>[-<variant>]-<slot>[-<state>]`), no JS API.
  Components resolve recipe hook -> treatment role -> token. Key finding:
  component-local variables cannot be the public surface (components define
  them, so app overrides lose the cascade) — the seed's `--poodle-recipe-*`
  namespace from g03.005 was the correct pattern and is now the documented
  contract (`docs/architecture/007-appearance-recipe-contract.md`), with a
  generated inventory
  (`packages/svelte/preview/artifacts/recipe-inventory.json`).
- ~~GPUI/Jetstream consumption depth~~ **Resolved (g11.006):**
  machine-guarded handlers — GPUI parents keep state ownership, components
  call the Rust machines for guard decisions and execute emitted effect
  intents. Dialog/Drawer/Menu adopted; remaining families adopt as touched.
  Jetstream deferred per program posture.

## Honesty Rule

Poodle may say:

- the Svelte layer is self-contained today and ready for core extraction
- the consumer rollout machinery from g11.001 exists and worked

Poodle may not say:

- the headless core exists (nothing is extracted yet)
- multi-framework support exists before a second adapter passes the pilot
  bar in `g11.007`
- consumer safety is proven by intent; it is proven per wave by the
  validation matrix

## Promotion Targets

- machine model, package shape, adapter boundary → `docs/architecture/`
  (extend `001-poodle-system-shape.md` or add a new numbered file)
- machine-spec format and per-component machine sections →
  `docs/contracts/components/` template extension
- recipe override surface and stability guarantees → contract update layered
  on spec `026`
- consumer compatibility rule → `docs/contracts/001-working-rules.md` if it
  proves durable beyond this program

## Next Task

Open `docs/roadmaps/g11/README.md` runway. First executable milestone:
`g11.002` machine-spec format and pilot contracts.
