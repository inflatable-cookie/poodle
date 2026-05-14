# g11 Svelte Modernization And Consumer Rollout

Status: active
Updated: 2026-05-14

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

## Active Runway

- `g11.001` active — full modernization program, consumer matrix, wave process,
  validation posture, and first execution order

## Next Task

Open `g11/001-svelte-modernization-and-consumer-rollout.md`.
