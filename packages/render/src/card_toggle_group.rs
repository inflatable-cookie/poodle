//! CardToggleGroup — multi-select card group in a capped-column grid.
//!
//! Contract: `docs/contracts/components/card-toggle-group.md`
//! Ported from: `packages/jetstream/components/src/card_toggle_group.rs`.
//!
//! Options lay out in rows of `column_count()` cells (1–4); a short final
//! row is padded with flex spacers so card widths stay aligned across rows.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CursorHint, LayoutDirection, Node};
use poodle_specs::{CardSpec, CardToggleGroupSpec};

use crate::card::card;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};

fn flex1_cell() -> Node {
    let mut n = Node::container();
    let s = &mut n.style;
    // Explicit Row (see switch.rs).
    s.descriptor.layout.direction = LayoutDirection::Row;
    s.flex_grow = Some(1.0);
    s.flex_basis = Some(0.0);
    n
}

pub fn card_toggle_group(
    spec: &CardToggleGroupSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7 size scale — via the spec helpers.
    let title_font = rem_to_px(CardToggleGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardToggleGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap + Card body rhythm.
    let grid_gap = rem_to_px(control_space_x_rem(spec.density));
    let body_gap = rem_to_px(0.25);

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");

    // Contract §6: rows of `column_count()` cells.
    let cols = spec.column_count();
    let mut cells: Vec<Node> = Vec::new();

    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_option_disabled = spec.disabled || option.disabled;

        // Card body: title + optional description.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = body_gap;
        }
        let mut title = Node::text(option.title.clone());
        title.style.text_size = Some(title_font);
        title.style.text_weight = Some(600);
        title.style.descriptor.text_color = Some(text_primary);
        let mut body = body.child(title);
        if let Some(description) = &option.description {
            let mut d = Node::text(description.clone());
            d.style.text_size = Some(description_font);
            d.style.descriptor.text_color = Some(text_secondary);
            body = body.child(d);
        }

        // Compose the Card primitive — selected state owns the fill/border.
        let mut card_spec = CardSpec::new().interactive();
        if is_selected {
            card_spec = card_spec.selected();
        }
        let option_card = card(&card_spec, theme, vec![body]);

        // Wrap each Card so it can grow within the grid; dim per-item disabled.
        let mut option_el = flex1_cell();
        option_el.style.min_width = Some(0.0);
        let mut option_el = option_el.child(option_card);
        if is_option_disabled {
            option_el.style.descriptor.opacity = disabled_opacity;
        } else if let Some(handler) = &on_change {
            let handler = Arc::clone(handler);
            let value = option.value.clone();
            option_el.style.descriptor.cursor = CursorHint::Pointer;
            option_el.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        }

        cells.push(option_el);
    }

    // Assemble rows; pad a short final row with flex spacers.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = grid_gap;
    }
    let mut iter = cells.into_iter();
    let mut remaining = spec.options.len();
    while remaining > 0 {
        let take = cols.min(remaining);
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.spacing.gap = grid_gap;
        }
        for _ in 0..take {
            if let Some(cell) = iter.next() {
                row = row.child(cell);
            }
        }
        for _ in take..cols {
            row = row.child(flex1_cell());
        }
        root = root.child(row);
        remaining -= take;
    }

    if spec.disabled {
        root.style.descriptor.opacity = disabled_opacity;
    }

    root
}
