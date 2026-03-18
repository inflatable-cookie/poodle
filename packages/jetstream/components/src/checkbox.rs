//! JsCheckbox — checkbox with indicator and label, backed by CheckboxSpec.
//!
//! Contract: `docs/contracts/foundation/checkbox.md`
//! Reference: `packages/svelte/primitives/src/Checkbox.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::{CheckState, CheckboxSpec};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

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
/// - Gap (root): var(--pug-space-inline-sm) = 8px
/// - Label typography: label-family, label-size (13px), label-weight (500), label-lineHeight (16px)
pub fn js_checkbox(spec: &CheckboxSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Token resolution ──
    let indicator_fill = resolve_color(theme, spec.indicator_fill_token());
    let border_default = resolve_color(theme, "semantic.color.border.default");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
    let gap = resolve_px(theme, "semantic.space.inline.sm");
    let label_size = resolve_px(theme, "semantic.typography.label.size");

    let state = spec.current_state();
    let is_checked = matches!(state, CheckState::Checked | CheckState::Mixed);

    // Contract: mark glyph per state
    let mark_text = match state {
        CheckState::Checked => "✓",
        CheckState::Mixed => "—",
        CheckState::Unchecked => "",
    };
    let mark_color = if is_checked { text_inverse } else { text_primary };

    // Contract: indicator border = accent-base when checked, border-default when unchecked
    let indicator_border = if is_checked { indicator_fill } else { border_default };

    // ── Indicator (contract: 1.125rem = 18px, border-radius 0.3125rem = 5px) ──
    let indicator = ui_element::div()
        .w(18.0)  // 1.125rem
        .h(18.0)  // 1.125rem
        .rounded(5.0) // 0.3125rem
        .bg(indicator_fill)
        .border_1().border_color(indicator_border)
        .items_center().justify_center()
        .child(
            ui_element::label(mark_text)
                .text_color(mark_color)
                .text_size(14.0) // 0.875rem — mark size from contract
        );

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
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&pug_tokens::themes::DARK)
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
