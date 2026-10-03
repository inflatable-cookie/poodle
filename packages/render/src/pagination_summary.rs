//! PaginationSummary — the "Showing X–Y of Z" line beside a pager.
//!
//! Ported from: `packages/jetstream/components/src/pagination_summary.rs`.

use poodle_node::{Node, NodeRole};
use poodle_specs::PaginationSummarySpec;

use crate::context::RenderContext;

pub fn pagination_summary(spec: &PaginationSummarySpec, ctx: &RenderContext<'_>) -> Node {
    let text_color = ctx.theme().resolve_color("color.text.secondary");
    let font_size = ctx.theme().resolve_space("typography.body.size");

    // Built in the spec so all three targets say the same thing (contract §7).
    let mut label = Node::text(spec.summary_text());
    label.style.descriptor.text_color = Some(text_color);
    label.style.text_size = Some(font_size);
    label.a11y.role = Some(NodeRole::Status);
    label.a11y.label = Some(spec.accessible_label());
    label
}
