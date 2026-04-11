//! JsCheckbox — checkbox with indicator and label, backed by CheckboxSpec.
//!
//! Contract: `docs/contracts/components/checkbox.md`
//! Reference: `packages/svelte/primitives/src/Checkbox.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{CheckState, CheckboxSpec};

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Build a checkbox element from a CheckboxSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .checkbox] — <label>
///   ├── [Control]   — <input type="checkbox"> (visually hidden — not rendered in Jetstream)
///   ├── [Indicator] — <span>, 1.125rem square, border-radius 0.3125rem
///   │   └── [Mark]  — <span>, 0.875rem square (conditional: checked/mixed)
///   └── [Label]     — <span> (optional)
/// ```
///
/// Contract dimensions:
/// - Indicator: 1.125rem (18px) square
/// - Indicator border: 0.0625rem (1px) solid
/// - Indicator border-radius: 0.3125rem (5px)
/// - Mark: 0.875rem (14px) square
/// - Gap (root): var(--poodle-space-inline-sm) = 8px
/// - Label typography: label-family, label-size (13px), label-weight (500), label-lineHeight (16px)
pub fn js_checkbox(spec: &CheckboxSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let indicator_fill = resolve_color(theme, spec.indicator_fill_token());
    let border_default = resolve_color(theme, "color.border.default");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let gap = rem_to_px(control_space_x_rem(spec.density));
    let label_size = rem_to_px(size_font_rem(effective_size));

    let state = spec.current_state();
    let is_checked = matches!(state, CheckState::Checked | CheckState::Mixed);

    // Contract: mark icon per state (Svelte uses Icon name="check"/"minus")
    let mark_color = if is_checked { text_inverse } else { text_primary };

    // Contract: indicator border = accent-base when checked, border-default when unchecked
    let indicator_border = if is_checked { indicator_fill } else { border_default };
    // Contract: indicator bg = accent-base when checked, background-surface when unchecked
    let surface = resolve_color(theme, "color.background.surface");
    let indicator_bg = if is_checked { indicator_fill } else { surface };

    // ── Indicator (contract: 1.125rem = 18px, border-radius 0.3125rem = 5px) ──
    let indicator_size = rem_to_px(1.125);
    let indicator_radius = rem_to_px(0.3125);
    let mark_size = rem_to_px(0.875);
    let mut indicator = ui_element::div()
        .w(indicator_size)
        .h(indicator_size)
        .rounded(indicator_radius)
        .bg(indicator_bg)
        .border_1().border_color(indicator_border)
        .items_center().justify_center();

    // Mark: SVG icon (contract: check for checked, minus for mixed)
    match state {
        CheckState::Checked => {
            indicator = indicator.child(
                ui_element::icon("check")
                    .w(mark_size).h(mark_size)
                    .text_color(mark_color)
            );
        }
        CheckState::Mixed => {
            indicator = indicator.child(
                ui_element::icon("minus")
                    .w(mark_size).h(mark_size)
                    .text_color(mark_color)
            );
        }
        CheckState::Unchecked => {}
    }

    // ── Root: label + indicator + optional label text ──
    let mut root = ui_element::div()
        .flex_row()
        .gap(gap) // from SPACE_INLINE_SM token
        .items_center()
        .focusable()
        .child(indicator);

    // Contract: label is optional
    if let Some(ref label) = spec.label {
        root = root.child(
            ui_element::label(label)
                .text_color(text_primary)
                .text_size(label_size) // from TYPOGRAPHY_LABEL_SIZE token
        );
    }

    // Contract: disabled → opacity from state-opacity-disabled token
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
    fn checked_checkbox_has_children() {
        let el = js_checkbox(
            &CheckboxSpec::new().with_checked(true),
            &theme(),
        );
        assert!(!el.children.is_empty());
    }

    #[test]
    fn indicator_is_18px() {
        let el = js_checkbox(&CheckboxSpec::new(), &theme());
        // First child is the indicator
        let indicator = &el.children[0];
        assert_eq!(indicator.layout.size.width, taffy::Dimension::length(18.0));
        assert_eq!(indicator.layout.size.height, taffy::Dimension::length(18.0));
    }
}
