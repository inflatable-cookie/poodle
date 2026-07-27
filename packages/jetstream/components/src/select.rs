//! Select — Jetstream select dropdown backed by SelectSpec.
//!
//! Contract: `docs/contracts/components/select.md`
//! Reference: `packages/svelte/components/src/Select.svelte`
//!
//! Closed state: trigger button with chevron-down indicator.
//! Open state: trigger plus option-list overlay panel below it.
//! When `spec.searchable` is true, a search input row appears at
//! the top of the panel.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SelectSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px,
    resolve_semantic_size, resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius};

pub fn js_select(spec: &SelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let item_gap = rem_to_px(0.5);

    let fill = resolve_color(theme, "color.background.surface");
    let base_border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.control");
    let surface_radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_placeholder = resolve_color(theme, "color.text.placeholder");
    let icon_muted = resolve_color(theme, "color.icon.muted");
    let panel_fill = resolve_color(theme, spec.overlay_fill_token());

    // Validation-state border colour. Mirrors GPUI / TextInput: when set,
    // the closed trigger border switches to the matching status colour.
    use poodle_specs::ValidationState;
    let validation_border = match spec.validation_state {
        ValidationState::Invalid => Some(resolve_color(theme, "color.status.danger")),
        ValidationState::Valid => Some(resolve_color(theme, "color.status.success")),
        ValidationState::Pending => Some(resolve_color(theme, "color.accent.base")),
        ValidationState::None => None,
    };
    let border_color = validation_border.unwrap_or(base_border);

    // Hover: border shifts toward text (validation border holds when set),
    // background lifts toward the hover surface.
    let border_c: Color = base_border.into();
    let text_c: Color = text_primary.into();
    let fill_c: Color = fill.into();
    let elevated_c: Color = panel_fill.into();
    let hover_border = validation_border
        .map(Into::into)
        .unwrap_or_else(|| border_c.mix_srgb(text_c, 0.78));
    let hover_fill = fill_c.mix_srgb(elevated_c, 0.5);

    // Trigger display text and colour
    let (display_text, display_color) = match spec.trigger_text() {
        Some(text) => (text, text_primary),
        None => (
            spec.placeholder.as_deref().unwrap_or("Select…"),
            text_placeholder,
        ),
    };

    // Clear button visible when clearable + value selected + enabled.
    let show_clear = spec.clearable && spec.current_value().is_some() && !spec.is_disabled;

    // ── Trigger ─────────────────────────────────────────────────

    let trigger = build_trigger(
        display_text,
        display_color,
        icon_muted,
        text_secondary,
        font_size,
        icon_size,
        pad_x,
        height,
        item_gap,
        fill,
        border_color,
        radius,
        hover_border,
        hover_fill,
        show_clear,
        spec.is_disabled,
        theme,
    );

    // Closed state — return trigger only
    if !spec.current_open() {
        return trigger;
    }

    // ── Open state — wrapper + overlay panel ────────────────────

    // Panel vertical offset: trigger height + stack spacing token
    // (GPUI uses space.stack.sm; matches across targets).
    let panel_top = height + crate::theme_ext::resolve_px(theme, "space.stack.sm");

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
    let root = ui_element::div()
        .flex_col()
        .relative()
        .child(trigger)
        .child(panel);
    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::ComboBox).aria_expanded(spec.open.unwrap_or(false))
}

// ── Trigger builder ──────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn build_trigger(
    display_text: &str,
    display_color: glam::Vec4,
    icon_muted: glam::Vec4,
    text_secondary: glam::Vec4,
    font_size: f32,
    icon_size: f32,
    pad_x: f32,
    height: f32,
    item_gap: f32,
    fill: glam::Vec4,
    border_color: glam::Vec4,
    radius: f32,
    hover_border: Color,
    hover_fill: Color,
    show_clear: bool,
    is_disabled: bool,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    // Hover shifts both border (toward text) and background (toward elevated),
    // matching the contract focus-within treatment direction.
    // Contract (select.css): trigger width: 100% — fill the parent container.
    let mut el = ui_element::div()
        .w_full()
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
        .hover(move |s| s.border_color(hover_border).bg(hover_fill));

    el = el.child(
        ui_element::label(display_text)
            .text_color(display_color)
            .text_size(font_size)
            .grow()
            .text_ellipsis(),
    );

    // Clear button — contract anatomy Clear Button (aria-label "Clear
    // selection"). Rendered as a pill-backed "x" icon before the chevron.
    if show_clear {
        let radius_pill = resolve_radius(theme, "radius.pill");
        let clear_pill: Color = Color::from(text_secondary).with_alpha(0.18);
        el = el.child(
            ui_element::div()
                .bg(clear_pill)
                .rounded(radius_pill)
                .flex_row()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .child(
                    ui_element::icon("x")
                        .size(icon_size)
                        .text_color(text_secondary),
                ),
        );
    }

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
    // Panel dimensions resolve from tokens (GPUI parity). `menu_min_width`
    // prop, when set, overrides the default select min-width.
    let token_min_width = crate::theme_ext::resolve_px(theme, "size.select.minWidth");
    let min_width = spec
        .menu_min_width
        .as_deref()
        .map(parse_css_length_to_px)
        .filter(|w| *w > 0.0)
        .unwrap_or(token_min_width);
    let max_height = crate::theme_ext::resolve_px(theme, "size.menu.maxHeight");

    // Token-accurate `elevation-overlay` (Svelte: var(--poodle-elevation-overlay))
    // resolved from the typed semantic token via the runtime shadow builder
    // (single layer, spread 0; matches GPUI's mapping).
    let mut panel = elevation_overlay(
        ui_element::div()
            .absolute()
            .top(panel_top)
            .left(0.0)
            .bg(panel_fill)
            .border(1.0)
            .border_color(border_color)
            .rounded(surface_radius),
    )
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
                    .cursor_pointer()
                    .focusable();

                // Rows with a description are taller (label + secondary line);
                // plain rows keep the fixed control row height.
                if opt.description.is_none() {
                    row = row.h(row_height);
                }

                // Contract anatomy: Option Label + optional Option Description
                // (Svelte: description color text-secondary, font-size 0.6875rem).
                if let Some(ref description) = opt.description {
                    row = row.child(
                        ui_element::div()
                            .flex_col()
                            .grow()
                            .child(
                                ui_element::label(opt.label.as_str())
                                    .text_color(label_color)
                                    .text_size(font_size)
                                    .text_ellipsis(),
                            )
                            .child(
                                ui_element::label(description.as_str())
                                    .text_color(text_secondary)
                                    .text_size(rem_to_px(0.6875)),
                            ),
                    );
                } else {
                    row = row.child(
                        ui_element::label(opt.label.as_str())
                            .text_color(label_color)
                            .text_size(font_size)
                            .grow()
                            .text_ellipsis(),
                    );
                }

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

/// Parse a CSS length string (e.g. "12rem", "200px") to logical pixels.
/// Returns 0.0 on parse failure. Uses 16px as the rem base (matches `rem_to_px`).
fn parse_css_length_to_px(value: &str) -> f32 {
    let trimmed = value.trim();
    if let Some(num) = trimmed.strip_suffix("rem") {
        num.trim().parse::<f32>().map(rem_to_px).unwrap_or(0.0)
    } else if let Some(num) = trimmed.strip_suffix("px") {
        num.trim().parse::<f32>().unwrap_or(0.0)
    } else {
        trimmed.parse::<f32>().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ChoiceOption, ValidationState};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn fruit_options() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("apple", "Apple"),
            ChoiceOption::new("banana", "Banana"),
            ChoiceOption::new("cherry", "Cherry"),
        ]
    }

    #[test]
    fn closed_trigger_shows_placeholder_and_chevron() {
        let spec = SelectSpec::new(fruit_options()).with_placeholder("Choose a fruit");
        let tree = probe(&js_select(&spec, &theme()), 320.0, 200.0);
        assert!(tree.has_text("Choose a fruit"), "placeholder missing: {:?}", tree.texts());
        assert!(tree.has_text("chevron-down"), "chevron missing: {:?}", tree.texts());
        // Closed: no option rows rendered.
        assert!(!tree.has_text("Apple"), "options leaked when closed: {:?}", tree.texts());
    }

    #[test]
    fn open_renders_all_options() {
        let spec = SelectSpec::new(fruit_options())
            .with_placeholder("Choose a fruit")
            .with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("Apple") && tree.has_text("Banana") && tree.has_text("Cherry"),
            "open options missing: {:?}", tree.texts());
    }

    #[test]
    fn selected_option_renders_check_indicator() {
        let spec = SelectSpec::new(fruit_options())
            .with_value("banana")
            .with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("check"), "selected checkmark missing: {:?}", tree.texts());
    }

    #[test]
    fn searchable_open_renders_search_row() {
        let spec = SelectSpec::new(fruit_options())
            .with_searchable(true)
            .with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("search"), "search icon missing: {:?}", tree.texts());
    }

    #[test]
    fn searchable_query_filters_options() {
        let spec = SelectSpec::new(fruit_options())
            .with_searchable(true)
            .with_search_query("ban")
            .with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("Banana"), "matching option missing: {:?}", tree.texts());
        assert!(!tree.has_text("Apple"), "non-matching option not filtered: {:?}", tree.texts());
    }

    #[test]
    fn empty_query_renders_empty_message() {
        let spec = SelectSpec::new(fruit_options())
            .with_searchable(true)
            .with_search_query("zzz")
            .with_empty_message("No matches")
            .with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("No matches"), "empty message missing: {:?}", tree.texts());
    }

    #[test]
    fn grouped_options_render_headers() {
        let opts = vec![
            ChoiceOption::new("apple", "Apple").with_group("Fruits"),
            ChoiceOption::new("carrot", "Carrot").with_group("Vegetables"),
        ];
        let spec = SelectSpec::new(opts).with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("Fruits") && tree.has_text("Vegetables"),
            "group headers missing: {:?}", tree.texts());
    }

    #[test]
    fn option_description_renders() {
        let opts = vec![ChoiceOption::new("apple", "Apple").with_description("A red fruit")];
        let spec = SelectSpec::new(opts).with_open(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 320.0);
        assert!(tree.has_text("A red fruit"), "option description missing: {:?}", tree.texts());
    }

    #[test]
    fn clearable_with_value_renders_clear_button() {
        let spec = SelectSpec::new(fruit_options())
            .with_value("apple")
            .with_clearable(true);
        let tree = probe(&js_select(&spec, &theme()), 320.0, 200.0);
        // An "x" icon appears in the trigger as the clear affordance.
        assert!(tree.has_text("x"), "clear button icon missing: {:?}", tree.texts());
    }

    #[test]
    fn invalid_validation_recolors_trigger_border() {
        let theme = theme();
        let base = SelectSpec::new(fruit_options()).with_placeholder("Choose");
        let invalid = base.clone().with_validation_state(ValidationState::Invalid);
        // The first node is the trigger; its border color should differ when invalid.
        let el_none = js_select(&base, &theme);
        let el_invalid = js_select(&invalid, &theme);
        assert_ne!(
            el_none.style.border_color, el_invalid.style.border_color,
            "validation state did not recolor the trigger border"
        );
    }
}
