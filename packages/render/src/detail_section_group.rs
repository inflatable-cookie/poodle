//! DetailSectionGroup — multi-section layout.
//!
//! Contract: `docs/contracts/components/detail-section-group.md`
//! Ported from: `packages/jetstream/components/src/detail_section_group.rs`.
//!
//! A wrapping grid (or vertical stack) of detail sections. Gap is
//! density-driven; the column min width resolves from `min_column_width`.
//! The `max_columns` cap (contract §7) is approximated: no percentage
//! flex-basis here, so wrapping + `min_width` keeps columns legible but does
//! not hard-cap their number.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{DetailSectionGroupLayout, DetailSectionGroupSpec};

use crate::presentation::rem_to_px;

/// Build a detail-section-group from its spec + child sections.
pub fn detail_section_group(
    spec: &DetailSectionGroupSpec,
    _theme: &dyn ThemeProvider,
    children: Vec<Node>,
) -> Node {
    let gap = rem_to_px(spec.gap_rem());

    let mut root = Node::container();
    match spec.layout {
        DetailSectionGroupLayout::Stack => {
            // Single vertical column regardless of available width.
            {
                let s = &mut root.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = gap;
            }
            for child in children {
                let mut wrap = Node::container();
                {
                    let s = &mut wrap.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.fill_width = true;
                }
                root = root.child(wrap.child(child));
            }
        }
        DetailSectionGroupLayout::Grid => {
            // Wrapping grid. Each column is at least `min_column_width` wide
            // and grows to fill the row, wrapping when columns no longer fit.
            let column_min = rem_to_px(spec.min_column_width_rem());

            {
                let s = &mut root.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_wrap = true;
                s.descriptor.layout.spacing.gap = gap;
            }
            for child in children {
                let mut wrap = Node::container();
                {
                    let s = &mut wrap.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    // flex: 1 — grow from a zero basis, min width legible.
                    s.flex_grow = Some(1.0);
                    s.flex_basis = Some(0.0);
                    s.min_width = Some(column_min);
                }
                root = root.child(wrap.child(child));
            }
        }
    }
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
