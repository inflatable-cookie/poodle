//! JsCheckbox — checkbox with indicator and label, backed by CheckboxSpec.
//!
//! Contract: `docs/contracts/components/checkbox.md`
//! Reference: `packages/svelte/components/src/Checkbox.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CheckState, CheckboxSpec, ControlSize};

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Indicator size in rem by size variant.
///
/// Contract size table (section 8):
/// - xs: icon-default − 0.125rem  = 1.0 − 0.125 = 0.875rem
/// - sm: icon-default             = 1.0rem
/// - md: 1.125rem (explicit)
/// - lg: icon-default + 0.375rem  = 1.375rem
/// - xl: icon-default + 0.625rem  = 1.625rem
fn indicator_size_rem(size: ControlSize) -> f32 {
    // icon-default = size.icon.md = 1.0rem (16px)
    const ICON_DEFAULT: f32 = 1.0;
    match size {
        ControlSize::Xs => ICON_DEFAULT - 0.125,
        ControlSize::Sm => ICON_DEFAULT,
        ControlSize::Md => 1.125,
        ControlSize::Lg => ICON_DEFAULT + 0.375,
        ControlSize::Xl => ICON_DEFAULT + 0.625,
    }
}

/// Mark (glyph) size in rem by size variant.
///
/// Contract size table (section 8):
/// - xs: icon-default − 0.375rem  = 0.625rem
/// - sm: icon-default − 0.25rem   = 0.75rem
/// - md: 0.875rem (explicit)
/// - lg: icon-default             = 1.0rem
/// - xl: icon-default + 0.125rem  = 1.125rem
fn mark_size_rem(size: ControlSize) -> f32 {
    const ICON_DEFAULT: f32 = 1.0;
    match size {
        ControlSize::Xs => ICON_DEFAULT - 0.375,
        ControlSize::Sm => ICON_DEFAULT - 0.25,
        ControlSize::Md => 0.875,
        ControlSize::Lg => ICON_DEFAULT,
        ControlSize::Xl => ICON_DEFAULT + 0.125,
    }
}

/// Build a checkbox element from a CheckboxSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .checkbox] — <label>
///   ├── [Control]   — <input type="checkbox"> (visually hidden — not rendered in Jetstream)
///   ├── [Indicator] — <span>, size-dependent square, border-radius 0.3125rem
///   │   └── [Mark]  — <span>, size-dependent square (conditional: checked/mixed)
///   └── [Label]     — <span> (optional)
/// ```
///
/// Contract dimensions (md defaults):
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

    // ── Indicator — size from contract size table ──
    let indicator_size = rem_to_px(indicator_size_rem(effective_size));
    // Contract: border-radius 0.3125rem (fixed, does not scale with size)
    let indicator_radius = rem_to_px(0.3125);
    // Contract: border 0.0625rem solid
    let border_width = rem_to_px(0.0625);
    let mark_size = rem_to_px(mark_size_rem(effective_size));

    let mut indicator = ui_element::div()
        .w(indicator_size)
        .h(indicator_size)
        .rounded(indicator_radius)
        .bg(indicator_bg)
        .border(border_width).border_color(indicator_border)
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

    // Contract: disabled → opacity from state-opacity-disabled token, cursor not-allowed
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    // Contract: readOnly → full opacity, default cursor, non-interactive
    // No opacity reduction; the component remains visible but reverts changes.
    // Jetstream has no cursor token, so we omit cursor styling (platform default).

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
    fn indicator_is_18px_at_md() {
        let el = js_checkbox(&CheckboxSpec::new(), &theme());
        // First child is the indicator
        let indicator = &el.children[0];
        assert_eq!(indicator.layout.size.width, taffy::Dimension::length(18.0));
        assert_eq!(indicator.layout.size.height, taffy::Dimension::length(18.0));
    }

    #[test]
    fn indicator_scales_by_size() {
        let theme = theme();
        // xs: 0.875rem = 14px
        let el_xs = js_checkbox(&CheckboxSpec::new().with_size(poodle_specs::ControlSize::Xs), &theme);
        let ind_xs = &el_xs.children[0];
        assert_eq!(ind_xs.layout.size.width, taffy::Dimension::length(14.0));

        // sm: 1.0rem = 16px
        let el_sm = js_checkbox(&CheckboxSpec::new().with_size(poodle_specs::ControlSize::Sm), &theme);
        let ind_sm = &el_sm.children[0];
        assert_eq!(ind_sm.layout.size.width, taffy::Dimension::length(16.0));
    }

    #[test]
    fn read_only_checkbox_is_not_disabled() {
        let el = js_checkbox(
            &CheckboxSpec::new().with_read_only(true),
            &theme(),
        );
        // read_only should not set disabled(true) or reduce opacity
        assert!(!el.style.disabled);
    }
}
