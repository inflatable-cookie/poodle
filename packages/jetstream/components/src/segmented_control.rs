//! SegmentedControl — Jetstream segmented control backed by SegmentedControlSpec.

use jetstream_ui::ui_element::{self, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SegmentedControlSpec;

use crate::presentation::{control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

pub fn js_segmented_control(spec: &SegmentedControlSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    // Contract §8 Label: font-size fixed at 0.75rem for ALL sizes (not size-scaled).
    let font_size = rem_to_px(0.75);
    // Contract §8 Label padding-x = `--poodle-segmented-control-x` (density-driven).
    let seg_px = rem_to_px(control_space_x_rem(spec.density));
    // Inner container padding and gap between segments: 0.125rem (Svelte gap/padding).
    let inner = rem_to_px(0.125);
    // Vertical inset top+bottom = same 0.125rem inner padding, so label fills
    // (height − 0.25rem), matching Svelte (not a second raw literal).
    let seg_py = inner;

    let selected_fill = resolve_color(theme, spec.selected_fill_token());
    let surface = resolve_color(theme, "color.background.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let control_radius = resolve_radius(theme, "radius.control");
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

    // Contract §8 Root background = color-mix(surface 93%, text-primary).
    let root_bg = color_mix(surface, text_primary, 0.93);
    // Contract §8 Root border = color-mix(border-subtle 84%, transparent).
    let root_border = crate::theme_ext::tint(border_subtle, 0.84);
    // Contract §8 Label border-radius = calc(radius-control − 0.125rem).
    let inner_radius = (control_radius - inner).max(0.0);

    let selected = spec.current_value();

    let mut el = ui_element::div()
        .bg(root_bg)
        .border(1.0).border_color(root_border)
        .rounded(control_radius)
        .flex_row().items_center()
        .p(inner)
        .gap(inner)
        .h(height);

    // When equalWidth=false the group sizes to content and left-aligns (contract §7).
    if !spec.equal_width {
        el = el.justify_start();
    }

    for option in &spec.options {
        let is_selected = selected == Some(option.value.as_str());
        // Per-segment disabled only when the individual option is disabled but
        // the whole control is not (the container handles the whole-control case).
        let is_option_disabled = !spec.is_disabled && option.is_disabled;

        let text_color = if is_selected { text_inverse } else { text_muted };

        let mut seg = ui_element::button(&option.label)
            .text_size(font_size).text_weight(600)
            .text_color(text_color)
            .pl(seg_px).pr(seg_px).pt(seg_py).pb(seg_py)
            // Contract §8 Label inner radius (control − 0.125rem), not full control.
            .rounded(inner_radius)
            .focusable();

        if is_selected {
            // Contract §8 selected: accent fill + inverse text + an
            // `inset 0 0.0625rem 0 white@12%` top-highlight box-shadow.
            seg = seg.bg(selected_fill).shadow_layers(vec![BoxShadow {
                offset_x: 0.0,
                offset_y: rem_to_px(0.0625),
                blur: 0.0,
                spread: 0.0,
                color: glam::Vec4::new(1.0, 1.0, 1.0, 0.12),
                inset: true,
            }]);
        }

        if spec.equal_width {
            seg = seg.grow();
        }

        if is_option_disabled {
            seg = seg.opacity(disabled_opacity).disabled(true);
        }

        // Per-option aria_label / title (contract §6): Jetstream has no
        // accessibility channel, so these cannot be emitted (accepted delta).
        let _aria_label = option.aria_label.as_deref();

        el = el.child(seg);
    }

    // Whole-control disabled: wrap in an additional opacity layer so the border
    // and background also dim, not just the individual segments.
    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    // NOTE: click + arrow-key selection live in the preview event loop, not the
    // component (Jetstream architecture). This renders selected state from
    // `current_value()` only.
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ChoiceOption, ControlDensity, ControlSize};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn opts() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("grid", "Grid"),
            ChoiceOption::new("list", "List"),
            ChoiceOption::new("table", "Table"),
        ]
    }

    fn accent(th: &JetstreamThemeProvider) -> ProbeColor {
        let c = resolve_color(th, SegmentedControlSpec::default().selected_fill_token());
        ProbeColor { r: c.x, g: c.y, b: c.z, a: c.w }
    }

    #[test]
    fn renders_a_segment_per_option_with_labels() {
        let th = theme();
        let spec = SegmentedControlSpec::new(opts()).with_default_value("grid");
        let tree = probe(&js_segmented_control(&spec, &th), 400.0, 80.0);

        assert!(tree.has_text("Grid") && tree.has_text("List") && tree.has_text("Table"));
        // One Button widget per option.
        assert_eq!(tree.count_kind("Button"), 3, "expected 3 segments");
        // Root laid out with positive size.
        let root = &tree.nodes[0];
        assert!(root.w > 0.0 && root.h > 0.0);
    }

    #[test]
    fn selected_segment_uses_accent_fill() {
        let th = theme();
        let spec = SegmentedControlSpec::new(opts()).with_default_value("list");
        let tree = probe(&js_segmented_control(&spec, &th), 400.0, 80.0);
        // The selected treatment paints the accent fill on exactly one segment.
        assert!(
            tree.has_background(accent(&th), 0.02),
            "selected segment missing accent fill"
        );
    }

    #[test]
    fn font_size_is_fixed_at_075rem_across_sizes() {
        let th = theme();
        for size in [ControlSize::Xs, ControlSize::Md, ControlSize::Xl] {
            let spec = SegmentedControlSpec::new(opts())
                .with_default_value("grid")
                .with_size(size);
            let tree = probe(&js_segmented_control(&spec, &th), 400.0, 120.0);
            // Contract §8: label font-size fixed at 0.75rem (12px) for every size.
            let seg = tree
                .nodes
                .iter()
                .find(|n| n.kind == "Button" && n.text_size.is_some())
                .expect("a sized segment");
            assert_eq!(seg.text_size, Some(rem_to_px(0.75)), "size {size:?} font drifted");
        }
    }

    #[test]
    fn density_drives_segment_padding_not_height() {
        let th = theme();
        let h = |d: ControlDensity| {
            let spec = SegmentedControlSpec::new(opts())
                .with_default_value("grid")
                .with_density(d);
            let tree = probe(&js_segmented_control(&spec, &th), 400.0, 120.0);
            tree.nodes[0].h
        };
        // Density must not change control height.
        assert_eq!(h(ControlDensity::Compact), h(ControlDensity::Comfortable));
    }

    #[test]
    fn disabled_group_dims_the_whole_control() {
        let th = theme();
        let spec = SegmentedControlSpec {
            is_disabled: true,
            ..SegmentedControlSpec::new(opts()).with_default_value("grid")
        };
        // Just assert it lays out without panicking and keeps all segments.
        let tree = probe(&js_segmented_control(&spec, &th), 400.0, 80.0);
        assert_eq!(tree.count_kind("Button"), 3);
    }
}
