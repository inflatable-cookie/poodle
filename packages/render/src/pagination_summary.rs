//! PaginationSummary — the "Showing X–Y of Z" line beside a pager.
//!
//! Ported from: `packages/jetstream/components/src/pagination_summary.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::PaginationSummarySpec;

pub fn pagination_summary(spec: &PaginationSummarySpec, theme: &dyn ThemeProvider) -> Node {
    let text_color = theme.resolve_color("color.text.secondary");
    let font_size = theme.resolve_space("typography.body.size");

    // Built in the spec so all three targets say the same thing (contract §7).
    let mut label = Node::text(spec.summary_text());
    label.style.descriptor.text_color = Some(text_color);
    label.style.text_size = Some(font_size);
    label
}
