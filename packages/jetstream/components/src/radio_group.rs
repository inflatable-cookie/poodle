//! RadioGroup — Jetstream radio group backed by RadioGroupSpec.
//!
//! Contract: `docs/contracts/components/radio-group.md`
//! Reference: `packages/svelte/components/src/RadioGroup.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, Orientation, RadioGroupSpec};

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

    // Group gap. Svelte cascade: orientation sets the base gap (vertical
    // space-stack-sm, horizontal space-inline-md); a density override of
    // compact/comfortable then wins over both (`RadioGroup.svelte` 107/186-192):
    //   compact     → space-stack-sm
    //   comfortable → space-stack-lg
    //   default     → orientation gap (no [data-density="default"] rule)
    let group_gap = match spec.density {
        ControlDensity::Compact => resolve_px(theme, "space.stack.sm"),
        ControlDensity::Comfortable => resolve_px(theme, "space.stack.lg"),
        ControlDensity::Default => resolve_px(theme, spec.option_gap_token()),
    };
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

        let option_disabled = spec.is_disabled || option.is_disabled;

        let mut row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(item_gap);
        // Contract: enabled options are pointer + focusable (focus ring is driven
        // by the preview loop); disabled options revert to the default cursor —
        // JsEl has no `not-allowed` cursor (Svelte uses it; noted runtime limit).
        if option_disabled {
            row = row.cursor_default();
        } else {
            row = row.cursor_pointer().focusable();
        }
        row = row.child(indicator);
        row = row.child(
            ui_element::label(&option.label)
                .text_color(text_color)
                .text_size(font_size)
        );

        // Contract: per-option disabled applies opacity to that option row only.
        // Group-level disabled also dims each row (so the opacity reads correctly
        // even though the group-level pass below dims the container too).
        if option.is_disabled {
            row = row.opacity(disabled_opacity);
        }

        el = el.child(row);
    }

    // Contract: group-level disabled → opacity on the whole group
    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::RadioGroup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::ChoiceOption;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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

    fn three_opt_spec() -> RadioGroupSpec {
        RadioGroupSpec::new(vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("ent", "Enterprise"),
        ])
        .with_value("pro")
    }

    #[test]
    fn renders_all_option_labels() {
        let el = js_radio_group(&three_opt_spec(), &theme());
        let tree = probe(&el, 400.0, 160.0);
        assert!(tree.has_text("Free") && tree.has_text("Pro") && tree.has_text("Enterprise"));
        // 3 option rows under the group root.
        assert_eq!(el.children.len(), 3);
    }

    #[test]
    fn selected_option_has_dot_unselected_does_not() {
        let el = js_radio_group(&three_opt_spec(), &theme());
        // "free" (idx 0) unselected → indicator has no dot child.
        let free_indicator = &el.children[0].children[0];
        assert!(free_indicator.children.is_empty(), "unselected indicator has no dot");
        // "pro" (idx 1) selected → indicator has a dot child.
        let pro_indicator = &el.children[1].children[0];
        assert_eq!(pro_indicator.children.len(), 1, "selected indicator renders dot");
    }

    fn gap_px(el: &JsEl) -> taffy::LengthPercentage {
        el.layout.gap.width
    }

    #[test]
    fn vertical_default_gap_is_space_stack_sm() {
        let th = theme();
        let el = js_radio_group(&three_opt_spec(), &th);
        let expected = taffy::LengthPercentage::length(resolve_px(&th, "space.stack.sm"));
        assert_eq!(gap_px(&el), expected, "vertical default gap = space-stack-sm");
    }

    #[test]
    fn horizontal_default_gap_is_space_inline_md() {
        let th = theme();
        let el = js_radio_group(
            &three_opt_spec().with_orientation(Orientation::Horizontal),
            &th,
        );
        let expected = taffy::LengthPercentage::length(resolve_px(&th, "space.inline.md"));
        assert_eq!(gap_px(&el), expected, "horizontal default gap = space-inline-md");
    }

    #[test]
    fn comfortable_density_overrides_gap_to_space_stack_lg() {
        let th = theme();
        let el = js_radio_group(
            &three_opt_spec().with_density(ControlDensity::Comfortable),
            &th,
        );
        let expected = taffy::LengthPercentage::length(resolve_px(&th, "space.stack.lg"));
        assert_eq!(gap_px(&el), expected, "comfortable density gap = space-stack-lg");
        // And it must differ from the default vertical gap, proving density took effect.
        assert_ne!(
            resolve_px(&th, "space.stack.lg"),
            resolve_px(&th, "space.stack.sm")
        );
    }

    #[test]
    fn indicator_scales_per_size() {
        let th = theme();
        // Radio indicator follows contract §8 (icon-md ± offset), matching Svelte's
        // resolved per-size values.
        // md = 1.125rem = 18px (icon-md + 0.125rem).
        let md = &js_radio_group(&three_opt_spec(), &th).children[0].children[0];
        assert_eq!(md.layout.size.width, taffy::Dimension::length(18.0));
        // xs = icon-md − 0.125rem = 0.875rem = 14px (== Svelte icon-xs + 0.25rem).
        let xs = &js_radio_group(&three_opt_spec().with_size(ControlSize::Xs), &th)
            .children[0]
            .children[0];
        assert_eq!(xs.layout.size.width, taffy::Dimension::length(14.0));
        // xl = icon-md + 0.625rem = 1.625rem = 26px (== Svelte icon-xl + 0.125rem).
        let xl = &js_radio_group(&three_opt_spec().with_size(ControlSize::Xl), &th)
            .children[0]
            .children[0];
        assert_eq!(xl.layout.size.width, taffy::Dimension::length(26.0));
    }

    #[test]
    fn per_option_disabled_dims_only_that_row() {
        let th = theme();
        let spec = RadioGroupSpec::new(vec![
            ChoiceOption::new("a", "Enabled"),
            ChoiceOption::new("b", "Disabled").with_disabled(true),
        ])
        .with_value("a");
        let el = js_radio_group(&spec, &th);
        let dim = resolve_opacity(&th, "state.opacity.disabled");
        assert_eq!(el.children[0].style.opacity, 1.0, "enabled row full opacity");
        assert!(
            (el.children[1].style.opacity - dim).abs() < 0.001,
            "disabled row dimmed to state-opacity-disabled"
        );
    }

    #[test]
    fn group_disabled_dims_container() {
        let th = theme();
        let mut spec = three_opt_spec();
        spec.is_disabled = true;
        let el = js_radio_group(&spec, &th);
        let dim = resolve_opacity(&th, "state.opacity.disabled");
        assert!((el.style.opacity - dim).abs() < 0.001, "group-disabled dims container");
    }
}
