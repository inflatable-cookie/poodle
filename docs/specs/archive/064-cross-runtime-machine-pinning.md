# 064 Cross-Runtime Machine Pinning

Status: retired — follow-up spec 066 also rejected
Updated: 2026-08-15
Owner: Poodle core

## Record

The first g14 runway proposed five independent mechanisms:

- generated TS/Rust machine interfaces
- differential transition traces
- vector completeness
- capability absence registry
- specimen evidence gates

The contract was promoted before the stack worked. Four machine interfaces
merged, then made `docs:machine-shape-drift` fail because the gate did not
follow generated imports. The differential batch never landed. More
importantly, machine equality could not prove component interface,
composition, renderer, specimen, accessibility, or backend parity.

The attempted follow-up moved the useful pieces under component-level cases
and observations in
[066 Executable Component Conformance](../066-executable-component-conformance.md),
which `g14.008` also rejected. Retain machine tests only for the focused claims
they own. The full false-start evidence is archived in
`../../roadmaps/archive/2026-08-14-g14-machine-pinning-false-start.md`.

Do not dispatch work from this document.
