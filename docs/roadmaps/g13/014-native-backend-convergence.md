# g13.014 Native Backend Convergence

Status: closed — superseded by the `g13.008` **revise** verdict
(`docs/roadmaps/g13/pilot-verdict-evidence.md` §7–8). This milestone describes
family-by-family migration to a generative model the verdict declines. It is
retained as evidence and is **not executable**. The replacement runway is
`g13.017`–`g13.020`.
Owner: Poodle core
Depends on: `g13.010`, `g13.011`, `g13.012`, `g13.013`

## Objective

Make GPUI and Jetstream strict interpreters of the shared Rust definition,
render, and node path.

## Scope

- Remove direct Jetstream component implementations and preview compatibility
  branches superseded by the shared path.
- Keep GPUI and Jetstream ownership to node interpretation, runtime input,
  lifecycle, text, accessibility projection, and engine drawing.
- Add checks that reject component-specific composition in backend crates.

## Acceptance

- One native component implementation exists in `poodle-render`.
- Both native previews consume the same scene/component registries.
- Registration parity cannot report green when a runtime renders a placeholder
  or bypasses the shared path.

## Next

`g13.015` makes the new authority mechanically enforceable.
