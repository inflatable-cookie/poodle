//! Select — Jetstream select dropdown backed by SelectSpec.
//!
//! Contract: `docs/contracts/foundation/select.md`
//! Reference: `packages/svelte/primitives/src/Select.svelte`
//!
//! Uses SVG chevron icon and overlay() for the dropdown panel.
//! Jetstream always renders a custom dropdown (no native `<select>` support).
//! When `spec.searchable` is true, a search input placeholder is shown at the
//! top of the dropdown panel.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SelectSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_select(spec: &SelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border_color = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");
    let icon_muted = resolve_color(theme, "semantic.color.icon.muted");

    // Hover state
    let border_c: Color = border_color.into();
    let text_primary: Color = text_color.into();
    let hover_border = border_c.mix(text_primary, 0.78);

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());
    let display = selected
        .and_then(|v| spec.options.iter().find(|o| o.value == v).map(|o| o.label.as_str()))
        .or(spec.placeholder.as_deref())
        .unwrap_or("Select...");

    let display_color = if selected.is_some() { text_color } else { muted };

    // Searchable: show search icon instead of just label for the trigger
    let trigger_icon = if spec.shows_search_input() {
        "search"
    } else {
        "chevron-down"
    };

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border_color)
        .rounded(radius)
        .h(height)
        .pl(pad_x).pr(pad_x - rem_to_px(0.25))
        .flex_row().items_center()
        .gap(rem_to_px(0.25))
        .focusable()
        .cursor_pointer()
        .hover(|s| s.border_color(hover_border));

    el = el.child(
        ui_element::label(display)
            .text_color(display_color)
            .text_size(font_size)
            .grow()
    );

    // SVG indicator icon (chevron-down for standard, search for searchable)
    el = el.child(
        ui_element::icon(trigger_icon)
            .w(icon_size).h(icon_size)
            .text_color(icon_muted)
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
