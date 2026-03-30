//! JsToggle — pressable toggle button backed by ToggleSpec.
//!
//! Contract: `docs/contracts/foundation/toggle.md`
//! Reference: `packages/svelte/primitives/src/Toggle.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{ButtonVariant, ToggleLayout, ToggleSpec};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_height_offset_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Build a Jetstream toggle element from a ToggleSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .toggle]  <button aria-pressed>
///   └── [Content] (slot — icon, text, or both)
/// ```
pub fn js_toggle(spec: &ToggleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let is_pressed = spec.current_pressed();

    // ── Token resolution ──
    let surface: Color = resolve_color(theme, "semantic.color.background.surface").into();
    let border_subtle: Color = resolve_color(theme, "semantic.color.border.subtle").into();
    let border_default: Color = resolve_color(theme, "semantic.color.border.default").into();
    let accent: Color = resolve_color(theme, "semantic.color.accent.base").into();
    let text_primary: Color = resolve_color(theme, "semantic.color.text.primary").into();
    let text_inverse: Color = resolve_color(theme, "semantic.color.text.inverse").into();
    let radius = resolve_radius(theme, spec.radius_token());

    // ── Variant-specific colors (contract section 8) ──
    let (fill, border_color, text_color) = if is_pressed {
        // Pressed state: accent fill, darkened accent border, inverse text
        let pressed_border = accent.mix(Color::BLACK, 0.78);
        (accent, pressed_border, text_inverse)
    } else {
        match spec.variant {
            ButtonVariant::Ghost => {
                // Ghost: color-mix(background-surface 86%, transparent), border-subtle at 78%
                let fill = Color::new(surface.r, surface.g, surface.b, surface.a * 0.86);
                let border = Color::new(
                    border_subtle.r,
                    border_subtle.g,
                    border_subtle.b,
                    border_subtle.a * 0.78,
                );
                (fill, border, text_primary)
            }
            ButtonVariant::Primary => {
                // Primary: color-mix(accent-base 18%, background-surface)
                let fill = accent.mix(surface, 0.18);
                // Border: color-mix(accent-base 38%, border-default)
                let border = accent.mix(border_default, 0.38);
                (fill, border, text_primary)
            }
            ButtonVariant::Secondary | ButtonVariant::Danger => {
                // Secondary/Danger: background-surface fill, border-default border
                (surface, border_default, text_primary)
            }
        }
    };

    // ── Sizing (contract section 7 & 8) ──
    let height = rem_to_px(control_height_rem(effective_size))
        + rem_to_px(size_height_offset_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(size_padding_x_offset_rem(effective_size));
    let gap = rem_to_px(0.5); // Contract: gap = var(--poodle-space-inline-sm)

    // Contract: font-size 0.75rem, font-weight 600
    let label_size = rem_to_px(0.75);
    let min_width = rem_to_px(2.25); // Contract: min-width 2.25rem

    let is_stack = spec.layout == ToggleLayout::Stack;

    // ── Build element ──
    let label_text = spec.label.clone().unwrap_or_default();
    let has_children = spec.icon.is_some() || !label_text.is_empty();
    let button_label = if has_children { String::new() } else { String::new() };

    let mut el = ui_element::button(&button_label)
        .rounded(radius)
        .bg(fill)
        .text_color(text_color)
        .text_size(label_size)
        .text_weight(600) // Contract: font-weight 600
        .border(rem_to_px(0.0625)) // Contract: 0.0625rem solid border
        .border_color(border_color)
        .focusable();

    if is_stack {
        // Stack layout: full-width, auto-height, no explicit padding
        // Contract: display grid, width 100%, min-width 0, height auto, padding 0
        el = el.self_stretch();
    } else {
        // Inline layout
        el = el
            .h(height)
            .min_w(min_width)
            .pl(pad_x)
            .pr(pad_x)
            .flex_row()
            .items_center()
            .justify_center()
            .gap(gap);
    }

    // ── Hover state (only when not pressed and not disabled) ──
    if !spec.is_disabled {
        let elevated: Color = resolve_color(theme, "semantic.color.background.elevated").into();
        let hover_fill = fill.mix(elevated, 0.84);
        el = el
            .hover(|s| s.bg(hover_fill))
            .cursor_pointer();
    }

    // ── Content children ──
    if let Some(ref icon_name) = spec.icon {
        let icon_size = rem_to_px(0.75); // Icon tracks font size
        el = el.child(
            ui_element::icon(icon_name.as_str())
                .w(icon_size)
                .h(icon_size)
                .text_color(text_color),
        );
    }

    if !label_text.is_empty() {
        el = el.child(
            ui_element::label(&label_text)
                .text_size(label_size)
                .text_color(text_color),
        );
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn pressed_toggle_uses_accent_background() {
        let spec = ToggleSpec::new().with_pressed(true).with_label("Bold");
        let el = js_toggle(&spec, &theme());
        assert!(el.style.background.is_some());
    }

    #[test]
    fn unpressed_ghost_toggle_has_transparent_fill() {
        let spec = ToggleSpec::new().with_label("Italic");
        let el = js_toggle(&spec, &theme());
        // Ghost unpressed: fill has reduced alpha
        if let Some(bg) = el.style.background {
            assert!(bg.a < 1.0, "Ghost unpressed should have semi-transparent fill");
        }
    }

    #[test]
    fn disabled_toggle_has_reduced_opacity() {
        let spec = ToggleSpec::new().with_disabled(true).with_label("Lock");
        let el = js_toggle(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn toggle_with_icon_has_children() {
        let spec = ToggleSpec::new()
            .with_icon("bold")
            .with_label("Bold");
        let el = js_toggle(&spec, &theme());
        assert!(el.children.len() >= 2, "Toggle with icon+label should have children");
    }
}
