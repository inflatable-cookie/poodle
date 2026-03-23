# 2026-03-11 Research Alignment For Token And Contract Foundation

## Changed

- reviewed the active `docs/research/` surface before making further planning
  changes so the architecture/spec/roadmap layer would not drift from the
  parallel research thread
- aligned `docs/architecture/001-poodle-system-shape.md` to the active research
  program by adding explicit research inputs and an implementation-gate rule
- aligned `docs/architecture/002-token-system-and-package-layout.md` to the
  current token-system research by shifting the planned schema toward
  `primitives/`, `semantic/`, `modes/`, and `metadata/`
- updated `docs/specs/001-token-source-and-artifact-contract.md` so the token
  spec explicitly names DTCG as the source format, Style Dictionary as the
  initial emission baseline, and generated Rust artifacts as the GPUI posture
- strengthened `g01.002` through `g01.006` with explicit research inputs and
  more concrete execution checklists derived from the current translation memos
  and source hubs
- left the research source files themselves untouched to avoid colliding with
  the other active thread while still absorbing their current conclusions into
  the main planning surface

## Validation

- `git diff --check`

## Remaining

- decide whether to formalize a dedicated token-schema architecture note or let
  `002-token-system-and-package-layout.md` continue to carry that detail
- promote the now-research-backed token decisions into actual package scaffolds
  when implementation begins
- continue the research program for Underlay and workstation-shell patterns

## Next Task

Open `docs/roadmaps/g01/002-token-system-and-artifact-emission.md` and
`docs/roadmaps/g01/003-token-artifact-emission-themes-and-density-modes.md` and
turn the research-backed checklist items into actual package scaffolding plus a
first DTCG schema slice.
