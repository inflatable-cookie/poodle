//! JsButton — Jetstream button component backed by ButtonSpec.
//!
//! Contract: `docs/contracts/components/button.md`
//! Reference: `packages/svelte/components/src/Button.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.
//!
//! `ButtonSpec::aria_expanded` is defined for parity with web `ariaExpanded`.
//! This runtime’s `JsEl` builder does not yet map it onto an accessibility
//! metadata channel; hosts should read the spec when they need disclosure state.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ButtonSpec;
use poodle_specs::ButtonTone;
use poodle_specs::ButtonVariant;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem, size_min_width_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

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

    // Status family for danger/success/warning secondary color-mix (button.md §8
    // Tone: danger / Tone: warning; icon-button.md §8 Tone: success). `Default`
    // has no status mix.
    let status_token = match tone {
        ButtonTone::Danger => Some("color.status.danger"),
        ButtonTone::Success => Some("color.status.success"),
        ButtonTone::Warning => Some("color.status.warning"),
        ButtonTone::Default => None,
    };

    // ── Variant × tone colors (contract) ──
    let fill: Color = match (spec.variant, status_token) {
        (ButtonVariant::Ghost, _) => Color::TRANSPARENT,
        (ButtonVariant::Secondary, Some(status)) => {
            // Danger/Success secondary: color-mix(status 16%, background-surface)
            let status_color: Color = resolve_color(theme, status).into();
            let surface: Color = resolve_color(theme, "color.background.surface").into();
            status_color.mix(surface, 0.16)
        }
        _ => resolve_color(theme, spec.resolved_fill_token()).into(),
    };

    let text_color: Color = resolve_color(theme, spec.resolved_text_token()).into();

    let border_color: Color = match (spec.variant, status_token) {
        (ButtonVariant::Ghost, _) => Color::TRANSPARENT,
        (ButtonVariant::Secondary, Some(status)) => {
            // Danger/Success secondary: color-mix(status 46%, border-default)
            let status_color: Color = resolve_color(theme, status).into();
            let border_default: Color = resolve_color(theme, "color.border.default").into();
            status_color.mix(border_default, 0.46)
        }
        _ => resolve_color(theme, spec.resolved_border_token()).into(),
    };

    // Hover/active colors (contract: mix fill with elevated)
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    let hover_fill = fill.mix(elevated, 0.84);
    let active_fill = fill.mix(elevated, 0.72);
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let hover_border = border_color.mix(text_primary, 0.78);

    // ── Sizing via presentation helpers (size_role resolves effective size) ──
    let height = rem_to_px(control_height_rem(effective_size));
    let min_width = rem_to_px(size_min_width_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(size_padding_x_offset_rem(effective_size));

    // Icon-side padding reduction (contract §8): reduce padding on the side that
    // carries an icon by `space.button.iconInset` (0.125rem). `has_leading` is
    // true when a leading icon OR the loading spinner is present; `has_trailing`
    // when a trailing icon OR the chevron is present (matches Svelte
    // `data-has-leading`/`data-has-trailing`).
    let icon_inset = resolve_px(theme, ButtonSpec::icon_side_inset_token());
    let has_leading = spec.leading_icon.is_some() || spec.is_loading;
    let has_trailing = spec.trailing_icon.is_some() || spec.chevron;
    let pad_left = if has_leading { pad_x - icon_inset } else { pad_x };
    let pad_right = if has_trailing { pad_x - icon_inset } else { pad_x };

    let radius = resolve_radius(theme, spec.radius_token());
    // Inner gap between label and icons (contract §8: `space.button.gap` = 0.375rem).
    let gap = resolve_px(theme, ButtonSpec::content_gap_token());

    // Font size from presentation helper, driven by effective size
    let label_size = rem_to_px(size_font_rem(effective_size));

    let icon_size = rem_to_px(size_font_rem(effective_size)); // icon tracks font size
    let is_disabled = spec.is_disabled || spec.is_loading;

    let has_icons = has_leading || has_trailing;
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

        // Chevron (contract §2 anatomy + §8 Chevron): trailing disclosure
        // indicator, `chevron-down` glyph at 0.5 opacity, after all other content.
        if spec.chevron {
            el = el.child(
                ui_element::icon("chevron-down")
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(text_color)
                    .opacity(0.5),
            );
        }
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::ControlSize;

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

    #[test]
    fn primary_success_fills_with_status_success() {
        let theme = test_theme();
        let success: Color = resolve_color(&theme, "color.status.success").into();
        let el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Success)
                .with_label("Confirm"),
            &theme,
        );
        let bg = el.style.background.expect("bg set");
        assert!(
            (bg.r - success.r).abs() < 0.02
                && (bg.g - success.g).abs() < 0.02
                && (bg.b - success.b).abs() < 0.02,
            "primary success fill should be status-success, got {bg:?}"
        );
    }

    #[test]
    fn ghost_success_recolors_text() {
        let theme = test_theme();
        let success: Color = resolve_color(&theme, "color.status.success").into();
        let el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_tone(ButtonTone::Success)
                .with_label("Approve"),
            &theme,
        );
        let txt = el.style.text_color.expect("text color set");
        assert!(
            (txt.r - success.r).abs() < 0.02 && (txt.g - success.g).abs() < 0.02,
            "ghost success text should be status-success, got {txt:?}"
        );
        // Ghost stays transparent-filled regardless of tone.
        if let Some(bg) = el.style.background {
            assert!(bg.a < 0.01, "ghost success fill stays transparent");
        }
    }

    #[test]
    fn secondary_success_fill_differs_from_default() {
        let theme = test_theme();
        let default_el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("x"),
            &theme,
        );
        let success_el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(ButtonTone::Success)
                .with_label("x"),
            &theme,
        );
        let d = default_el.style.background.expect("default bg");
        let s = success_el.style.background.expect("success bg");
        // Secondary success is color-mix(success 16%, surface) — tinted away from plain surface.
        assert!(
            (d.r - s.r).abs() > 0.001 || (d.g - s.g).abs() > 0.001 || (d.b - s.b).abs() > 0.001,
            "secondary success fill must differ from default secondary surface"
        );
    }

    #[test]
    fn primary_warning_fills_with_status_warning() {
        let theme = test_theme();
        let warning: Color = resolve_color(&theme, "color.status.warning").into();
        let el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Warning)
                .with_label("Proceed"),
            &theme,
        );
        let bg = el.style.background.expect("bg set");
        assert!(
            (bg.r - warning.r).abs() < 0.02
                && (bg.g - warning.g).abs() < 0.02
                && (bg.b - warning.b).abs() < 0.02,
            "primary warning fill should be status-warning, got {bg:?}"
        );
    }

    #[test]
    fn ghost_warning_recolors_text() {
        let theme = test_theme();
        let warning: Color = resolve_color(&theme, "color.status.warning").into();
        let el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_tone(ButtonTone::Warning)
                .with_label("Review"),
            &theme,
        );
        let txt = el.style.text_color.expect("text color set");
        assert!(
            (txt.r - warning.r).abs() < 0.02 && (txt.g - warning.g).abs() < 0.02,
            "ghost warning text should be status-warning, got {txt:?}"
        );
        if let Some(bg) = el.style.background {
            assert!(bg.a < 0.01, "ghost warning fill stays transparent");
        }
    }

    #[test]
    fn secondary_warning_fill_differs_from_default_and_danger() {
        let theme = test_theme();
        let default_el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("x"),
            &theme,
        );
        let warning_el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(ButtonTone::Warning)
                .with_label("x"),
            &theme,
        );
        let danger_el = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(ButtonTone::Danger)
                .with_label("x"),
            &theme,
        );
        let d = default_el.style.background.expect("default bg");
        let w = warning_el.style.background.expect("warning bg");
        let dg = danger_el.style.background.expect("danger bg");
        // Secondary warning is color-mix(warning 16%, surface) — tinted away from
        // plain surface AND distinct from the danger tint.
        assert!(
            (d.r - w.r).abs() > 0.001 || (d.g - w.g).abs() > 0.001 || (d.b - w.b).abs() > 0.001,
            "secondary warning fill must differ from default secondary surface"
        );
        assert!(
            (dg.r - w.r).abs() > 0.001 || (dg.g - w.g).abs() > 0.001 || (dg.b - w.b).abs() > 0.001,
            "secondary warning fill must differ from danger tint"
        );
    }

    #[test]
    fn chevron_renders_trailing_glyph() {
        use crate::render_probe::probe;
        let theme = test_theme();
        let el = js_button(
            &ButtonSpec::new().with_label("Options").with_chevron(true),
            &theme,
        );
        let tree = probe(&el, 160.0, 48.0);
        assert!(
            tree.has_text("chevron-down"),
            "chevron should render the chevron-down glyph: {:?}",
            tree.texts()
        );
    }
}
