//! Menubar — Jetstream horizontal menu bar backed by MenubarSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MenubarSpec;

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, tint};

pub fn js_menubar(spec: &MenubarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // 4px vertical — fixed chrome-compact padding for a top-bar menu.
    let pad_y = rem_to_px(0.25);

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let open_bg = tint(accent, 0.12);
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    let open_value = spec.current_value();

    let mut el = ui_element::div().flex_row().items_center().gap(rem_to_px(0.125));

    for entry in &spec.items {
        let is_open = open_value == Some(entry.value.as_str());
        let text_color = if is_open { text_primary } else { text_muted };

        let mut btn = ui_element::button(&entry.label)
            .text_color(text_color)
            .text_size(font_size)
            .text_weight(if is_open { 600 } else { 400 })
            .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
            .focusable()
            .cursor_pointer();

        if is_open {
            btn = btn.bg(open_bg);
        }

        if entry.is_disabled {
            btn = btn.opacity(disabled_opacity).disabled(true);
        }

        el = el.child(btn);
    }

    el
}
