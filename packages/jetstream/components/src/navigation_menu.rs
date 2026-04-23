//! NavigationMenu — Jetstream nav menu backed by NavigationMenuSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::NavigationMenuSpec;

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, tint};

pub fn js_navigation_menu(spec: &NavigationMenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // 4px vertical — fixed chrome-compact padding for a top-bar nav.
    let pad_y = rem_to_px(0.25);

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let active_bg = tint(accent, 0.12);
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    let current = spec.current_value();

    let mut el = ui_element::div().flex_row().items_center().gap(rem_to_px(0.25));

    for entry in &spec.items {
        let is_active = current == Some(entry.value.as_str());
        let text_color = if is_active { text_primary } else { text_muted };

        let mut btn = ui_element::button(&entry.label)
            .text_color(text_color)
            .text_size(font_size)
            .text_weight(if is_active { 600 } else { 400 })
            .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
            .focusable()
            .cursor_pointer();

        if is_active {
            btn = btn.bg(active_bg);
        }

        if entry.is_disabled {
            btn = btn.opacity(disabled_opacity).disabled(true);
        }

        el = el.child(btn);
    }

    el
}
