//! SidebarNav — Jetstream sidebar navigation backed by SidebarNavSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SidebarNavSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_radius, resolve_px, tint};

pub fn js_sidebar_nav(spec: &SidebarNavSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Size-driven dimensions from contract
    let item_height = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 1.375,
        poodle_specs::ControlSize::Sm => 1.625,
        poodle_specs::ControlSize::Md => 1.875,
        poodle_specs::ControlSize::Lg => 2.125,
        poodle_specs::ControlSize::Xl => 2.375,
    });
    let item_font = rem_to_px(size_font_rem(effective_size));
    let title_font = rem_to_px(size_font_rem(effective_size) * 0.75);

    // Density-driven spacing from contract
    let (group_gap, item_px, item_py, title_gap) = match spec.density {
        poodle_specs::ControlDensity::Compact => (0.625, 0.5, 0.3125, 0.125),
        poodle_specs::ControlDensity::Default => (0.75, 0.75, 0.375, 0.1875),
        poodle_specs::ControlDensity::Comfortable => (0.875, 0.875, 0.4375, 0.25),
    };

    let pad_y = resolve_px(theme, "space.panel.y");
    let item_color = resolve_color(theme, spec.item_color_token());
    let item_active_color = resolve_color(theme, spec.item_active_color_token());
    let group_title_color = resolve_color(theme, spec.group_title_color_token());
    let separator_color = resolve_color(theme, spec.separator_color_token());
    let accent = resolve_color(theme, spec.active_indicator_color_token());
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let item_radius = ctrl_radius - rem_to_px(0.125);

    let visible_groups = spec.visible_groups();
    let has_multiple_groups = visible_groups.len() > 1;

    // Root: <nav> element represented as div column
    let mut el = ui_element::div()
        .flex_col().gap(rem_to_px(group_gap))
        .pt(pad_y).pb(pad_y);

    for (gi, group) in visible_groups.iter().enumerate() {
        let mut group_el = ui_element::div()
            .flex_col().gap(rem_to_px(0.3125));

        // Group separator (between groups when multiple visible)
        if has_multiple_groups && gi > 0 {
            group_el = group_el
                .border(1.0).border_color(tint(separator_color, 0.54))
                .pt(rem_to_px(group_gap - 0.125));
        }

        // Group title (optional)
        if let Some(ref label) = group.label {
            group_el = group_el.child(
                ui_element::label(&label.to_uppercase())
                    .text_color(group_title_color)
                    .text_size(title_font)
                    .text_weight(700)
                    .pl(rem_to_px(item_px))
                    .pb(rem_to_px(title_gap))
            );
        }

        // Item list
        let mut list = ui_element::div()
            .flex_col().gap(rem_to_px(0.125));

        for item in &group.items {
            let is_active = spec.is_active(&item.value);

            let mut item_el = if item.href.is_some() && !item.is_disabled {
                ui_element::button(&item.label)
            } else {
                ui_element::button(&item.label)
            };

            item_el = item_el
                .min_h(item_height)
                .self_stretch()
                .text_size(item_font)
                .pl(rem_to_px(item_px)).pr(rem_to_px(item_px))
                .pt(rem_to_px(item_py)).pb(rem_to_px(item_py))
                .rounded(item_radius);

            if is_active {
                item_el = item_el
                    .text_color(item_active_color)
                    .text_weight(600)
                    .bg(tint(accent, 0.10));
            } else {
                item_el = item_el
                    .text_color(item_color)
                    .text_weight(500);
            }

            if item.is_disabled {
                item_el = item_el.opacity(0.48);
            } else {
                item_el = item_el.focusable();
            }

            list = list.child(item_el);
        }

        group_el = group_el.child(list);
        el = el.child(group_el);
    }

    el
}
