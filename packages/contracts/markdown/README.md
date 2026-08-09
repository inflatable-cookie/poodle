# poodle-markdown

Poodle's shared Markdown block model for native renderers.

The crate normalizes `pulldown-cmark` output into the same renderer-neutral
blocks that the web core derives from `marked`. Unsupported HTML degrades to
text instead of disappearing.

This crate is a pre-1.0 source preview and is not yet published to crates.io.
