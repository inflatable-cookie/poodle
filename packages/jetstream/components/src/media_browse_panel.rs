//! MediaBrowsePanel — Jetstream media browse panel backed by MediaBrowsePanelSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MediaBrowsePanelSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_media_browse_panel(spec: &MediaBrowsePanelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let _font_size = rem_to_px(size_font_rem(effective_size));
    let body_font = rem_to_px(0.875);
    let label_font = rem_to_px(0.8125);

    // Density-driven spacing from contract
    let (grid_gap, item_gap, item_pad) = match spec.density {
        poodle_specs::ControlDensity::Compact => (0.375, 0.25, 0.5),
        poodle_specs::ControlDensity::Default => (0.5, 0.375, 0.75),
        poodle_specs::ControlDensity::Comfortable => (0.75, 0.5, 0.875),
    };

    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");
    let panel_bg = resolve_color(theme, "color.background.panel");

    // Root
    let mut el = ui_element::div()
        .flex_col()
        .min_h(rem_to_px(18.0));

    // Loading state
    if spec.loading && spec.items.is_empty() {
        let state = ui_element::div()
            .items_center().justify_center()
            .min_h(rem_to_px(18.0))
            .child(
                ui_element::label("Loading media...")
                    .text_color(text_secondary).text_size(label_font)
            );
        return el.child(state);
    }

    // Error state
    if let Some(ref error) = spec.error {
        let state = ui_element::div()
            .items_center().justify_center()
            .min_h(rem_to_px(18.0))
            .child(
                ui_element::label(error)
                    .text_color(resolve_color(theme, "color.status.danger"))
                    .text_size(label_font)
            );
        return el.child(state);
    }

    // Empty state
    if spec.items.is_empty() {
        let state = ui_element::div()
            .items_center().justify_center()
            .min_h(rem_to_px(18.0))
            .child(
                ui_element::label("No media found")
                    .text_color(text_secondary).text_size(label_font)
            );
        return el.child(state);
    }

    // Ready: render grid
    let mut grid = ui_element::div()
        .flex_row().flex_wrap().gap(rem_to_px(grid_gap));

    for item in &spec.items {
        let panel_bg_tinted = tint(panel_bg, 0.92);
        let mut card = ui_element::div()
            .flex_col().gap(rem_to_px(item_gap))
            .pl(rem_to_px(item_pad)).pr(rem_to_px(item_pad))
            .pt(rem_to_px(item_pad)).pb(rem_to_px(item_pad))
            .border(1.0).border_color(border_subtle)
            .rounded(radius)
            .bg(panel_bg_tinted);

        // Thumbnail placeholder
        card = card.child(
            ui_element::div()
                .self_stretch().min_h(rem_to_px(6.0))
                .bg(panel_bg)
                .rounded(resolve_radius(theme, "radius.control"))
        );

        // Label
        card = card.child(
            ui_element::label(&item.label)
                .text_color(text_primary).text_size(body_font).text_weight(600)
        );

        // Meta (optional)
        if let Some(ref meta) = item.meta {
            card = card.child(
                ui_element::label(meta)
                    .text_color(text_secondary).text_size(label_font)
            );
        }

        grid = grid.child(card);
    }
    el = el.child(grid);

    // Load more
    if spec.has_more {
        let load_label = if spec.loading { "Loading..." } else { "Load more" };
        let actions = ui_element::div()
            .flex_row().justify_center()
            .child(
                ui_element::button(load_label)
                    .text_color(text_primary)
                    .text_size(rem_to_px(0.8125))
                    .focusable()
            );
        el = el.child(actions);
    }

    el
}
