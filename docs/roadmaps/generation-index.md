# Roadmap Generation Index

- `g01`
  - Status: completed
  - Range: `001` to `014`
  - Notes: repository/program bootstrap, token model, contract system, Svelte
    and GPUI substrates, primitive suite, workstation shells, Underlay bridge,
    and first parity baseline

- `g02`
  - Status: completed
  - Range: `001` to `016`
  - Notes: advanced composites, product/workstation depth, documentation and
    examples, preview/docs cleanup, API cleanup, packaging, and release
    baseline

- `g03`
  - Status: completed
  - Range: `001` to `014`
  - Notes: migration policy, parity automation, docs publishing, downstream
    adoption, ecosystem validation, change control, extension support

- `g04`
  - Status: completed
  - Range: `001` to `018`
  - Notes: Underlay component parity, new component families, feature depth
    for existing components, specialist editing/media surfaces

- `g05`
  - Status: completed
  - Range: `001` to `014`
  - Notes: GPUI foundation, spec crates, cross-runtime parity baseline, demo
    alignment

- `g06`
  - Status: completed
  - Range: `001` to `015`
  - Notes: shared multi-renderer contract layer, crate restructuring, typed
    token resolution, layout intent, event model, style descriptors, renderer
    adapter trait, spec expansion to full component surface

- `g07`
  - Status: completed
  - Range: `001` to `015`
  - Notes: GPUI rendering build-out, adapter crate, theme integration,
    primitive and composite rendering batches, workstation shell updates,
    cross-runtime parity report

- `g08`
  - Status: active
  - Range: `001` to `009`
  - Notes: GPUI production quality — consolidates and replaces the previous
    g08–g13 which contained inflated completion claims (see
    `docs/roadmaps/archive/g08-g11-reference-notes.md` for preserved
    architectural decisions). `g08.001` syncs GPUI implementations with
    current contracts (names, props, token methods may have changed during
    concurrent Svelte refactoring), `g08.002` fixes cross-cutting issues
    (hardcoded disabled opacity, hover colors, geometry values) across 18+
    components, `g08.003` through `g08.005` fix components in three parallel
    batches (high-visibility, input/selection, remaining + broken), `g08.006`
    adds focus rings and ARIA attributes to all interactive components,
    `g08.007` aligns specimen pages to contract definitions, `g08.008`
    performs systematic visual parity verification and produces a delta
    register, and `g08.009` closes the generation. Every milestone begins
    with contract verification since the Svelte side is a concurrent moving
    target.

- `g09`
  - Status: planned
  - Range: `001` to `008`
  - Notes: Jetstream production quality — mirrors g08 for the Jetstream
    rendering target. Currently has 8 real components (all with hardcoded
    dimensions) and ~100 adapter stubs. `g09.001` syncs with contracts and
    assesses feasibility within Jetstream constraints (no SVG, no gradients,
    no transforms, no ARIA), `g09.002` fixes existing 8 components for token
    resolution, `g09.003` and `g09.004` implement feasible missing components,
    `g09.005` removes or documents unfeasible stubs, `g09.006` builds
    specimen pages, `g09.007` performs visual parity verification and
    produces a delta register, and `g09.008` closes the generation.

## What Was Consolidated

The previous g08–g13 (6 generations, 75 planned milestones) were replaced with
g08–g09 (2 generations, 17 milestones) after an audit revealed:

- g08 (old) claimed 117 Jetstream components; only 8 existed
- g09 (old) claimed full GPUI build-out complete; all components were Partial
- g10 (old) claimed 125 Jetstream specimens; only 8 existed
- g11 (old) delivered real workstation contracts but workstation category has
  since been consolidated into primitives/composites
- g12–g13 (old) were superseded by the consolidated g08–g09

Useful architectural decisions and constraint documentation from the old
generations are preserved in `docs/roadmaps/archive/g08-g11-reference-notes.md`.

## Next Task

Execute `g08.001` — sync GPUI component implementations with current contracts
before beginning quality fixes.
