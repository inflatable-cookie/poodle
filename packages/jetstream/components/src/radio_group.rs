//! RadioGroup — Jetstream radio group backed by RadioGroupSpec.
//!
//! Contract: `docs/contracts/components/radio-group.md`
//! Reference: `packages/svelte/components/src/RadioGroup.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, Orientation, RadioGroupSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{hex_to_rgb255, resolve_color, resolve_opacity, resolve_px, rgb255_to_vec4};

/// Indicator (outer circle) size in px by size variant.
///
/// `icon_md_px` is the resolved `size.icon.md` token (the contract's
/// `icon-default`, GPUI: `resolve_px(theme, "size.icon.md")`). The rem offsets
/// below are contract §8 literals via the sanctioned `rem_to_px` helper.
///
/// Contract size table (section 8):
/// - xs: icon-default − 0.125rem
/// - sm: icon-default
/// - md: 1.125rem (explicit)
/// - lg: icon-default + 0.375rem
/// - xl: icon-default + 0.625rem
fn indicator_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px - rem_to_px(0.125),
        ControlSize::Sm => icon_md_px,
        ControlSize::Md => rem_to_px(1.125),
        ControlSize::Lg => icon_md_px + rem_to_px(0.375),
        ControlSize::Xl => icon_md_px + rem_to_px(0.625),
    }
}

/// Dot (inner filled circle) size in px by size variant.
///
/// `icon_md_px` is the resolved `size.icon.md` token. xs/sm/lg/xl scale it by
/// the contract's per-size ratios; md uses the explicit `0.5rem` literal.
///
/// Contract size table (section 8):
/// - xs: icon-default × 0.40
/// - sm: icon-default × 0.45
/// - md: 0.5rem (explicit)
/// - lg: icon-default × 0.55
/// - xl: icon-default × 0.60
fn dot_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px * 0.40,
        ControlSize::Sm => icon_md_px * 0.45,
        ControlSize::Md => rem_to_px(0.5),
        ControlSize::Lg => icon_md_px * 0.55,
        ControlSize::Xl => icon_md_px * 0.60,
    }
}

pub fn js_radio_group(spec: &RadioGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    // icon-default = size.icon.md token (GPUI: resolve_px(theme, "size.icon.md")).
    let icon_md = resolve_px(theme, "size.icon.md");
    let indicator_size = indicator_size_px(effective_size, icon_md);
    let dot_size = dot_size_px(effective_size, icon_md);
    // Contract: border 0.0625rem (1px) solid
    let border_width = rem_to_px(0.0625);

    // Contract: group gap driven by spec.option_gap_token() (space-stack-sm vertical, space-inline-md horizontal)
    let group_gap = resolve_px(theme, spec.option_gap_token());
    // Contract: option item gap = space-inline-sm (between indicator and label)
    let item_gap = resolve_px(theme, "space.inline.sm");

    // Selected indicator/dot color: `spec.selected_color` (a custom hex string)
    // wins when present, else `color.accent.base`. Mirrors GPUI
    // (`parse_hex_color(hex).unwrap_or(accent)`) and Svelte's `selectedColor`.
    // Value-derived (non-token) colors carry sRGB channels straight into the
    // pipeline, matching the color-picker convention (accepted minor space delta).
    let accent = spec
        .selected_color
        .as_deref()
        .and_then(hex_to_rgb255)
        .map(|rgb| rgb255_to_vec4(rgb, rgb.a))
        .unwrap_or_else(|| resolve_color(theme, "color.accent.base"));
    let border = resolve_color(theme, "color.border.default");
    let text_color = resolve_color(theme, "color.text.primary");
    let selected_value = spec.value.as_deref().or(spec.default_value.as_deref());

    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

    let mut el = match spec.orientation {
        Orientation::Horizontal => ui_element::div().flex_row().gap(group_gap),
        Orientation::Vertical => ui_element::div().flex_col().gap(group_gap),
    };

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = resolve_color(theme, "color.background.surface");

        // Radio indicator: circle with inner dot when selected
        let mut indicator = ui_element::div()
            .w(indicator_size).h(indicator_size)
            .rounded(indicator_size * 0.5)
            .bg(indicator_bg)
            .border(border_width).border_color(indicator_color)
            .items_center().justify_center();

        if is_selected {
            indicator = indicator.child(
                ui_element::div()
                    .w(dot_size).h(dot_size)
                    .rounded(dot_size * 0.5)
                    .bg(accent)
            );
        }

        let mut row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(item_gap)
            .cursor_pointer();
        row = row.child(indicator);
        row = row.child(
            ui_element::label(&option.label)
                .text_color(text_color)
                .text_size(font_size)
        );

        // Contract: per-option disabled applies opacity to that option row only
        if option.is_disabled {
            row = row.opacity(disabled_opacity);
        }

        el = el.child(row);
    }

    // Contract: group-level disabled → opacity on the whole group
    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::ChoiceOption;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn vec4_to_probe(c: glam::Vec4) -> ProbeColor {
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    fn spec_with_color(color: Option<&str>) -> RadioGroupSpec {
        let mut spec = RadioGroupSpec::new(vec![
            ChoiceOption::new("a", "Free"),
            ChoiceOption::new("b", "Pro"),
        ])
        .with_value("a");
        spec.selected_color = color.map(|c| c.to_string());
        spec
    }

    #[test]
    fn custom_selected_color_drives_indicator_and_dot() {
        let th = theme();
        let el = js_radio_group(&spec_with_color(Some("#ff0000")), &th);
        let tree = probe(&el, 400.0, 120.0);

        // The custom hex parses to sRGB red, mirroring the color-picker's
        // value-derived color convention (sRGB channels straight into .bg()).
        let custom = rgb255_to_vec4(hex_to_rgb255("#ff0000").unwrap(), 1.0);
        let custom_probe = vec4_to_probe(custom);
        let accent_base = vec4_to_probe(resolve_color(&th, "color.accent.base"));

        // The selected dot renders with the custom color...
        assert!(
            tree.has_background(custom_probe, 0.01),
            "selected dot should use custom selected_color (#ff0000): {}",
            tree.to_json()
        );
        // ...and the custom color must be distinct from accent-base, proving
        // the fix actually reads spec.selected_color rather than the accent.
        assert!(
            !custom_probe.approx(accent_base, 0.01),
            "test precondition: custom color must differ from accent-base"
        );

        // Border-color is not surfaced through ProbeNode, so assert the
        // selected indicator's border on the built JsEl tree directly: the
        // first option ("a") is selected and its indicator border must be the
        // custom color, not accent-base.
        let selected_row = &el.children[0];
        let indicator = &selected_row.children[0];
        let bc = indicator.style.border_color.expect("indicator border");
        assert!(
            (bc.r - custom.x).abs() < 0.01
                && (bc.g - custom.y).abs() < 0.01
                && (bc.b - custom.z).abs() < 0.01,
            "selected indicator border should use custom selected_color"
        );
    }

    #[test]
    fn no_selected_color_falls_back_to_accent_base() {
        let th = theme();
        let el = js_radio_group(&spec_with_color(None), &th);
        let tree = probe(&el, 400.0, 120.0);

        let accent_base = vec4_to_probe(resolve_color(&th, "color.accent.base"));
        // With no custom color, the selected dot uses accent-base.
        assert!(
            tree.has_background(accent_base, 0.01),
            "selected dot should fall back to accent-base when selected_color is None: {}",
            tree.to_json()
        );
    }
}
