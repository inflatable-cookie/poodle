//! ColorPicker — Jetstream color picker trigger backed by ColorPickerSpec.
//!
//! Contract: `docs/contracts/components/color-picker.md`
//! Reference: `packages/svelte/components/src/ColorPicker.svelte`
//!
//! Renders the trigger button with color preview swatch and optional
//! inline hex text input.  The popover surface (gradient pad, hue/alpha
//! sliders, mode toggle, inputs, swatches) is overlay-managed at runtime.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ColorPickerSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// Build the trigger-level element for a ColorPicker.
pub fn js_color_picker(spec: &ColorPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let trigger_size = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let border_color = resolve_color(theme, spec.border_token());
    let trigger_radius = resolve_radius(theme, spec.trigger_radius_token());
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");

    // Trigger border is 62% opacity of border-default per contract
    let trigger_border = tint(border_color, 0.62);

    // Color preview swatch inside the trigger.
    // The actual color comes from the spec value; at runtime the host
    // maps the hex string to a Color.  Here we use accent-base as the
    // representative fill — the runtime overlay replaces it with the
    // real parsed color.
    let accent_fill: jetstream_runtime::game_ui::Color =
        resolve_color(theme, "color.accent.base").into();
    let preview = ui_element::div()
        .grow()
        .rounded(rem_to_px(0.125))
        .bg(accent_fill);

    let trigger = ui_element::div()
        .w(trigger_size)
        .h(trigger_size)
        .border(1.0)
        .border_color(trigger_border)
        .rounded(trigger_radius)
        .overflow_hidden()
        .cursor_pointer()
        .focusable()
        .child(preview);

    // Root: inline-flex column containing trigger + optional text input
    let mut root = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.5))
        .child(trigger);

    // Optional inline hex text input
    if spec.show_input {
        let input_display = spec.current_value().unwrap_or("#6366f1");
        let input_color = if spec.current_value().is_some() { text_color } else { muted };

        root = root.child(
            ui_element::div()
                .h(trigger_size)
                .pl(pad_x)
                .pr(pad_x)
                .border(1.0)
                .border_color(border_color)
                .rounded(trigger_radius)
                .bg(resolve_color(theme, "color.background.surface"))
                .flex_row()
                .items_center()
                .child(
                    ui_element::label(input_display)
                        .text_color(input_color)
                        .text_size(font_size),
                ),
        );
    }

    // Disabled state
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity).disabled(true);
    }

    root
}
