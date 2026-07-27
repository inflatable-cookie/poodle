//! JsCheckbox — checkbox with indicator and label, backed by CheckboxSpec.
//!
//! Contract: `docs/contracts/components/checkbox.md`
//! Reference: `packages/svelte/components/src/Checkbox.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CheckState, CheckboxSpec, ControlDensity, ControlSize};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{hex_to_rgb255, resolve_color, resolve_opacity, resolve_px, rgb255_to_vec4};

/// Per-size icon token (Svelte resolves `--poodle-size-icon-{size}`).
fn icon_token(size: ControlSize) -> &'static str {
    match size {
        ControlSize::Xs => "size.icon.xs",
        ControlSize::Sm => "size.icon.sm",
        ControlSize::Md => "size.icon.md",
        ControlSize::Lg => "size.icon.lg",
        ControlSize::Xl => "size.icon.xl",
    }
}

/// Indicator size in px = per-size icon token + 0.125rem.
///
/// Svelte adds `+0.125rem` to the per-size icon token at every size
/// (`Checkbox.svelte` lines 139/191/202/214/225), e.g. md = icon-md + 0.125rem
/// = 1.125rem. `icon_px` is the resolved `size.icon.{size}` token.
fn indicator_size_px(icon_px: f32) -> f32 {
    icon_px + rem_to_px(0.125)
}

/// Mark (glyph) size in px = per-size icon token + offset.
///
/// Svelte: xs/sm/md mark = `icon-{size} − 0.125rem`; lg/xl mark =
/// `icon-{size} − 0.25rem` (`Checkbox.svelte` lines 162/197/208/219/230).
fn mark_size_px(size: ControlSize, icon_px: f32) -> f32 {
    let offset = match size {
        ControlSize::Xs | ControlSize::Sm | ControlSize::Md => -0.125,
        ControlSize::Lg | ControlSize::Xl => -0.25,
    };
    icon_px + rem_to_px(offset)
}

/// Indicator border-radius in rem per size.
///
/// Svelte ladder (`Checkbox.svelte` lines 193/204/216/227): xs `0.1875`, sm
/// `0.25`, md `0.3125`, lg `0.375`, xl `0.4375rem`. Contract-exact rem literals
/// (no semantic radius token matches them) applied via `rem_to_px`.
fn indicator_radius_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.1875,
        ControlSize::Sm => 0.25,
        ControlSize::Md => 0.3125,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.4375,
    }
}

/// Root gap in px by density.
///
/// Svelte ladder: compact `0.375rem` (literal), default `space-inline-sm`,
/// comfortable `space-inline-md` (`Checkbox.svelte` lines 113/176/179).
fn root_gap_px(density: ControlDensity, theme: &JetstreamThemeProvider) -> f32 {
    match density {
        ControlDensity::Compact => rem_to_px(0.375),
        ControlDensity::Default => resolve_px(theme, "space.inline.sm"),
        ControlDensity::Comfortable => resolve_px(theme, "space.inline.md"),
    }
}

/// Build a checkbox element from a CheckboxSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .checkbox] — <label>
///   ├── [Control]   — <input type="checkbox"> (visually hidden — not rendered in Jetstream)
///   ├── [Indicator] — <span>, size-dependent square, per-size border-radius
///   │   └── [Mark]  — <span>, size-dependent square (conditional: checked/mixed)
///   └── [Label]     — <span> (optional)
/// ```
///
/// Contract dimensions (md defaults):
/// - Indicator: 1.125rem (18px) square (icon-md + 0.125rem)
/// - Indicator border: 0.0625rem (1px) solid (border-width.default token)
/// - Indicator border-radius: 0.3125rem (5px) at md; scales 0.1875→0.4375rem per size
/// - Mark: 0.875rem (14px) square at md (icon-md − 0.125rem)
/// - Gap (root): space-inline-sm at default density (compact 0.375rem, comfortable space-inline-md)
/// - Label typography: label-family, label-size (13px), label-weight (500), label-lineHeight (16px)
pub fn js_checkbox(spec: &CheckboxSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    // Selected fill/border: custom `spec.selected_color` (a hex string) wins when
    // present, else the spec's accent token. Mirrors GPUI + Svelte `selectedColor`.
    let selected_fill = spec
        .selected_color
        .as_deref()
        .and_then(hex_to_rgb255)
        .map(|rgb| rgb255_to_vec4(rgb, rgb.a))
        .unwrap_or_else(|| resolve_color(theme, spec.indicator_fill_token()));
    let border_default = resolve_color(theme, "color.border.default");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let gap = root_gap_px(spec.density, theme);
    let label_size = rem_to_px(size_font_rem(effective_size));

    let state = spec.current_state();
    let is_checked = matches!(state, CheckState::Checked | CheckState::Mixed);

    // Contract: mark icon per state (Svelte uses Icon name="check"/"minus")
    let mark_color = if is_checked { text_inverse } else { text_primary };

    // Contract: indicator border = selected color when checked, border-default when unchecked
    let indicator_border = if is_checked { selected_fill } else { border_default };
    // Contract: indicator bg = selected color when checked, background-surface when unchecked
    let surface = resolve_color(theme, "color.background.surface");
    let indicator_bg = if is_checked { selected_fill } else { surface };

    // ── Indicator — icon token + 0.125rem; radius scales per size (Svelte) ──
    let icon_px = resolve_px(theme, icon_token(effective_size));
    let indicator_size = indicator_size_px(icon_px);
    let indicator_radius = rem_to_px(indicator_radius_rem(effective_size));
    // Contract: border 0.0625rem solid → border-width.default token (= 0.0625rem)
    let border_width = resolve_px(theme, "border.width.default");
    let mark_size = mark_size_px(effective_size, icon_px);

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

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::ControlSize;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn vec4_to_probe(c: glam::Vec4) -> ProbeColor {
        ProbeColor { r: c.x, g: c.y, b: c.z, a: c.w }
    }

    #[test]
    fn checked_checkbox_renders_check_glyph() {
        let el = js_checkbox(&CheckboxSpec::new().with_checked(true), &theme());
        let tree = probe(&el, 200.0, 60.0);
        assert!(tree.has_text("check"), "checked → check icon: {}", tree.to_json());
    }

    #[test]
    fn mixed_checkbox_renders_minus_glyph() {
        let el = js_checkbox(&CheckboxSpec::new().with_mixed(true), &theme());
        let tree = probe(&el, 200.0, 60.0);
        assert!(tree.has_text("minus"), "mixed → minus icon: {}", tree.to_json());
    }

    #[test]
    fn unchecked_checkbox_renders_no_glyph() {
        let el = js_checkbox(&CheckboxSpec::new(), &theme());
        let tree = probe(&el, 200.0, 60.0);
        assert!(!tree.has_text("check") && !tree.has_text("minus"));
    }

    #[test]
    fn label_is_rendered() {
        let el = js_checkbox(&CheckboxSpec::new().with_label("Accept terms"), &theme());
        let tree = probe(&el, 200.0, 60.0);
        assert!(tree.has_text("Accept terms"), "{}", tree.to_json());
    }

    #[test]
    fn indicator_is_18px_at_md() {
        // md indicator = icon-md (1.0rem) + 0.125rem = 1.125rem = 18px.
        let el = js_checkbox(&CheckboxSpec::new(), &theme());
        let indicator = &el.children[0];
        assert_eq!(indicator.layout.size.width, taffy::Dimension::length(18.0));
        assert_eq!(indicator.layout.size.height, taffy::Dimension::length(18.0));
    }

    #[test]
    fn indicator_includes_plus_offset_per_size() {
        let theme = theme();
        // Svelte: indicator = icon-{size} + 0.125rem.
        // xs: 0.625 + 0.125 = 0.75rem = 12px
        let ind_xs = &js_checkbox(&CheckboxSpec::new().with_size(ControlSize::Xs), &theme).children[0];
        assert_eq!(ind_xs.layout.size.width, taffy::Dimension::length(12.0));
        // sm: 0.75 + 0.125 = 0.875rem = 14px
        let ind_sm = &js_checkbox(&CheckboxSpec::new().with_size(ControlSize::Sm), &theme).children[0];
        assert_eq!(ind_sm.layout.size.width, taffy::Dimension::length(14.0));
        // xl: 1.5 + 0.125 = 1.625rem = 26px
        let ind_xl = &js_checkbox(&CheckboxSpec::new().with_size(ControlSize::Xl), &theme).children[0];
        assert_eq!(ind_xl.layout.size.width, taffy::Dimension::length(26.0));
    }

    #[test]
    fn checked_indicator_uses_accent_fill() {
        let th = theme();
        let el = js_checkbox(&CheckboxSpec::new().with_checked(true), &th);
        let tree = probe(&el, 200.0, 60.0);
        let accent = vec4_to_probe(resolve_color(&th, "color.accent.base"));
        assert!(tree.has_background(accent, 0.01), "checked fill = accent: {}", tree.to_json());
    }

    #[test]
    fn custom_selected_color_drives_indicator_fill_and_border() {
        let th = theme();
        let mut spec = CheckboxSpec::new().with_checked(true);
        spec.selected_color = Some("#ff0000".to_string());
        let el = js_checkbox(&spec, &th);
        let tree = probe(&el, 200.0, 60.0);

        let custom = rgb255_to_vec4(hex_to_rgb255("#ff0000").unwrap(), 1.0);
        let custom_probe = vec4_to_probe(custom);
        let accent = vec4_to_probe(resolve_color(&th, "color.accent.base"));
        assert!(!custom_probe.approx(accent, 0.01), "precondition: custom != accent");
        // Fill (background) uses the custom color.
        assert!(tree.has_background(custom_probe, 0.01), "custom fill: {}", tree.to_json());
        // Border tracks selected_color too (not surfaced via probe → check JsEl tree).
        let indicator = &el.children[0];
        let bc = indicator.style.border_color.expect("indicator border");
        assert!(
            (bc.r - custom.x).abs() < 0.01 && (bc.g - custom.y).abs() < 0.01 && (bc.b - custom.z).abs() < 0.01,
            "checked indicator border should use selected_color"
        );
    }

    #[test]
    fn disabled_checkbox_reduces_opacity() {
        let th = theme();
        let el = js_checkbox(&CheckboxSpec::new().with_disabled(true), &th);
        let expected = resolve_opacity(&th, "state.opacity.disabled");
        assert!(el.style.disabled);
        assert!((el.style.opacity - expected).abs() < 0.001, "opacity {} != {}", el.style.opacity, expected);
    }

    #[test]
    fn read_only_checkbox_is_not_disabled() {
        let el = js_checkbox(&CheckboxSpec::new().with_read_only(true), &theme());
        // read_only must not set disabled(true) or reduce opacity.
        assert!(!el.style.disabled);
    }
}
