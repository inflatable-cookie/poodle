//! FilterToolbar — Jetstream filter toolbar backed by FilterToolbarSpec.
//!
//! Layout container matching the Svelte/GPUI FilterToolbar composite:
//!   - Optional header row with collapse toggle, summary text, actions slot
//!   - Grid of filter control children (flex-wrap)
//!   - Optional secondary slot below the grid
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::FilterToolbarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Render a filter toolbar.
///
/// - `children`: filter controls laid out in a responsive grid
/// - `actions`: optional element rendered in the header row
/// - `secondary`: optional element rendered below the grid
pub fn js_filter_toolbar(
    spec: &FilterToolbarSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
    secondary: Option<JsEl>,
) -> JsEl {
    // Contract §8 summary size table (size-scaled label-size).
    let font_size = rem_to_px(spec.summary_font_size_rem());

    // Contract §8 density table: distinct root padding-block / padding-inline,
    // root gap, and controls-grid gap per density.
    let pad_block = rem_to_px(spec.padding_block_rem());
    let pad_inline = rem_to_px(spec.padding_inline_rem());
    let root_gap = match spec.density_gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => resolve_px(theme, spec.gap_token()),
    };
    let header_gap = resolve_px(theme, "space.inline.sm");
    let controls_gap = match spec.density_controls_gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => resolve_px(theme, spec.controls_gap_token()),
    };
    let actions_gap = resolve_px(theme, spec.actions_gap_token());

    let bg = resolve_color(theme, spec.background_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let summary_color = resolve_color(theme, spec.summary_color_token());
    let icon_muted = resolve_color(theme, "color.icon.muted");
    let toggle_size = resolve_px(theme, spec.toggle_size_token());
    let toggle_radius = resolve_radius(theme, spec.toggle_radius_token());

    let is_expanded = spec.is_grid_visible();
    let had_children = !children.is_empty();

    let mut toolbar = ui_element::div()
        .flex_col()
        .gap(root_gap)
        .px(pad_inline)
        .py(pad_block)
        .bg(bg)
        .border(1.0)
        .border_color(border)
        .rounded(radius);

    // ── Header row ──
    let needs_header = spec.collapsible || spec.summary_text.is_some() || actions.is_some();
    if needs_header {
        let mut header = ui_element::div().flex_row().items_center().gap(header_gap);

        // Collapse toggle chevron (chevron-down when expanded, -right when
        // collapsed). Interaction (collapse click) is wired by the preview
        // event loop, not the component.
        if spec.collapsible {
            let chevron = if is_expanded {
                "chevron-down"
            } else {
                "chevron-right"
            };
            header = header.child(
                ui_element::div()
                    .w(toggle_size)
                    .h(toggle_size)
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .rounded(toggle_radius)
                    .child(
                        ui_element::icon(chevron)
                            .w(toggle_size)
                            .h(toggle_size)
                            .text_color(icon_muted),
                    ),
            );
        }

        // Summary text — grows so the actions slot anchors right (Svelte
        // summary `flex: 1`, actions `margin-left: auto`).
        if let Some(ref summary) = spec.summary_text {
            header = header.child(
                ui_element::label(summary)
                    .text_color(summary_color)
                    .text_size(font_size)
                    .grow(),
            );
        } else {
            // Reserve the grow space so actions still anchor right.
            header = header.child(ui_element::div().grow());
        }

        if let Some(actions_el) = actions {
            header = header.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(actions_gap)
                    .child(actions_el),
            );
        }

        toolbar = toolbar.child(header);
    }

    // ── Filter controls grid ──
    if is_expanded && had_children {
        let mut grid = ui_element::div().flex_row().flex_wrap().gap(controls_gap);
        for child in children {
            grid = grid.child(
                ui_element::div()
                    .flex_grow()
                    .min_w(rem_to_px(spec.min_item_width_rem))
                    .child(child),
            );
        }
        toolbar = toolbar.child(grid);
    }

    // ── Secondary slot ──
    if let Some(secondary_el) = secondary {
        let _ = had_children;
        toolbar = toolbar.child(secondary_el);
    }

    toolbar
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ControlDensity;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn expanded_renders_search_chip_summary_and_toggle() {
        let th = theme();
        let spec = FilterToolbarSpec::new()
            .with_summary_text("Showing 24 of 156 items")
            .with_collapsible(true)
            .with_collapsed(false);
        // Stand-in search control + an active filter chip.
        let children = vec![
            ui_element::text_input("", "Search…"),
            ui_element::label("Status: Active"),
        ];
        let el = js_filter_toolbar(&spec, &th, children, None, None);
        let tree = probe(&el, 700.0, 200.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        // Summary (result count) present.
        assert!(
            tree.has_text("Showing 24 of 156 items"),
            "result-count summary missing: {:?}",
            tree.texts()
        );
        // Collapse chevron rendered (expanded → chevron-down).
        assert!(
            tree.has_text("chevron-down"),
            "expanded collapse chevron missing: {:?}",
            tree.texts()
        );
        // Search input + active chip both laid out.
        assert!(tree.has_text("Search…"), "search input missing");
        assert!(tree.has_text("Status: Active"), "active filter chip missing");
    }

    #[test]
    fn collapsed_hides_grid_and_shows_right_chevron() {
        let th = theme();
        let spec = FilterToolbarSpec::new()
            .with_summary_text("3 filters active")
            .with_collapsible(true)
            .with_collapsed(true);
        let children = vec![ui_element::text_input("", "Search…")];
        let el = js_filter_toolbar(&spec, &th, children, None, None);
        let tree = probe(&el, 700.0, 120.0);

        // Collapsed → chevron-right, summary present, grid (search input) hidden.
        assert!(tree.has_text("chevron-right"), "collapsed chevron missing");
        assert!(tree.has_text("3 filters active"), "summary missing when collapsed");
        assert!(
            !tree.has_text("Search…"),
            "controls grid should be hidden when collapsed"
        );
    }

    #[test]
    fn actions_slot_and_clear_all_secondary_render() {
        let th = theme();
        let spec = FilterToolbarSpec::new()
            .with_summary_text("Showing 8 of 24 items")
            .with_collapsible(false);
        let actions = Some(ui_element::label("Refresh"));
        let secondary = Some(ui_element::label("Clear all"));
        let children = vec![ui_element::text_input("", "Search…")];
        let el = js_filter_toolbar(&spec, &th, children, actions, secondary);
        let tree = probe(&el, 700.0, 200.0);

        // Header action (refresh), search control, and clear-all secondary.
        assert!(tree.has_text("Refresh"), "actions slot missing");
        assert!(tree.has_text("Search…"), "search control missing");
        assert!(tree.has_text("Clear all"), "clear-all secondary missing");
    }

    #[test]
    fn density_changes_root_padding() {
        let th = theme();
        let compact = FilterToolbarSpec::new()
            .with_summary_text("x")
            .with_collapsible(false)
            .with_density(ControlDensity::Compact);
        let comfortable = FilterToolbarSpec::new()
            .with_summary_text("x")
            .with_collapsible(false)
            .with_density(ControlDensity::Comfortable);

        let el_c = js_filter_toolbar(&compact, &th, vec![], None, None);
        let el_f = js_filter_toolbar(&comfortable, &th, vec![], None, None);
        let tree_c = probe(&el_c, 400.0, 120.0);
        let tree_f = probe(&el_f, 400.0, 120.0);

        // Comfortable padding-block is larger, so the header content (first
        // child after root) is pushed further down from the root's top edge.
        let header_y = |t: &crate::render_probe::ProbeTree| -> f32 {
            t.nodes
                .iter()
                .find(|n| n.depth == 1)
                .map(|n| n.y - t.nodes[0].y)
                .unwrap_or(0.0)
        };
        let inset_c = header_y(&tree_c);
        let inset_f = header_y(&tree_f);
        assert!(
            inset_f > inset_c,
            "comfortable top inset ({inset_f}) should exceed compact ({inset_c})"
        );
        assert!(compact.padding_inline_rem() < comfortable.padding_inline_rem());
    }
}
