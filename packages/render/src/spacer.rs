//! Spacer — flexible space filler for flex layouts.
//!
//! Contract: `docs/contracts/components/spacer.md`
//! Ported from: `packages/jetstream/components/src/spacer.rs`.

use poodle_node::{LayoutDirection, Node};
use poodle_specs::SpacerSpec;

use crate::context::RenderContext;

/// Every public component renderer receives the construction context
/// (architecture 010); Spacer has no presentation-dependent output, so the
/// context is accepted and unused.
pub fn spacer(spec: &SpacerSpec, _ctx: &RenderContext<'_>) -> Node {
    let mut el = Node::container();
    // Explicit Row (see switch.rs).
    el.style.descriptor.layout.direction = LayoutDirection::Row;
    if spec.grow > 0.0 {
        el.style.flex_fill = true;
    }
    if let Some(size) = spec.min_size {
        el.style.min_width = Some(size);
        el.style.min_height = Some(size);
    }
    el
}
