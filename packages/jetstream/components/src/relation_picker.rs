//! RelationPicker — Jetstream relation picker backed by RelationPickerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{BrowseState, CheckboxSpec, RelationPickerSpec, SkeletonSpec};

use crate::checkbox::js_checkbox;
use crate::presentation::{
    control_height_rem, control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::skeleton::js_skeleton;
use crate::theme_ext::{resolve_color, resolve_radius, tint};

/// Default number of skeleton rows shown in the loading state.
const DEFAULT_LOADING_ROW_COUNT: usize = 3;

pub fn js_relation_picker(spec: &RelationPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let desc_font = rem_to_px(size_font_rem(effective_size) - 0.125);
    let icon_size = rem_to_px(size_font_rem(effective_size));
    let item_px = rem_to_px(control_space_x_rem(spec.density));
    let item_py = rem_to_px(panel_space_y_rem(spec.density) - 0.375);
    let item_gap = rem_to_px(0.5);
    let row_gap = rem_to_px(0.125);
    let row_h = rem_to_px(control_height_rem(effective_size));
    let search_gap = rem_to_px(0.5);
    let search_px = rem_to_px(0.625);
    let search_py = rem_to_px(0.375);
    let search_radius = resolve_radius(theme, "radius.control");

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");
    let surface = resolve_color(theme, "color.background.surface");

    let mut el = ui_element::div().flex_col().gap(row_gap);

    // ── Search input ─────────────────────────────────────────────
    // Show search bar whenever we have a query string or when state is
    // not loading/error (i.e. the picker is usable). Mirrors Svelte which
    // always renders the TextInput in the flat picker path.
    let show_search = !matches!(spec.state, BrowseState::Loading | BrowseState::Error);
    if show_search {
        let query_text = if spec.query.is_empty() {
            "Search…"
        } else {
            spec.query.as_str()
        };
        let text_color = if spec.query.is_empty() {
            text_secondary
        } else {
            text_primary
        };

        let search_row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(search_gap)
            .pl(search_px).pr(search_px).pt(search_py).pb(search_py)
            .border(1.0).border_color(border)
            .rounded(search_radius)
            .bg(tint(surface, 0.6))
            .child(
                ui_element::icon("search")
                    .w(icon_size).h(icon_size)
                    .text_color(text_secondary)
            )
            .child(
                ui_element::label(query_text)
                    .text_color(text_color)
                    .text_size(font_size)
                    .grow()
            );

        el = el.child(search_row);
    }

    // ── Loading state ────────────────────────────────────────────
    if spec.state == BrowseState::Loading {
        let skel_spec = SkeletonSpec::new();
        for _ in 0..DEFAULT_LOADING_ROW_COUNT {
            let row = ui_element::div()
                .flex_row().items_center().gap(item_gap)
                .pl(item_px).pr(item_px)
                .min_h(row_h)
                .child(js_skeleton(&skel_spec, theme).grow());
            el = el.child(row);
        }
        return el;
    }

    // ── Filter items by query (case-insensitive substring on label) ──
    let query_lower = spec.query.trim().to_lowercase();
    let filtered: Vec<_> = spec
        .items
        .iter()
        .filter(|item| {
            if query_lower.is_empty() {
                true
            } else {
                item.label.to_lowercase().contains(&query_lower)
                    || item
                        .description
                        .as_deref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || item
                        .meta
                        .as_deref()
                        .map(|m| m.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            }
        })
        .collect();

    // ── Empty state ──────────────────────────────────────────────
    if filtered.is_empty() {
        el = el.child(
            ui_element::div()
                .flex_row().items_center().justify_center()
                .pl(item_px).pr(item_px).pt(item_py).pb(item_py)
                .child(
                    ui_element::label("No results")
                        .text_color(text_secondary)
                        .text_size(font_size)
                )
        );
        return el;
    }

    // ── Item rows ────────────────────────────────────────────────
    for item in &filtered {
        let is_selected = spec.selected_ids.contains(&item.id);

        // Item row background: accent tint when selected
        let row_bg = if is_selected {
            tint(accent, 0.10)
        } else {
            tint(surface, 0.86)
        };
        let row_border = if is_selected {
            tint(accent, 0.60)
        } else {
            border
        };

        // Checkbox via js_checkbox
        let cb_spec = CheckboxSpec::new().with_checked(is_selected);
        let checkbox = js_checkbox(&cb_spec, theme);

        // Label + optional description stacked vertically
        let mut copy_col = ui_element::div()
            .flex_col().gap(rem_to_px(0.25)).grow()
            .child(
                ui_element::label(&item.label)
                    .text_color(text_primary)
                    .text_size(font_size)
            );

        if let Some(ref desc) = item.description {
            copy_col = copy_col.child(
                ui_element::label(desc)
                    .text_color(text_secondary)
                    .text_size(desc_font)
            );
        } else if let Some(ref meta) = item.meta {
            copy_col = copy_col.child(
                ui_element::label(meta)
                    .text_color(text_secondary)
                    .text_size(desc_font)
            );
        }

        let row = ui_element::div()
            .flex_row().items_center().gap(item_gap)
            .pl(item_px).pr(item_px).pt(item_py).pb(item_py)
            .min_h(row_h)
            .border(1.0).border_color(row_border)
            .rounded(search_radius)
            .bg(row_bg)
            .child(checkbox)
            .child(copy_col);

        el = el.child(row);
    }

    el
}
