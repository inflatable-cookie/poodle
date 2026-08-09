# poodle-specs

Renderer-neutral component inputs, state, and semantic helpers for Poodle's
native targets.

Specs describe what a component means and how its state behaves. They do not
draw UI or depend on GPUI or Jetstream. `poodle-render` consumes these specs and
emits `poodle-node` trees for each backend to interpret.

This crate is a pre-1.0 source preview and is not yet published to crates.io.
Public behavior is defined by the matching files in
[`docs/contracts/components`](../../../docs/contracts/components/README.md).
