//! CardToggleGroup — multi-select card group in a capped-column grid.
//!
//! Contract: `docs/contracts/components/card-toggle-group.md`
//! Ported from: `packages/jetstream/components/src/card_toggle_group.rs`.
//!
//! Options lay out in rows of `column_count()` cells (1–4); a short final
//! row is padded with flex spacers so card widths stay aligned across rows.
//! Recipe reconciled to the old GPUI tier
//! (`packages/gpui/components/src/composites/card_toggle_group.rs`): the
//! density-table grid gap, spec-helper fonts, Card-composed selection
//! treatment, and focusable option cells carrying the toggle handler.

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

    // Density-driven grid gap (contract §7 density table) + Card body rhythm.
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
        card_spec = card_spec.with_aria_label(option.title.clone());
        // The old Card's body slot wraps content in a flex-growing box.
        let mut body_slot = Node::container();
        body_slot.style.descriptor.layout.direction = LayoutDirection::Row;
        body_slot.style.flex_grow = Some(1.0);
        let option_card = card(&card_spec, theme, vec![body_slot.child(body)]);

        // Wrap each Card so it can grow within the grid. The cell is the
        // focusable activation target; a disabled option dims and shows the
        // not-allowed cursor instead of wiring the toggle.
        let mut option_el = flex1_cell();
        option_el.style.min_width = Some(0.0);
        option_el.interaction.focusable = true;
        let mut option_el = option_el.child(option_card);
        if is_option_disabled {
            option_el.style.descriptor.opacity = disabled_opacity;
            option_el.style.descriptor.cursor = CursorHint::NotAllowed;
        } else {
            option_el.style.descriptor.cursor = CursorHint::Pointer;
            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let value = option.value.clone();
                option_el.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::CardToggleOption;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn options() -> Vec<CardToggleOption> {
        vec![
            CardToggleOption::new("alpha", "Alpha").with_description("First option"),
            CardToggleOption::new("beta", "Beta"),
            CardToggleOption::new("gamma", "Gamma"),
        ]
    }

    /// The option cells are the focusable wrappers around each card.
    fn cells(node: &Node) -> Vec<&Node> {
        fn walk<'a>(node: &'a Node, out: &mut Vec<&'a Node>) {
            if node.interaction.focusable {
                out.push(node);
            }
            for child in &node.children {
                walk(child, out);
            }
        }
        let mut out = Vec::new();
        walk(node, &mut out);
        out
    }

    #[test]
    fn grid_gap_follows_the_density_ladder() {
        // Contract §7 density table (rem_to_px = rem * 16):
        // compact 0.5rem · default 0.75rem · comfortable 1rem.
        let cases = [
            (poodle_specs::ControlDensity::Compact, 8.0),
            (poodle_specs::ControlDensity::Default, 12.0),
            (poodle_specs::ControlDensity::Comfortable, 16.0),
        ];
        for (density, expected) in cases {
            let spec = CardToggleGroupSpec::new(options()).with_density(density);
            let node = card_toggle_group(&spec, &theme(), None);
            assert_eq!(
                node.style.descriptor.layout.spacing.gap, expected,
                "root gap for {density:?}"
            );
            assert_eq!(
                node.children[0].style.descriptor.layout.spacing.gap, expected,
                "row gap for {density:?}"
            );
        }
    }

    #[test]
    fn title_and_description_fonts_follow_the_size_ladder() {
        // Contract §7 size tables: title xs 0.6875 · sm 0.75 · md 0.875 ·
        // lg 1 · xl 1.125; description xs 0.625 · sm 0.6875 · md 0.75 ·
        // lg 0.875 · xl 0.9375.
        let cases = [
            (poodle_specs::ControlSize::Xs, 11.0, 10.0),
            (poodle_specs::ControlSize::Sm, 12.0, 11.0),
            (poodle_specs::ControlSize::Md, 14.0, 12.0),
            (poodle_specs::ControlSize::Lg, 16.0, 14.0),
            (poodle_specs::ControlSize::Xl, 18.0, 15.0),
        ];
        let theme = theme();
        let text_primary = theme.resolve_color("color.text.primary");
        let text_secondary = theme.resolve_color("color.text.secondary");
        for (size, expected_title, expected_description) in cases {
            let spec = CardToggleGroupSpec::new(options()).with_size(size);
            let node = card_toggle_group(&spec, &theme, None);

            let title = node
                .find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "Alpha"),
                )
                .expect("title text");
            assert_eq!(
                title.style.text_size,
                Some(expected_title),
                "title font for {size:?}"
            );
            assert_eq!(title.style.text_weight, Some(600));
            assert_eq!(title.style.descriptor.text_color, Some(text_primary));

            let description = node
                .find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "First option"),
                )
                .expect("description text");
            assert_eq!(
                description.style.text_size,
                Some(expected_description),
                "description font for {size:?}"
            );
            assert_eq!(
                description.style.descriptor.text_color,
                Some(text_secondary)
            );
        }
    }

    #[test]
    fn selected_card_carries_the_accent_border_and_option_label() {
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");
        let spec = CardToggleGroupSpec::new(options()).with_values(vec!["alpha".to_string()]);
        let node = card_toggle_group(&spec, &theme, None);

        // Each card is labelled with its option title; selection owns the
        // border through the composed Card primitive.
        let alpha_card = node
            .find(&|n| n.a11y.label.as_deref() == Some("Alpha"))
            .expect("alpha card");
        assert_eq!(alpha_card.style.descriptor.border.color, accent);
        let beta_card = node
            .find(&|n| n.a11y.label.as_deref() == Some("Beta"))
            .expect("beta card");
        assert_ne!(beta_card.style.descriptor.border.color, accent);
    }

    #[test]
    fn toggling_an_option_reports_its_value_through_the_node_handler() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));
        let spec = CardToggleGroupSpec::new(options());
        let node = card_toggle_group(&spec, &theme(), Some(on_change));

        let cells = cells(&node);
        assert_eq!(cells.len(), 3, "one focusable cell per option");
        let beta = cells
            .iter()
            .find(|c| c.has_text("Beta"))
            .expect("beta cell");
        (beta
            .interaction
            .on_activate
            .as_ref()
            .expect("beta is activatable"))();
        assert_eq!(seen.lock().unwrap().as_slice(), ["beta"]);
    }

    #[test]
    fn enabled_options_keep_the_old_tiers_pointer_without_a_handler() {
        let node = card_toggle_group(&CardToggleGroupSpec::new(options()), &theme(), None);
        assert!(cells(&node)
            .iter()
            .all(|cell| cell.style.descriptor.cursor == CursorHint::Pointer));
    }

    #[test]
    fn a_disabled_option_dims_and_shows_the_not_allowed_cursor() {
        let theme = theme();
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let mut opts = options();
        opts[2] = opts[2].clone().with_disabled(true);
        let spec = CardToggleGroupSpec::new(opts);
        let node = card_toggle_group(&spec, &theme, Some(Arc::new(|_: &str| {})));

        let cells = cells(&node);
        let gamma = cells
            .iter()
            .find(|c| c.has_text("Gamma"))
            .expect("gamma cell");
        assert!(gamma.interaction.on_activate.is_none());
        assert_eq!(gamma.style.descriptor.opacity, disabled_opacity);
        assert!(matches!(
            gamma.style.descriptor.cursor,
            CursorHint::NotAllowed
        ));

        let alpha = cells
            .iter()
            .find(|c| c.has_text("Alpha"))
            .expect("alpha cell");
        assert!(alpha.interaction.on_activate.is_some());
        assert!(matches!(alpha.style.descriptor.cursor, CursorHint::Pointer));
    }

    #[test]
    fn a_disabled_group_dims_the_root_and_wires_nothing() {
        let theme = theme();
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let spec = CardToggleGroupSpec::new(options()).with_disabled(true);
        let node = card_toggle_group(&spec, &theme, Some(Arc::new(|_: &str| {})));
        assert_eq!(node.style.descriptor.opacity, disabled_opacity);
        assert!(cells(&node)
            .iter()
            .all(|c| c.interaction.on_activate.is_none()));
    }

    #[test]
    fn a_short_final_row_is_padded_with_spacers() {
        // 3 options in 2 columns: rows of 2 and 1, the short row padded so
        // card widths stay aligned across rows.
        let spec = CardToggleGroupSpec::new(options()).with_columns(2);
        let node = card_toggle_group(&spec, &theme(), None);
        assert_eq!(node.children.len(), 2, "two rows");
        assert_eq!(node.children[0].children.len(), 2, "full first row");
        assert_eq!(node.children[1].children.len(), 2, "padded second row");
        // The spacer carries no card.
        assert!(!node.children[1].children[1].has_text("Gamma"));
    }
}
