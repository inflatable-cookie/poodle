//! IconButton — Jetstream icon-only button backed by IconButtonSpec.
//!
//! Contract: `docs/contracts/foundation/icon-button.md`
//! Reference: `packages/svelte/primitives/src/IconButton.svelte`
//!
//! Uses SVG icon() for the icon glyph and Color::mix for hover/active.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::IconButtonSpec;

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, resolve_supporting_visual_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_icon_button(spec: &IconButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    // Hover state: subtle background
    let surface: Color = resolve_color(theme, "semantic.color.background.surface").into();
    let elevated: Color = resolve_color(theme, "semantic.color.background.elevated").into();
    let hover_bg = surface.mix(elevated, 0.84);

    let icon_name = spec.icon.as_deref().unwrap_or("help-circle");

    // Use empty button label — icon is a child
    let mut el = ui_element::button("")
        .h(height).w(height) // square
        .rounded(radius)
        .text_color(text_color)
        .flex_row().items_center().justify_center()
        .focusable()
        .cursor_pointer()
        .hover(|s| s.bg(hover_bg))
        .child(
            ui_element::icon(icon_name)
                .w(icon_size)
                .h(icon_size)
                .text_color(text_color)
        );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
