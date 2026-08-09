# poodle-layout

Renderer-neutral layout intent for Poodle's native render pipeline.

The crate models sizing, flow, alignment, overflow, gaps, and edges without
depending on a layout engine. Target adapters translate these values into GPUI
or Taffy-compatible structures.

This crate is a pre-1.0 source preview and is not yet published to crates.io.
