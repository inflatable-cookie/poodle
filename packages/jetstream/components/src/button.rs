//! JsButton — Jetstream button component backed by ButtonSpec.
//!
//! Contract: `docs/contracts/foundation/button.md`
//! Reference: `packages/svelte/primitives/src/Button.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::ButtonSpec;
use pug_primitives::ButtonVariant;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Build a Jetstream button element from a ButtonSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root]  — button element
///   ├── [Spinner]       — conditional (isLoading)
///   ├── [Leading Icon]  — optional
///   ├── [Label]         — optional
///   └── [Trailing Icon] — optional
/// ```
pub fn js_button(spec: &ButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Token resolution (every value from spec token methods) ──
    let fill = resolve_color(theme, spec.resolved_fill_token());
    let text_color = resolve_color(theme, spec.resolved_text_token());
    let border_color = resolve_color(theme, spec.resolved_border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let height = resolve_px(theme, spec.control_height_token());
    let pad_x = resolve_px(theme, spec.horizontal_padding_token());
    let min_width = resolve_px(theme, spec.control_min_width_token());
    let gap = resolve_px(theme, "semantic.space.inline.sm");
    let label_size = resolve_px(theme, "semantic.typography.label.size");
    let icon_size = resolve_px(theme, spec.icon_size_token());

    let label_text = spec.label.clone().unwrap_or_default();
    let is_ghost = spec.variant == ButtonVariant::Ghost;
    let is_disabled = spec.is_disabled || spec.is_loading;

    // ── Build element — anatomy matches contract ──
    let mut el = ui_element::button(&label_text)
        .h(height)
        .min_w(min_width)
        .px(pad_x)
        .rounded(radius)
        .bg(fill)
        .text_color(text_color)
        .flex_row()
        .items_center()
        .justify_center()
        .gap(gap)
        .text_size(label_size)
        .focusable();

    // Border — ghost variant gets transparent border (contract: border: transparent)
    if is_ghost {
        // Ghost: no visible border per contract
    } else {
        el = el.border_1().border_color(border_color);
    }

    // Disabled state — opacity from token, not hardcoded
    if is_disabled {
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(disabled_opacity).disabled(true);
    }

    // Spinner (contract: conditional, when isLoading)
    if spec.is_loading {
        el = el.child(ui_element::label("⟳").text_size(icon_size));
    }

    // Leading icon (contract: optional, icon wrapper sized from icon_size_token)
    if let Some(ref icon) = spec.leading_icon {
        el = el.child(ui_element::label(icon).text_size(icon_size));
    }

    // Label is carried by Widget::Button, rendered by runtime

    // Trailing icon
    if let Some(ref icon) = spec.trailing_icon {
        el = el.child(ui_element::label(icon).text_size(icon_size));
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&pug_tokens::themes::DARK)
    }

    #[test]
    fn primary_button_has_background_and_border() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_label("Save");
        let el = js_button(&spec, &theme);
        assert!(el.style.background.is_some());
        assert!(el.style.border_width > 0.0);
    }

    #[test]
    fn ghost_button_has_no_border() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_label("Cancel");
        let el = js_button(&spec, &theme);
        assert_eq!(el.style.border_width, 0.0);
    }

    #[test]
    fn disabled_button_is_marked_disabled() {
        let theme = test_theme();
        let spec = ButtonSpec::new()
            .with_label("Disabled")
            .with_disabled(true);
        let el = js_button(&spec, &theme);
        assert!(el.style.disabled);
    }

    #[test]
    fn button_height_resolves_from_token() {
        let theme = test_theme();
        let spec = ButtonSpec::new().with_label("Sized");
        let el = js_button(&spec, &theme);
        // Height = control_height_token = 36.0px (from typed::semantic)
        assert!(el.layout.size.height != taffy::Dimension::auto());
    }

    #[test]
    fn button_gap_resolves_from_token() {
        let theme = test_theme();
        let spec = ButtonSpec::new().with_label("Gap");
        let el = js_button(&spec, &theme);
        // Gap should be SPACE_INLINE_SM = 8.0px (from typed::semantic), not hardcoded 6.0
        let expected_gap = crate::theme_ext::resolve_px(&theme, "semantic.space.inline.sm");
        assert_eq!(el.layout.gap.width, taffy::LengthPercentage::length(expected_gap));
    }
}
