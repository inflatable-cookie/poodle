# poodle-ir

Versioned, serializable Rust-authored component and scene IR for Poodle's
renderer-independent definitions — data and validation only.

`poodle-ir` is the source-of-truth vocabulary for component and scene
definitions: identity, first-class shared enumerated types with per-component
permitted subsets, props, controlled state, events, anatomy parts,
state-derived attributes, axes, token and recipe references, accessibility
intent, adapter capabilities, keyboard tables, VisualState projections,
conformance vectors, and scenes. It does not render, does not generate code,
and depends on no framework, DOM, GPUI, Jetstream, or `poodle-node` types.

Governing references:

- `docs/specs/063-rust-authored-component-and-scene-ir.md` (IR-01–IR-12)
- `docs/roadmaps/g13/pilot-expressiveness-corpus.md` (129 requirements;
  `CROSS-*`, `BTN-*`, `RNG-*`, `TXT-*`, `SHELL-*`, `NEG-*`)
- `docs/roadmaps/g13/batch-cards/011-poodle-ir-schema-core.md`

This crate is a pre-1.0 source preview and is not yet published to crates.io.
Its only in-repo dependency is `poodle-tokens`, used to resolve token
references; serialization is serde.
