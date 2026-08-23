//! FieldSet — legend, description, and an equal-column field grid.
//!
//! Contract: `docs/contracts/components/field-set.md`
//! Ported from: `packages/jetstream/components/src/field_set.rs`.

use poodle_node::{LayoutDirection, LayoutSizing, Node};
use poodle_specs::FieldSetSpec;

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn field_set(spec: &FieldSetSpec, ctx: &RenderContext<'_>, children: Vec<Node>) -> Node {
    let col_gap = spec
        .column_gap_token()
        .map(|t| ctx.theme().resolve_space(t))
        .unwrap_or(0.0);
    let row_gap = col_gap + rem_to_px(FieldSetSpec::ROW_GAP_EXTRA_REM);
    let legend_color = ctx.theme().resolve_color(spec.legend_color_token());
    let legend_size = rem_to_px(FieldSetSpec::LEGEND_SIZE_REM);

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.style.self_stretch = true;

    if let Some(ref legend) = spec.legend {
        let mut l = Node::text(legend.to_uppercase());
        l.style.descriptor.text_color = Some(legend_color);
        l.style.text_size = Some(legend_size);
        l.style.text_weight = Some(600);
        l.style.letter_spacing_em = Some(0.12);
        l.style.descriptor.layout.spacing.margin.bottom =
            ctx.theme().resolve_space(spec.legend_margin_bottom_token());
        root = root.child(l);
    }

    if let Some(ref description) = spec.description {
        let mut d = Node::text(description);
        d.style.descriptor.text_color = Some(ctx.theme().resolve_color(spec.description_color_token()));
        d.style.text_size = Some(ctx.theme().resolve_space(spec.description_size_token()));
        d.style.descriptor.layout.spacing.margin.bottom =
            ctx.theme().resolve_space(spec.description_margin_bottom_token());
        root = root.child(d);
    }

    // Equal-fraction columns via flex_wrap + per-child flex-basis.
    let cols = spec.columns.max(1);
    let mut grid = Node::container();
    grid.style.descriptor.layout.direction = if cols > 1 {
        LayoutDirection::Row
    } else {
        LayoutDirection::Column
    };
    grid.style.flex_wrap = cols > 1;
    grid.style.descriptor.layout.spacing.gap = if cols > 1 { col_gap } else { row_gap };
    grid.style.fill_width = true;
    grid.style.self_stretch = true;

    for mut child in children {
        // Field controls are full-width within their grid cell in the old
        // GPUI tier; make the slot's cross-axis intent explicit for nodes.
        child.style.fill_width = true;
        let mut wrapper = Node::container();
        // Explicit Row (see switch.rs): the old tier got taffy's Row default.
        wrapper.style.descriptor.layout.direction = LayoutDirection::Row;
        wrapper.style.min_width = Some(0.0);
        // The old builder chains .child() before layout config; children order
        // is what matters and the adapter emits fields, not call order.
        if cols > 1 {
            // GPUI's `.flex_1()` is grow + zero basis, which gives each
            // child an equal share of the wrapping row.
            wrapper.style.flex_grow = Some(1.0);
            wrapper.style.flex_basis = Some(0.0);
        } else {
            wrapper.style.descriptor.layout.width = LayoutSizing::Grow;
            wrapper.style.fill_width = true;
        }
        grid = grid.child(wrapper.child(child));
    }

    root.child(grid)
}
