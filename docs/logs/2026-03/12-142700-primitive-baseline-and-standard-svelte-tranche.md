# Primitive Baseline And Standard Svelte Tranche

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- added an explicit primitive-baseline spec in
  `docs/specs/028-primitive-baseline-and-bits-aligned-surface.md` so the true
  Pug primitive set is no longer implied by scattered contracts or whatever the
  preview happened to implement first
- expanded `@pug/svelte-primitives` with a broader standard control and
  feedback tranche: `Button`, `IconButton`, `Switch`, `RadioGroup`, `Select`,
  `TextArea`, `Progress`, `Badge`, `Pill`, `Callout`, and `StatusIndicator`
- updated the primitive package readme and root exports so the shipped Svelte
  surface now reflects a more credible generalized app baseline instead of only
  the original forms slice

## Validation

- `bun run docs:build`
- `git diff --check`

## Risks

- the new tranche still leaves the structural, value-control, and most
  overlay/navigation families incomplete, so the primitive baseline is now
  explicit but not yet fully shipped
- several remaining families will want real Bits-backed wrappers rather than
  native stand-ins if they are going to meet the intended accessibility and
  parity bar cleanly

## Next Task

Implement the remaining structural and value-control primitives as a coherent
family tranche before resuming more composite growth or treating overlay gaps
as acceptable debt.
