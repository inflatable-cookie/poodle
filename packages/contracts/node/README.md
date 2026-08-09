# poodle-node

The renderer-neutral tree emitted by `poodle-render` and interpreted by native
backends.

Nodes carry layout, resolved style, accessibility metadata, typed content,
and interaction intent. They are a render vocabulary, not a second component
API: semantic inputs remain in `poodle-specs` and platform behavior remains in
the backend.

This crate is a pre-1.0 source preview and is not yet published to crates.io.
