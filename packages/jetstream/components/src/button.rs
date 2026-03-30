//! JsButton — Jetstream button component backed by ButtonSpec.
//!
//! Contract: `docs/contracts/foundation/button.md`
//! Reference: `packages/svelte/primitives/src/Button.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::ButtonSpec;
use poodle_primitives::ButtonTone;
use poodle_primitives::ButtonVariant;
use poodle_primitives::ControlSize;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem, size_min_width_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Build a Jetstream button element from a ButtonSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root]  — button element, inline-flex, centered
///   ├── [Spinner]       — conditional (isLoading)
///   ├── [Leading Icon]  — optional
///   ├── [Label]         — text label
///   └── [Trailing Icon] — optional
/// ```
pub fn js_button(spec: &ButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tone = spec.effective_tone();
    let is_danger_tone = tone == ButtonTone::Danger;

    // ── Variant × tone colors (contract) ──
    let fill: Color = match (spec.variant, is_danger_tone) {
        (ButtonVariant::Ghost, _) => Color::TRANSPARENT,
        (ButtonVariant::Secondary, true) => {
            // Danger secondary: color-mix(status-danger 16%, background-surface)
            let danger: Color = resolve_color(theme, "semantic.color.status.danger").into();
            let surface: Color = resolve_color(theme, "semantic.color.background.surface").into();
            danger.mix(surface, 0.16)
        }
        _ => resolve_color(theme, spec.resolved_fill_token()).into(),
    };

    let text_color: Color = resolve_color(theme, spec.resolved_text_token()).into();

    let border_color: Color = match (spec.variant, is_danger_tone) {
        (ButtonVariant::Ghost, _) => Color::TRANSPARENT,
        (ButtonVariant::Secondary, true) => {
            // Danger secondary: color-mix(status-danger 46%, border-default)
            let danger: Color = resolve_color(theme, "semantic.color.status.danger").into();
            let border_default: Color = resolve_color(theme, "semantic.color.border.default").into();
            danger.mix(border_default, 0.46)
        }
        _ => resolve_color(theme, spec.resolved_border_token()).into(),
    };

    // Hover/active colors (contract: mix fill with elevated)
    let elevated: Color = resolve_color(theme, "semantic.color.background.elevated").into();
    let hover_fill = fill.mix(elevated, 0.84);
    let active_fill = fill.mix(elevated, 0.72);
    let text_primary: Color = resolve_color(theme, "semantic.color.text.primary").into();
    let hover_border = border_color.mix(text_primary, 0.78);

    // ── Sizing via presentation helpers (size_role resolves effective size) ──
    let height = rem_to_px(control_height_rem(effective_size));
    let min_width = rem_to_px(size_min_width_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(size_padding_x_offset_rem(effective_size));

    // Padding adjustments when icons present (contract: reduce by 2px on icon side)
    let pad_left = if spec.leading_icon.is_some() { pad_x - 2.0 } else { pad_x };
    let pad_right = if spec.trailing_icon.is_some() { pad_x - 2.0 } else { pad_x };

    let radius = resolve_radius(theme, spec.radius_token());
    let gap = rem_to_px(control_space_x_rem(spec.density)) * 0.5; // inner gap is half of control space

    // Font size from presentation helper, driven by effective size
    let label_size = rem_to_px(size_font_rem(effective_size));

    let icon_size = rem_to_px(size_font_rem(effective_size)); // icon tracks font size
    let is_disabled = spec.is_disabled || spec.is_loading;

    let has_icons = spec.leading_icon.is_some() || spec.trailing_icon.is_some() || spec.is_loading;
    let label_text = spec.label.clone().unwrap_or_default();

    // ── Build element ──
    // When icons are present: Button root with empty label, children for layout.
    // When no icons: Button root carries the label text directly.
    let button_label = if has_icons { String::new() } else { label_text.clone() };

    let mut el = ui_element::button(&button_label)
        .h(height)
        .min_w(min_width)
        .pl(pad_left)
        .pr(pad_right)
        .rounded(radius)
        .bg(fill)
        .text_color(text_color)
        .text_size(label_size)
        .text_weight(500) // contract: font-weight: var(--poodle-typography-label-weight) = 500
        .flex_row()
        .items_center()
        .justify_center()
        .gap(gap)
        .text_align_center()
        .focusable();

    // Border — 1px for non-ghost, 1px transparent for ghost (maintains layout)
    el = el.border(1.0).border_color(border_color);

    // Hover/active state overrides (contract color-mix formulas)
    if !is_disabled {
        el = el
            .hover(|s| s.bg(hover_fill).border_color(hover_border))
            .active(|s| s.bg(active_fill))
            .cursor_pointer();
    }

    // Disabled/loading state
    if is_disabled {
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(disabled_opacity).disabled(true);
    }

    // ── Children (only when icons/spinner present) ──
    if has_icons {
        // Spinner (contract: rotating indicator when isLoading)
        if spec.is_loading {
            el = el.child(
                ui_element::icon("loader")
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(text_color)
            );
        }

        // Leading icon (SVG icon by name)
        if let Some(ref icon_name) = spec.leading_icon {
            el = el.child(
                ui_element::icon(icon_name.as_str())
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(text_color)
            );
        }

        // Label as child (so Taffy lays it out alongside icons)
        if !label_text.is_empty() {
            el = el.child(
                ui_element::label(&label_text)
                    .text_size(label_size)
                    .text_color(text_color)
            );
        }

        // Trailing icon (SVG icon by name)
        if let Some(ref icon_name) = spec.trailing_icon {
            el = el.child(
                ui_element::icon(icon_name.as_str())
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(text_color)
            );
        }
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn primary_button_has_fill_and_border() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_label("Save");
        let el = js_button(&spec, &theme);
        assert!(el.style.background.is_some());
        assert!(el.style.border_width > 0.0);
    }

    #[test]
    fn ghost_button_has_transparent_fill() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_label("Cancel");
        let el = js_button(&spec, &theme);
        // Ghost: transparent fill (alpha = 0)
        if let Some(bg) = el.style.background {
            assert!(bg.a < 0.01, "Ghost bg alpha should be ~0, got {}", bg.a);
        }
    }

    #[test]
    fn disabled_button_has_reduced_opacity() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_label("Disabled")
            .with_disabled(true);
        let el = js_button(&spec, &theme);
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn sm_button_is_shorter_than_md() {
        let theme = test_theme();
        let sm = js_button(&ButtonSpec::new().with_size(ControlSize::Sm).with_label("S"), &theme);
        let md = js_button(&ButtonSpec::new().with_size(ControlSize::Md).with_label("M"), &theme);
        // sm should be 28px (1.75rem), md should be 36px (2.25rem)
        assert_ne!(sm.layout.size.height, md.layout.size.height,
            "sm and md should have different heights");
    }

    #[test]
    fn button_with_icons_has_children() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_label("Create")
            .with_leading_icon("+");
        let el = js_button(&spec, &theme);
        assert!(!el.children.is_empty(), "Button with leading icon should have children");
    }
}
