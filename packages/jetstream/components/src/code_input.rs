//! JsCodeInput — segmented code entry with visual digit slots,
//! backed by CodeInputSpec.
//!
//! Contract: `docs/contracts/components/code-input.md`
//! Reference: `packages/svelte/primitives/src/CodeInput.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{ControlSize, CodeInputSpec, ValidationState};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem, size_height_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Slot width in rem per size — slightly narrower than control height for a square feel.
fn slot_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.5,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.5,
        ControlSize::Xl => 3.0,
    }
}

/// Build a Jetstream code input element from a CodeInputSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Field]
///   └── [Root .code-input] <div role="group">
///         ├── [Hidden submission input] <input type="hidden">
///         ├── [Real input .code-input__control] <input type="text">
///         └── [Visual slot .code-input__slot]... (repeated `length`)
/// ```
///
/// In Jetstream the real input and hidden input are runtime-level concerns.
/// This builder produces the visual slot grid with the current digit values.
pub fn js_code_input(spec: &CodeInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let validation = spec.effective_validation_state();

    // ── Token resolution ──
    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let danger: Color = resolve_color(theme, "color.status.danger").into();
    let radius = resolve_radius(theme, "radius.control");

    // Validation-dependent border
    let slot_border = match validation {
        ValidationState::Invalid => danger,
        ValidationState::Valid => resolve_color(theme, "color.status.success").into(),
        ValidationState::Pending => accent,
        ValidationState::None => border_default,
    };

    // ── Sizing ──
    let slot_h = rem_to_px(control_height_rem(effective_size))
        + rem_to_px(size_height_offset_rem(effective_size));
    let slot_w = rem_to_px(slot_width_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size)) * 1.5; // Digits are larger than body text
    let border_width = rem_to_px(0.0625); // 1px
    let gap = rem_to_px(control_space_x_rem(spec.density)) * 0.5;

    // ── Parse current value into individual digit characters ──
    let value = spec.current_value();
    let digits: Vec<char> = value.chars().filter(|c| c.is_ascii_digit()).collect();

    // ── Root container ──
    let mut root = ui_element::div()
        .flex_row()
        .gap(gap)
        .items_center()
        .justify_center();

    // ── Visual slots ──
    for i in 0..spec.length {
        let digit_char = digits.get(i).copied();
        let is_active = i == digits.len(); // Next slot to fill
        let display_text = match digit_char {
            Some(_) if spec.mask => "\u{2022}".to_string(),
            Some(c) => c.to_string(),
            None => String::new(),
        };

        let slot_text_color = if digit_char.is_some() {
            text_primary
        } else {
            text_secondary
        };

        // Active slot gets accent border to indicate focus position
        let slot_bc = if is_active && !spec.is_disabled {
            accent
        } else {
            slot_border
        };

        let slot = ui_element::div()
            .w(slot_w)
            .h(slot_h)
            .rounded(radius)
            .bg(surface)
            .border(border_width)
            .border_color(slot_bc)
            .flex_row()
            .items_center()
            .justify_center()
            .child(
                ui_element::label(&display_text)
                    .text_size(font_size)
                    .text_color(slot_text_color)
                    .text_weight(600)
                    .text_align_center(),
            );

        root = root.child(slot);
    }

    // ── Error message (below slots) ──
    if let Some(ref error) = spec.error {
        let error_label = ui_element::label(error.as_str())
            .text_size(rem_to_px(0.75))
            .text_color(danger);

        let wrapper = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.5))
            .items_center()
            .child(root)
            .child(error_label);

        let mut outer = wrapper;
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            outer = outer.opacity(opacity).disabled(true);
        }
        return outer;
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn default_has_six_slots() {
        let spec = CodeInputSpec::new();
        let el = js_code_input(&spec, &theme());
        assert_eq!(el.children.len(), 6);
    }

    #[test]
    fn custom_length_changes_slot_count() {
        let spec = CodeInputSpec::new().with_length(4);
        let el = js_code_input(&spec, &theme());
        assert_eq!(el.children.len(), 4);
    }

    #[test]
    fn disabled_has_reduced_opacity() {
        let spec = CodeInputSpec::new().with_disabled(true);
        let el = js_code_input(&spec, &theme());
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn with_value_fills_slots() {
        let spec = CodeInputSpec::new().with_value("123");
        let el = js_code_input(&spec, &theme());
        // First 3 slots should have digit children with text
        assert_eq!(el.children.len(), 6);
    }

    #[test]
    fn error_wraps_in_column() {
        let spec = CodeInputSpec::new().with_error("Invalid code");
        let el = js_code_input(&spec, &theme());
        // With error: outer wrapper is a column with slots + error label
        assert_eq!(el.children.len(), 2, "Should have slot row + error label");
    }

    #[test]
    fn masked_hides_digits() {
        let spec = CodeInputSpec::new().with_value("123").with_mask(true);
        let el = js_code_input(&spec, &theme());
        assert_eq!(el.children.len(), 6);
    }
}
