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

use poodle_node::{LayoutDirection, Node};
use poodle_specs::{DetailSectionGroupLayout, DetailSectionGroupSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Build a detail-section-group from its spec + child sections.
pub fn detail_section_group(
    spec: &DetailSectionGroupSpec,
    ctx: &RenderContext<'_>,
    children: Vec<Node>,
) -> Node {
    let gap = rem_to_px(spec.gap_rem(ctx.resolve_density(spec.density)));

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
            // Match the old GPUI tier's capped wrapping grid: the percentage
            // seed admits at most `max_columns`, then flex-grow distributes
            // the remaining row width evenly.
            let column_min = rem_to_px(spec.min_column_width_rem());
            let columns = spec.max_columns.clamp(2, 5) as f32;
            let basis = 1.0 / columns - 0.01;

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
                    // GPUI's node backend maps width_pct to `relative(...)`,
                    // preserving the old percentage flex-basis geometry.
                    s.flex_grow = Some(1.0);
                    s.flex_shrink_zero = true;
                    s.width_pct = Some(basis);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_columns_seeds_each_grid_cell_with_the_old_percentage_width() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let spec = DetailSectionGroupSpec::new().with_max_columns(2);
        let node = detail_section_group(
            &spec,
            &ctx,
            vec![Node::container(), Node::container(), Node::container()],
        );

        assert_eq!(node.children.len(), 3);
        for cell in &node.children {
            assert_eq!(cell.style.width_pct, Some(0.49));
            assert_eq!(cell.style.flex_grow, Some(1.0));
            assert!(cell.style.flex_shrink_zero);
        }
    }
}
