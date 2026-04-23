//! SidebarNav — Jetstream sidebar navigation backed by SidebarNavSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SidebarNavSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

pub fn js_sidebar_nav(spec: &SidebarNavSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Size-driven dimensions from token helpers
    let item_height = rem_to_px(control_height_rem(effective_size));
    let item_font = rem_to_px(size_font_rem(effective_size));
    // Group title: caption-sized (0.75× item font, uppercase)
    let title_font = rem_to_px(size_font_rem(effective_size) * 0.75);

    // Density-driven horizontal padding
    let item_px = rem_to_px(control_space_x_rem(spec.density));

    let pad_y = resolve_px(theme, "space.panel.y");
    let group_gap = resolve_px(theme, "space.stack.md");
    let item_gap = rem_to_px(0.125);
    let title_mb = rem_to_px(0.1875);

    let item_color = resolve_color(theme, spec.item_color_token());
    let item_active_color = resolve_color(theme, spec.item_active_color_token());
    let group_title_color = resolve_color(theme, spec.group_title_color_token());
    let separator_color = resolve_color(theme, spec.separator_color_token());
    let accent = resolve_color(theme, spec.active_indicator_color_token());
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
    let ctrl_radius = resolve_radius(theme, "radius.control");
    // Item radius is slightly tighter than the panel's control radius
    let item_radius = (ctrl_radius - rem_to_px(0.125)).max(0.0);

    let visible_groups = spec.visible_groups();
    let has_multiple_groups = visible_groups.len() > 1;

    // Root: <nav> represented as a flex column
    let mut el = ui_element::div()
        .flex_col()
        .gap(group_gap)
        .pt(pad_y)
        .pb(pad_y);

    for (gi, group) in visible_groups.iter().enumerate() {
        let mut group_el = ui_element::div().flex_col().gap(rem_to_px(0.3125));

        // Inter-group divider (1px subtle border above, with extra top spacing)
        if has_multiple_groups && gi > 0 {
            let divider = ui_element::div()
                .h(1.0)
                .self_stretch()
                .bg(tint(separator_color, 0.54))
                .mb(rem_to_px(0.25));
            group_el = group_el.child(divider);
        }

        // Group title label — uppercase, caption-sized, accent-tinted
        if let Some(ref label_text) = group.label {
            let title = ui_element::label(&label_text.to_uppercase())
                .text_color(group_title_color)
                .text_size(title_font)
                .text_weight(700)
                .pl(item_px)
                .mb(title_mb);
            group_el = group_el.child(title);
        }

        // Item list
        let mut list = ui_element::div().flex_col().gap(item_gap);

        for item in &group.items {
            let is_active = spec.is_active(&item.value);

            let mut item_el = ui_element::button(&item.label)
                .min_h(item_height)
                .self_stretch()
                .flex_row()
                .items_center()
                .text_size(item_font)
                .pl(item_px)
                .pr(item_px)
                .rounded(item_radius);

            if is_active {
                item_el = item_el
                    .text_color(item_active_color)
                    .text_weight(600)
                    .bg(tint(accent, 0.10))
                    // Left 2px accent border indicates active item
                    .border_l(2.0)
                    .border_color_left(accent);
            } else {
                item_el = item_el
                    .text_color(item_color)
                    .text_weight(500);
            }

            if item.is_disabled {
                item_el = item_el.opacity(disabled_opacity);
            } else {
                item_el = item_el.focusable().cursor_pointer();
            }

            list = list.child(item_el);
        }

        group_el = group_el.child(list);
        el = el.child(group_el);
    }

    el
}
