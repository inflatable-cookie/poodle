//! ListGrid — responsive card/tile layout.
//!
//! Contract: `docs/contracts/components/list-grid.md`
//! Ported from: `packages/jetstream/components/src/list_grid.rs`.
//! `flex_wrap` with per-cell min-width + flex:1 approximates CSS
//! `auto-fill` / `minmax`.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{ListGridSpec, ListGridVariant};

use crate::presentation::rem_to_px;

/// List grid layout: optional header row, then a responsive grid or stacked
/// column.
pub fn list_grid(
    spec: &ListGridSpec,
    theme: &dyn ThemeProvider,
    header: Option<Node>,
    children: Vec<Node>,
) -> Node {
    let gap = theme.resolve_space(spec.gap_token());
    let min_w = if let Some(em) = spec.min_item_width_em {
        rem_to_px(em)
    } else {
        theme.resolve_space(spec.min_item_width_token())
    };
    let header_gap = theme.resolve_space(ListGridSpec::header_actions_gap_token());
    let header_after = theme.resolve_space(spec.header_margin_bottom_token());

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.self_stretch = true;
    }

    if let Some(h) = header {
        let mut header_row = Node::container();
        {
            let s = &mut header_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = header_gap;
            s.descriptor.layout.spacing.padding.bottom = header_after;
        }
        root = root.child(header_row.child(h));
    }

    let grid = match spec.variant {
        ListGridVariant::Compact => {
            let mut g = Node::container();
            {
                let s = &mut g.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = gap;
                s.self_stretch = true;
            }
            for c in children {
                g = g.child(c);
            }
            g
        }
        ListGridVariant::Default => {
            let mut g = Node::container();
            {
                let s = &mut g.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_wrap = true;
                s.descriptor.layout.spacing.gap = gap;
                s.self_stretch = true;
            }
            for c in children {
                let mut cell = Node::container();
                {
                    let s = &mut cell.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.min_width = Some(min_w);
                    // flex: 1 — grow from zero basis.
                    s.flex_grow = Some(1.0);
                    s.flex_basis = Some(0.0);
                }
                g = g.child(cell.child(c));
            }
            g
        }
    };

    root.child(grid)
}
