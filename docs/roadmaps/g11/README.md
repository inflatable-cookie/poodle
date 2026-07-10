# g11 Svelte Modernization, Headless Core, And Consumer Rollout

Status: active
Updated: 2026-07-10

## Context

The component audit confirmed that the public Svelte layer is still mostly on a
compatibility-first surface: `export let`, `$:`, `createEventDispatcher`,
legacy slots, and compatibility alias props dominate the package. That posture
does not block current consumers, but it does slow modern Svelte 5 adoption and
encourages new downstream code to keep depending on the old seams.

This generation turns that into a systematic program:

- modernize each public Svelte component deliberately
- update every downstream consumer that uses that component
- validate each rollout wave across Poodle, Underlay, the six Underlay-root app
  families, and direct desktop adopters

## Scope

- Public Svelte component API modernization
- Svelte 5 internals modernization where it improves maintainability without
  destabilizing consumers
- Downstream migration across Underlay-owned wrappers and direct app usage
- Consumer inventory, rollout tracking, and migration evidence

## Consumer Priority

1. `underlay`
2. Underlay-root app families:
   - `underlay-reference`
   - `contact-patch`
   - `compli-me`
   - `acowtancy`
   - `songsprout`
   - `loophole/composer`
3. Direct desktop adopters:
   - `finch/app-electron`
   - `soundcheck`
   - `loophole/aura`

## Working Rule

Do not modernize components in isolation and leave consumer drift behind. Each
component wave is only complete when the owning Poodle change and every known
downstream usage are updated or explicitly parked with a documented migration
reason.

## Second Program: Headless Core And Dual-Layer Split

With `g11.001` closed, this generation continues into the dual-layer program
defined in `docs/specs/062-headless-core-and-dual-layer-strategy.md`:

- extract Poodle's in-house behavioral logic (Bits is already gone from the
  dependency tree) into a framework-free state-machine core
- keep the `@poodle/svelte` public surface stable for the 17 source-linked
  consumer apps; interface-invariant swaps by default, g11.001 wave process
  when a surface must change
- productize the appearance-recipe layer so apps can restyle per-app without
  forking components
- mirror the machine layer to Rust for GPUI/Jetstream
- prove multi-framework reach with a React adapter pilot and settle the
  Mitosis question with evidence

## Active Runway

- `g11.001` complete — Svelte modernization program, consumer matrix, wave
  process, validation posture, and audited consumer rollout
- `g11.002` complete — headless machine-spec format and pilot contracts
  (template extension + tabs/popover/checkbox machine specs)
- `g11.003` complete — `@poodle/headless` core package, shared machinery,
  pilot swap (Tabs/Popover/Checkbox) runtime-verified, consumer matrix clean
- `g11.004` planned — full behavioral-component sweep onto core, in waves
- `g11.005` planned — appearance-recipe productization and per-app overrides
  (can start after `g11.004` wave 2)
- `g11.006` planned — Rust headless mirror with shared conformance vectors
- `g11.007` planned — React adapter pilot and Mitosis shell decision

## Next Task

Read `docs/specs/062-headless-core-and-dual-layer-strategy.md`, then open
`g11/002-headless-machine-spec-format-and-pilot-contracts.md`.
