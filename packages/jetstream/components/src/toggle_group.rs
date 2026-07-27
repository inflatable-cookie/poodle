//! JsToggleGroup — grouped toggle selection backed by ToggleGroupSpec.
//!
//! Contract: `docs/contracts/components/toggle-group.md`
//! Reference: `packages/svelte/components/src/ToggleGroup.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ToggleGroupSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    toggle_group_gap_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Build a Jetstream toggle group element from a ToggleGroupSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .toggle-group]  <div role="radiogroup"|role="group">
///   └── [Item .toggle-group__item...]  <button role="radio"|role="button">
/// ```
pub fn js_toggle_group(spec: &ToggleGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let border_subtle: Color = resolve_color(theme, "color.border.subtle").into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    let radius = resolve_radius(theme, "radius.control");

    // Contract §8 Root: gap is density-driven (compact 0.1875 / default 0.25 /
    // comfortable 0.375 rem). Matches Svelte + GPUI exactly.
    let gap = rem_to_px(toggle_group_gap_rem(spec.density));

    // Item sizing
    // Contract: min-height = calc(control-height - 0.25rem) (0.25rem is a
    // contract-exact rem reduction applied uniformly across sizes).
    let item_height = rem_to_px(control_height_rem(effective_size)) - rem_to_px(0.25);
    // Contract §8 Item: padding `0 var(--poodle-toggle-group-x)` (density-driven).
    let item_pad_x = rem_to_px(control_space_x_rem(spec.density));

    // Contract: item border 0.0625rem, border-color = color-mix(border-subtle 82%, transparent)
    let item_border_color = Color::new(
        border_subtle.r,
        border_subtle.g,
        border_subtle.b,
        border_subtle.a * 0.82,
    );

    // Svelte/contract: item background = color-mix(surface 93%, text-primary)
    // (the previous surface/elevated 72% was the stale pre-Svelte contract value).
    let item_fill = surface.mix_srgb(text_primary, 0.93);

    // Svelte/contract: selected = accent tinted at 22% *over the item fill*
    // (not accent over transparent — the previous stale value).
    let selected_fill = accent.mix_srgb(item_fill, 0.22);

    // Contract: selected border = color-mix(accent-base 42%, border-default)
    let selected_border = accent.mix_srgb(border_default, 0.42);

    // Contract §8 Item: font-size = var(--poodle-typography-label-size) (flat
    // across sizes — data-size only changes height, not font-size), font-weight 600.
    let font_size = resolve_px(theme, "typography.label.size");
    // Contract §8 Item: border 0.0625rem (contract-exact rem).
    let border_width = rem_to_px(0.0625);

    // ── Root container ──
    let mut root = ui_element::div()
        .flex_row()
        .flex_wrap()
        .gap(gap)
        .items_center();

    // ── Build items ──
    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_item_disabled = spec.is_disabled || option.is_disabled;

        let (bg, bc) = if is_selected {
            (selected_fill, selected_border)
        } else {
            (item_fill, item_border_color)
        };

        let mut item = ui_element::button(&option.label)
            // Hit-test id so a host can route option activation
            // (matches the tree/tabs `prefix:<value>` convention).
            .id(format!("toggle:{}", option.value))
            .min_h(item_height)
            .pl(item_pad_x)
            .pr(item_pad_x)
            .rounded(radius)
            .bg(bg)
            .border(border_width)
            .border_color(bc)
            .text_color(text_primary)
            .text_size(font_size)
            .text_weight(600)
            .flex_row()
            .items_center()
            .justify_center()
            .focusable();

        if !is_item_disabled {
            let hover_fill = bg.mix_srgb(elevated, 0.84);
            item = item
                .hover(|s| s.bg(hover_fill))
                .cursor_pointer();
        } else {
            let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            item = item.opacity(opacity).disabled(true);
        }

        root = root.child(item);
    }

    // ── Group-level disabled ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity);
    }

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
        // Contract: the group is a `radiogroup` when selection is single.
        .aria_role(jetstream_ui::accesskit::Role::RadioGroup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ControlDensity, ControlSize, ToggleGroupOption, ToggleGroupSelectionMode};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample_options() -> Vec<ToggleGroupOption> {
        vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List"),
            ToggleGroupOption::new("board", "Board"),
        ]
    }

    #[test]
    fn renders_correct_number_of_items() {
        let spec = ToggleGroupSpec::new(sample_options());
        let el = js_toggle_group(&spec, &theme());
        assert_eq!(el.children.len(), 3);
    }

    #[test]
    fn selected_item_has_different_background() {
        let spec = ToggleGroupSpec::new(sample_options())
            .with_value(vec![String::from("grid")]);
        let el = js_toggle_group(&spec, &theme());
        let first_bg = el.children[0].style.background;
        let second_bg = el.children[1].style.background;
        assert_ne!(first_bg, second_bg, "Selected and unselected items should differ");
    }

    #[test]
    fn disabled_group_has_reduced_opacity() {
        let spec = ToggleGroupSpec::new(sample_options()).with_disabled(true);
        let el = js_toggle_group(&spec, &theme());
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn item_fill_uses_svelte_surface_text_mix() {
        use jetstream_ui::Color;
        let th = theme();
        let surface: Color = resolve_color(&th, "color.background.surface").into();
        let text_primary: Color = resolve_color(&th, "color.text.primary").into();
        // Svelte: color-mix(surface 93%, text-primary). Was the stale surface/elevated 72%.
        let expected = surface.mix_srgb(text_primary, 0.93);
        let el = js_toggle_group(&ToggleGroupSpec::new(sample_options()), &th);
        let bg = el.children[0].style.background.expect("item bg");
        assert!(
            (bg.r - expected.r).abs() < 0.01
                && (bg.g - expected.g).abs() < 0.01
                && (bg.b - expected.b).abs() < 0.01,
            "item fill {:?} should match surface-93%/text-primary {:?}",
            bg,
            expected
        );
    }

    // ── Probe: one button per option, labels laid out ──
    #[test]
    fn renders_a_button_per_option_with_labels() {
        let th = theme();
        let tree = probe(&js_toggle_group(&ToggleGroupSpec::new(sample_options()), &th), 400.0, 80.0);
        assert!(tree.has_text("Grid") && tree.has_text("List") && tree.has_text("Board"));
        assert_eq!(tree.count_kind("Button"), 3, "expected one button per option");
        let root = &tree.nodes[0];
        assert!(root.w > 0.0 && root.h > 0.0, "root laid out");
    }

    // ── Probe: selected item paints the accent-tinted fill ──
    #[test]
    fn selected_item_paints_accent_tint() {
        use jetstream_ui::Color;
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let surface: Color = resolve_color(&th, "color.background.surface").into();
        let text_primary: Color = resolve_color(&th, "color.text.primary").into();
        // Selected fill = accent 22% over the surface-93%/text-primary item fill.
        let item_fill = surface.mix_srgb(text_primary, 0.93);
        let selected = accent.mix_srgb(item_fill, 0.22);
        let tree = probe(
            &js_toggle_group(
                &ToggleGroupSpec::new(sample_options()).with_value(vec!["grid".into()]),
                &th,
            ),
            400.0,
            80.0,
        );
        assert!(
            tree.has_background(ProbeColor { r: selected.r, g: selected.g, b: selected.b, a: selected.a }, 0.02),
            "selected item should paint the accent-tinted fill"
        );
    }

    // ── Multi-select: two selected items both differ from the unselected one ──
    #[test]
    fn multi_select_marks_multiple_items_selected() {
        let th = theme();
        let opts = vec![
            ToggleGroupOption::new("design", "Design"),
            ToggleGroupOption::new("eng", "Engineering"),
            ToggleGroupOption::new("docs", "Docs"),
        ];
        let spec = ToggleGroupSpec::new(opts)
            .with_selection_mode(ToggleGroupSelectionMode::Multiple)
            .with_value(vec!["design".into(), "docs".into()]);
        let el = js_toggle_group(&spec, &th);
        let design_bg = el.children[0].style.background.expect("design bg");
        let eng_bg = el.children[1].style.background.expect("eng bg");
        let docs_bg = el.children[2].style.background.expect("docs bg");
        assert_eq!(design_bg, docs_bg, "both selected items share the selected fill");
        assert_ne!(design_bg, eng_bg, "selected and unselected fills differ");
    }

    // ── Density drives gap, not item height ──
    #[test]
    fn density_changes_gap_not_item_height() {
        let th = theme();
        let item_h = |d: ControlDensity| {
            let spec = ToggleGroupSpec::new(sample_options()).with_density(d);
            let tree = probe(&js_toggle_group(&spec, &th), 400.0, 80.0);
            tree.nodes.iter().find(|n| n.kind == "Button").map(|n| n.h).unwrap_or(0.0)
        };
        assert_eq!(
            item_h(ControlDensity::Compact),
            item_h(ControlDensity::Comfortable),
            "density must not change item height"
        );
    }

    // ── Font-size resolves from the label-size token (flat across sizes) ──
    #[test]
    fn font_size_is_label_size_token_for_all_sizes() {
        let th = theme();
        let expected = resolve_px(&th, "typography.label.size");
        for size in [ControlSize::Xs, ControlSize::Md, ControlSize::Xl] {
            let spec = ToggleGroupSpec::new(sample_options()).with_size(size);
            let tree = probe(&js_toggle_group(&spec, &th), 400.0, 120.0);
            let btn = tree
                .nodes
                .iter()
                .find(|n| n.kind == "Button" && n.text_size.is_some())
                .expect("a sized button");
            assert_eq!(btn.text_size, Some(expected), "size {size:?} font drifted from label-size");
        }
    }
}
