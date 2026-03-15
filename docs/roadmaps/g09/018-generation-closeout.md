# g09.018 — Generation Closeout

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.017
Primary repos: `pug`

## Goals

- [ ] verify all g09 milestones are complete
- [ ] confirm the GPUI preview app matches the Svelte preview in structure,
  navigation, and visual fidelity
- [ ] document any deferred items for future generations

## Execution Checklist

- [ ] verify all 18 milestones are marked complete
- [ ] verify zero hand-built mockup specimens remain in the codebase — search
  for raw `div()` chains in specimen files that don't use `Pug*` components
- [ ] verify GPUI preview has: 4 section tabs, sidebar navigation,
  per-component pages, display controls, route state, 6-screen demo
- [ ] verify component count: all Svelte-appropriate components have GPUI
  implementations
- [ ] verify specimen count: every component has a dedicated specimen file
- [ ] run `cargo check` on all GPUI crates — zero errors
- [ ] run `cargo test` on all GPUI crates — all tests pass
- [ ] verify parity report artifacts are up to date
- [ ] document any items deferred to future generations
- [ ] update generation-index.md with g09 completion status

## Acceptance Criteria

- [ ] all milestones complete with no blocked items
- [ ] GPUI preview app is a first-class deliverable matching Svelte quality
- [ ] zero hand-built mockup specimens remain
- [ ] all tests pass, all builds clean
- [ ] generation-index.md updated

## Next Task

g09 is complete. g10 (Jetstream preview app build-out) can proceed.
