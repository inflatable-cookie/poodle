//! OrderBy — Jetstream sort control backed by OrderBySpec.
//!
//! Contract: `docs/contracts/components/order-by.md`
//! Reference: `packages/svelte/components/src/OrderBy.svelte`
//!
//! Anatomy (single flex-row items): popover wrapper → root (trigger wrap with
//! trigger content + integrated ghost reset IconButton) → dialog surface (list
//! of items, each = drag handle + label + direction-toggle IconButton + remove
//! IconButton; or empty text; plus an add-field Select). Sizes match the
//! corrected size table; the move-up/move-down buttons and the footer "Clear
//! all" button are gone (drag + Alt-arrow reorder is the contract's reorder
//! path; the reset `×` clears all).
//!
//! Interaction (menu open/close, selection, drag, Alt-arrow reorder) lives in
//! the preview event loop, not the component — render-only, build/probe-verified.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonVariant, ChoiceOption, ControlSize, IconButtonSpec, OrderBySpec, OrderByTriggerVariant,
    SelectSpec, SortDirection,
};

use crate::icon_button::js_icon_button;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_padding_x_offset_rem};
use crate::select::js_select;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

pub fn js_order_by(spec: &OrderBySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Size table (matches corrected contract §8) ────────────────────────────
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    });
    let label_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.5625,
        ControlSize::Sm => 0.625,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    });
    let summary_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let trigger_pad_x = rem_to_px(match effective_size {
        ControlSize::Xs => 0.5,
        ControlSize::Lg => 1.0,
        ControlSize::Xl => 1.125,
        _ => 0.75 + size_padding_x_offset_rem(effective_size),
    });

    // ── Density (trigger gap only) ────────────────────────────────────────────
    let trigger_gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => 0.5,
        poodle_specs::ControlDensity::Comfortable => 0.625,
    });

    // ── Panel/list/item spacing (contract §8) ─────────────────────────────────
    let root_gap = rem_to_px(0.375);
    let panel_gap = rem_to_px(0.375);
    let list_gap = rem_to_px(0.25);
    let item_gap = rem_to_px(0.375);
    let item_pad_x = rem_to_px(0.5);
    let item_pad_y = rem_to_px(0.3125);
    let item_label_font = rem_to_px(0.8125);

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = resolve_color(theme, spec.field_text_token());
    let text_secondary = resolve_color(theme, spec.label_color_token());
    let muted = resolve_color(theme, spec.muted_color_token());
    let border = resolve_color(theme, spec.field_border_token());
    let item_border = resolve_color(theme, spec.item_border_token());
    let surface = resolve_color(theme, spec.field_fill_token());
    let elevated = resolve_color(theme, spec.field_hover_fill_token());
    let accent = resolve_color(theme, spec.active_fill_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let surface_radius = resolve_radius(theme, spec.surface_radius_token());
    // Item bg: color-mix(surface 90%, elevated) per contract.
    let item_bg = color_mix(surface, elevated, 0.90);

    // ── Trigger ───────────────────────────────────────────────────────────────
    let mut root = ui_element::div().flex_col().gap(root_gap).min_w(0.0);

    match spec.trigger_variant {
        OrderByTriggerVariant::Icon => {
            root = root.child(js_icon_button(
                &IconButtonSpec::new()
                    .with_icon("arrow-up-down")
                    .with_aria_label(spec.aria_label.clone())
                    .with_tooltip(spec.aria_label.clone())
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(effective_size)
                    .with_expanded(spec.is_open)
                    .with_controls("order-by-surface")
                    .with_disabled(spec.is_disabled),
                theme,
            ));
        }
        OrderByTriggerVariant::Summary => {
            let mut trigger = ui_element::div()
                .flex_row()
                .items_center()
                .gap(trigger_gap)
                .grow()
                .min_w(0.0)
                .min_h(trigger_h)
                .pl(trigger_pad_x)
                .pr(trigger_pad_x)
                .rounded(radius)
                .border_1()
                .border_color(border)
                .bg(surface);

            if !spec.compact {
                trigger = trigger.child(
                    ui_element::label("SORT BY")
                        .text_color(text_secondary)
                        .text_size(label_font)
                        .text_weight(500)
                        .letter_spacing_em(0.05),
                );
            }

            let summary_is_placeholder = !spec.has_value();
            trigger = trigger.child(
                ui_element::label(&spec.summary_text())
                    .text_color(if summary_is_placeholder {
                        muted
                    } else {
                        text_primary
                    })
                    .text_size(summary_font)
                    .grow()
                    .min_w(0.0)
                    .text_ellipsis()
                    .whitespace_nowrap(),
            );

            if spec.show_clear_button && spec.has_value() {
                trigger = trigger.child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_aria_label("Clear sort")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(effective_size)
                        .with_disabled(spec.is_disabled),
                    theme,
                ));
            }

            trigger = trigger.child(
                ui_element::icon("chevron-down")
                    .w(summary_font)
                    .h(summary_font)
                    .text_color(text_secondary),
            );
            root = root.child(trigger);
        }
    }

    // ── Dialog surface (rendered inline when open) ───────────────────────────
    if spec.is_open {
        let current_value = spec.current_value();
        // Contract: the open overlay panel is a `dialog`.
        let mut panel = ui_element::div().flex_col().gap(panel_gap)
            .aria_role(jetstream_ui::accesskit::Role::Dialog);

        if matches!(spec.trigger_variant, OrderByTriggerVariant::Icon) {
            let mut header = ui_element::div()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(rem_to_px(0.5))
                .pl(rem_to_px(0.25))
                .pr(rem_to_px(0.25))
                .child(
                    ui_element::label("Sort order")
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.75)),
                );
            if spec.show_clear_button && spec.has_value() {
                header = header.child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_aria_label("Clear sort")
                        .with_tooltip("Clear sort")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Xs)
                        .with_disabled(spec.is_disabled),
                    theme,
                ));
            }
            panel = panel.child(header);
        }

        if current_value.is_empty() {
            panel = panel.child(
                ui_element::label("No sort fields")
                    .text_color(text_secondary)
                    .text_size(rem_to_px(0.75)),
            );
        } else {
            let mut list = ui_element::div().flex_col().gap(list_gap);
            for item in current_value.iter() {
                let field_label = spec.active_label(&item.key);
                let (dir_icon, dir_tooltip, dir_word) = match item.direction {
                    SortDirection::Asc => ("arrow-up", "Asc", "ascending"),
                    SortDirection::Desc => ("arrow-down", "Desc", "descending"),
                };

                let row = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .pl(item_pad_x)
                    .pr(item_pad_x)
                    .pt(item_pad_y)
                    .pb(item_pad_y)
                    .rounded(radius)
                    .border_1()
                    .border_color(item_border)
                    .bg(item_bg)
                    // Drag handle (focusable button carrying the braille glyph).
                    .child(
                        ui_element::button("⠿")
                            .min_w(rem_to_px(1.5))
                            .min_h(rem_to_px(1.5))
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .text_color(muted)
                            .text_size(rem_to_px(0.75))
                            .focusable(),
                    )
                    // Field label (single-line ellipsis).
                    .child(
                        ui_element::label(&field_label)
                            .grow()
                            .min_w(0.0)
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .text_color(text_primary)
                            .text_size(item_label_font),
                    )
                    // Direction toggle (xs ghost IconButton).
                    .child(js_icon_button(
                        &IconButtonSpec::new()
                            .with_icon(dir_icon)
                            .with_aria_label(format!("{field_label}: {dir_word}. Click to toggle."))
                            .with_tooltip(dir_tooltip)
                            .with_variant(poodle_specs::ButtonVariant::Ghost)
                            .with_size(ControlSize::Xs)
                            .with_disabled(spec.is_disabled),
                        theme,
                    ))
                    // Remove (xs ghost IconButton, no danger tone).
                    .child(js_icon_button(
                        &IconButtonSpec::new()
                            .with_icon("x")
                            .with_aria_label(format!("Remove {field_label}"))
                            .with_tooltip("Remove")
                            .with_variant(poodle_specs::ButtonVariant::Ghost)
                            .with_size(ControlSize::Xs)
                            .with_disabled(spec.is_disabled),
                        theme,
                    ));

                list = list.child(row);
            }
            panel = panel.child(list);
        }

        // Add-field Select (hidden when no fields remain or maxFields reached).
        let can_add_more = spec
            .max_fields
            .map(|max| spec.active_count() < max)
            .unwrap_or(true);
        if can_add_more && !spec.available_fields().is_empty() {
            let options: Vec<ChoiceOption> = spec
                .available_fields()
                .into_iter()
                .map(|field| {
                    ChoiceOption::new(field.resolved_key().to_string(), field.label.clone())
                })
                .collect();
            let mut select_spec = SelectSpec::new(options)
                .with_placeholder("+ Add field")
                .with_size(effective_size)
                .with_density(spec.density);
            select_spec.aria_label = Some("Add sort field".to_string());
            select_spec.is_disabled = spec.is_disabled;

            panel = panel.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .child(js_select(&select_spec, theme)),
            );
        }

        // Dialog surface chrome.
        let _ = accent; // accent reserved for the drop-target highlight (drag-time only)
        root = root.child(
            ui_element::div()
                .min_w(rem_to_px(14.0))
                .max_w(rem_to_px(24.0))
                .rounded(surface_radius)
                .border_1()
                .border_color(item_border)
                .bg(elevated)
                .pl(rem_to_px(0.375))
                .pr(rem_to_px(0.375))
                .pt(rem_to_px(0.375))
                .pb(rem_to_px(0.375))
                .child(panel),
        );
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    crate::aria::with_aria_label(root, Some(spec.aria_label.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{OrderByField, SortField};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn fields() -> Vec<SortField> {
        vec![
            SortField::new("title", "Title"),
            SortField::new("updated", "Updated").with_default_direction(SortDirection::Desc),
            SortField::new("created", "Created"),
        ]
    }

    #[test]
    fn empty_open_shows_no_sort_fields_and_no_reset() {
        let el = js_order_by(
            &OrderBySpec::new().with_fields(fields()).with_open(true),
            &theme(),
        );
        let tree = crate::render_probe::probe(&el, 320.0, 240.0);
        assert!(
            tree.has_text("No sort fields"),
            "empty text wrong: {:?}",
            tree.texts()
        );
        // No reset clear button when value is empty (Svelte hides it).
        assert!(
            !tree.has_text("×"),
            "reset should be hidden when empty: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("SORT BY"));
    }

    #[test]
    fn populated_item_shows_label_and_direction_icon() {
        let spec = OrderBySpec::new()
            .with_fields(fields())
            .with_open(true)
            .with_value(vec![
                OrderByField::new("updated", SortDirection::Desc),
                OrderByField::new("title", SortDirection::Asc),
            ]);
        let el = js_order_by(&spec, &theme());
        let tree = crate::render_probe::probe(&el, 320.0, 320.0);

        // Field labels present, single-row (no "Ascending"/"Descending" text line).
        assert!(tree.has_text("Updated") && tree.has_text("Title"));
        assert!(
            !tree.has_text("Ascending") && !tree.has_text("Descending"),
            "item must be single-row with no direction text: {:?}",
            tree.texts()
        );
        // Direction-toggle icons rendered (arrow-up for asc, arrow-down for desc).
        assert!(
            tree.has_text("arrow-up") && tree.has_text("arrow-down"),
            "direction icons missing: {:?}",
            tree.texts()
        );
        // Drag handle glyph present.
        assert!(
            tree.has_text("⠿"),
            "drag handle missing: {:?}",
            tree.texts()
        );
        // No move-up/move-down chevrons, no footer "Clear all".
        assert!(!tree.has_text("Clear all"), "footer Clear all must be gone");
    }

    #[test]
    fn summary_text_uses_arrows_not_words() {
        let spec = OrderBySpec::new()
            .with_fields(fields())
            .with_value(vec![OrderByField::new("title", SortDirection::Asc)]);
        let tree = crate::render_probe::probe(&js_order_by(&spec, &theme()), 320.0, 80.0);
        assert!(
            tree.has_text("Title ↑"),
            "summary should read 'Title ↑': {:?}",
            tree.texts()
        );
    }

    #[test]
    fn show_clear_button_false_suppresses_trigger_reset() {
        let spec = OrderBySpec::new()
            .with_fields(fields())
            .with_value(vec![OrderByField::new("title", SortDirection::Asc)])
            .with_show_clear_button(false);
        let tree = crate::render_probe::probe(&js_order_by(&spec, &theme()), 320.0, 80.0);
        assert!(
            !tree.has_text("x"),
            "reset icon should be hidden when show_clear_button=false: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn icon_trigger_moves_reset_into_open_panel() {
        let spec = OrderBySpec::new()
            .with_fields(fields())
            .with_value(vec![OrderByField::new("title", SortDirection::Asc)])
            .with_trigger_variant(OrderByTriggerVariant::Icon)
            .with_open(true);
        let tree = crate::render_probe::probe(&js_order_by(&spec, &theme()), 320.0, 240.0);
        assert!(
            tree.has_text("arrow-up-down"),
            "sort trigger icon missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Sort order"),
            "icon popover header missing: {:?}",
            tree.texts()
        );
        assert_eq!(
            tree.texts().into_iter().filter(|text| *text == "x").count(),
            2,
            "panel reset and row remove icons should both render: {:?}",
            tree.texts()
        );
        assert!(
            !tree.has_text("SORT BY"),
            "summary trigger must be hidden in icon mode"
        );
    }
}
