//! JsToggleGroup — grouped toggle selection backed by ToggleGroupSpec.
//!
//! Contract: `docs/contracts/components/toggle-group.md`
//! Reference: `packages/svelte/primitives/src/ToggleGroup.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::ToggleGroupSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

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

    // Contract: gap resolved from density
    let gap = rem_to_px(control_space_x_rem(spec.density)) * 0.5;

    // Item sizing
    // Contract: min-height = calc(control-height - 0.25rem)
    let item_height = rem_to_px(control_height_rem(effective_size)) - rem_to_px(0.25);
    let item_pad_x = rem_to_px(control_space_x_rem(spec.density));

    // Contract: item border 0.0625rem, border-color = color-mix(border-subtle 82%, transparent)
    let item_border_color = Color::new(
        border_subtle.r,
        border_subtle.g,
        border_subtle.b,
        border_subtle.a * 0.82,
    );

    // Contract: item background = color-mix(surface 72%, elevated)
    let item_fill = surface.mix(elevated, 0.72);

    // Contract: selected background = color-mix(accent-base 22%, transparent)
    let selected_fill = Color::new(accent.r, accent.g, accent.b, accent.a * 0.22);

    // Contract: selected border = color-mix(accent-base 42%, border-default)
    let selected_border = accent.mix(border_default, 0.42);

    // Contract: font-size 0.75rem, font-weight 600
    let font_size = rem_to_px(0.75);
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
            let hover_fill = bg.mix(elevated, 0.84);
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

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_components::ToggleGroupOption;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
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
}
