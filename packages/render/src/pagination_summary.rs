//! PaginationSummary — the "Showing X–Y of Z" line beside a pager.
//!
//! Ported from: `packages/jetstream/components/src/pagination_summary.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::{ControlSize, PaginationSummarySpec, SemanticControlSizeRole};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

pub fn pagination_summary(spec: &PaginationSummarySpec, theme: &dyn ThemeProvider) -> Node {
    let text_color = theme.resolve_color("color.text.secondary");

    // PaginationSummarySpec has no size/size_role fields — use defaults.
    let effective_size =
        resolve_semantic_size(ControlSize::default(), SemanticControlSizeRole::default());
    let font_size = rem_to_px(size_font_rem(effective_size));

    // Built in the spec so all three targets say the same thing (contract §7).
    let mut label = Node::text(&spec.summary_text());
    label.style.descriptor.text_color = Some(text_color);
    label.style.text_size = Some(font_size);
    label
}
