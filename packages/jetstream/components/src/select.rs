//! Select — Jetstream select dropdown backed by SelectSpec.
//!
//! Contract: `docs/contracts/components/select.md`
//! Reference: `packages/svelte/components/src/Select.svelte`
//!
//! Closed state: trigger button with chevron-down indicator.
//! Open state: trigger plus option-list overlay panel below it.
//! When `spec.searchable` is true, a search input row appears at
//! the top of the panel.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SelectSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px,
    resolve_semantic_size, resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_select(spec: &SelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let item_gap = rem_to_px(0.5);

    let fill = resolve_color(theme, "color.background.surface");
    let border_color = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.control");
    let surface_radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_placeholder = resolve_color(theme, "color.text.placeholder");
    let icon_muted = resolve_color(theme, "color.icon.muted");
    let panel_fill = resolve_color(theme, spec.overlay_fill_token());

    // Hover: border shifts toward text
    let border_c: Color = border_color.into();
    let text_c: Color = text_primary.into();
    let hover_border = border_c.mix(text_c, 0.78);

    // Trigger display text and colour
    let (display_text, display_color) = match spec.trigger_text() {
        Some(text) => (text, text_primary),
        None => (
            spec.placeholder.as_deref().unwrap_or("Select…"),
            text_placeholder,
        ),
    };

    // ── Trigger ─────────────────────────────────────────────────

    let trigger = build_trigger(
        display_text,
        display_color,
        icon_muted,
        font_size,
        icon_size,
        pad_x,
        height,
        item_gap,
        fill,
        border_color,
        radius,
        hover_border,
        spec.is_disabled,
        theme,
    );

    // Closed state — return trigger only
    if !spec.current_open() {
        return trigger;
    }

    // ── Open state — wrapper + overlay panel ────────────────────

    // Compute panel vertical offset: trigger height + 2px gap
    let panel_top = height + 2.0;

    let panel = build_panel(
        spec,
        theme,
        effective_size,
        font_size,
        icon_size,
        pad_x,
        height,
        item_gap,
        panel_top,
        panel_fill,
        border_color,
        surface_radius,
        text_primary,
        text_secondary,
        text_placeholder,
        icon_muted,
    );

    // Relative wrapper so the absolute panel is positioned relative to the trigger
    ui_element::div()
        .flex_col()
        .relative()
        .child(trigger)
        .child(panel)
}

// ── Trigger builder ──────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn build_trigger(
    display_text: &str,
    display_color: glam::Vec4,
    icon_muted: glam::Vec4,
    font_size: f32,
    icon_size: f32,
    pad_x: f32,
    height: f32,
    item_gap: f32,
    fill: glam::Vec4,
    border_color: glam::Vec4,
    radius: f32,
    hover_border: Color,
    is_disabled: bool,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .h(height)
        .pl(pad_x)
        .pr(pad_x)
        .flex_row()
        .items_center()
        .gap(item_gap)
        .focusable()
        .cursor_pointer()
        .hover(|s| s.border_color(hover_border));

    el = el.child(
        ui_element::label(display_text)
            .text_color(display_color)
            .text_size(font_size)
            .grow()
            .text_ellipsis(),
    );

    el = el.child(
        ui_element::icon("chevron-down")
            .size(icon_size)
            .text_color(icon_muted),
    );

    if is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}

// ── Panel builder ────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn build_panel(
    spec: &SelectSpec,
    theme: &JetstreamThemeProvider,
    effective_size: poodle_specs::ControlSize,
    font_size: f32,
    icon_size: f32,
    pad_x: f32,
    row_height: f32,
    item_gap: f32,
    panel_top: f32,
    panel_fill: glam::Vec4,
    border_color: glam::Vec4,
    surface_radius: f32,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    text_placeholder: glam::Vec4,
    icon_muted: glam::Vec4,
) -> JsEl {
    let panel_py = rem_to_px(0.25);
    let min_width = rem_to_px(10.0);
    let max_height = rem_to_px(15.0);

    let mut panel = ui_element::div()
        .absolute()
        .top(panel_top)
        .left(0.0)
        .bg(panel_fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(surface_radius)
        .shadow_md()
        .min_w(min_width)
        .max_h(max_height)
        .overflow_hidden()
        .flex_col()
        .pt(panel_py)
        .pb(panel_py)
        .overlay();

    // Search input row (when searchable)
    if spec.shows_search_input() {
        let query = spec.search_query.as_deref().unwrap_or("");
        let search_row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(item_gap)
            .pl(pad_x)
            .pr(pad_x)
            .h(row_height)
            .child(
                ui_element::icon("search")
                    .size(icon_size)
                    .text_color(icon_muted),
            )
            .child(if query.is_empty() {
                ui_element::label("Search…")
                    .text_color(text_placeholder)
                    .text_size(font_size)
                    .grow()
            } else {
                ui_element::label(query)
                    .text_color(text_primary)
                    .text_size(font_size)
                    .grow()
            });

        panel = panel.child(search_row);
    }

    // Filter options by search query when searchable
    let current_value = spec.current_value();
    let query_lower = spec
        .search_query
        .as_deref()
        .map(|q| q.to_lowercase());

    let filtered: Vec<&poodle_specs::ChoiceOption> = spec
        .options
        .iter()
        .filter(|opt| {
            if let Some(ref q) = query_lower {
                if !q.is_empty() {
                    return opt.label.to_lowercase().contains(q.as_str());
                }
            }
            true
        })
        .collect();

    if filtered.is_empty() {
        // Empty state
        panel = panel.child(
            ui_element::label(&spec.empty_message)
                .text_color(text_secondary)
                .text_size(font_size)
                .pl(pad_x)
                .pr(pad_x)
                .pt(rem_to_px(0.5))
                .pb(rem_to_px(0.5)),
        );
    } else {
        // Group rendering: collect group headers in encounter order,
        // then render each group's options under a header row.
        let mut seen_groups: Vec<Option<String>> = Vec::new();
        for opt in &filtered {
            let key = opt.group.clone();
            if !seen_groups.contains(&key) {
                seen_groups.push(key);
            }
        }

        for group_key in &seen_groups {
            // Render group header if named
            if let Some(ref name) = group_key {
                let header_py = rem_to_px(0.25);
                let header_font = rem_to_px(size_font_rem(
                    resolve_supporting_visual_size(effective_size),
                ));
                panel = panel.child(
                    ui_element::label(name.as_str())
                        .text_color(text_secondary)
                        .text_size(header_font)
                        .text_weight(600)
                        .pl(pad_x)
                        .pr(pad_x)
                        .pt(header_py)
                        .pb(header_py),
                );
            }

            // Render options for this group
            for opt in filtered.iter().filter(|o| &o.group == group_key) {
                let is_selected = current_value
                    .map(|v| v == opt.value.as_str())
                    .unwrap_or(false);

                let label_color = if opt.is_disabled {
                    text_secondary
                } else {
                    text_primary
                };

                let mut row = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .pl(pad_x)
                    .pr(pad_x)
                    .h(row_height)
                    .cursor_pointer()
                    .focusable();

                row = row.child(
                    ui_element::label(opt.label.as_str())
                        .text_color(label_color)
                        .text_size(font_size)
                        .grow()
                        .text_ellipsis(),
                );

                if is_selected {
                    row = row.child(
                        ui_element::icon("check")
                            .size(icon_size)
                            .text_color(icon_muted),
                    );
                }

                if opt.is_disabled {
                    let opacity = resolve_opacity(theme, "state.opacity.disabled");
                    row = row.opacity(opacity).disabled(true);
                }

                panel = panel.child(row);
            }
        }
    }

    panel
}
