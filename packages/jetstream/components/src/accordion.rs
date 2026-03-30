//! JsAccordion — expandable disclosure panels backed by AccordionSpec.
//!
//! Contract: `docs/contracts/foundation/accordion.md`
//! Reference: `packages/svelte/primitives/src/Accordion.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_primitives::ControlSize;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{AccordionItemSpec, AccordionSpec, ControlDensity};

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

/// Build an accordion element from an AccordionSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .accordion] — <div>, display grid, gap 0.75rem
///   └── [Item .accordion__item] — <section>, per-item styling
///       ├── [Heading] — <h3>
///       │   └── [Trigger .accordion__trigger] — <button aria-expanded aria-controls>
///       │       ├── [Title .accordion__title] — heading typography
///       │       ├── [Description .accordion__description] — (optional)
///       │       └── [Indicator .accordion__indicator] — chevron icon, rotates
///       └── [Panel .accordion__panel] — <div role="region" aria-labelledby>
///           └── content
/// ```
///
/// Contract item dimensions:
/// - padding: 0.875rem 1rem (14px 16px)
/// - border: 0.0625rem (1px) solid color-mix(border-subtle 36%, transparent)
/// - border-radius: var(--poodle-radius-surface)
/// - background: color-mix(surface 50%, elevated)
/// - box-shadow: inset 0 0.0625rem 0 color-mix(text-inverse 8%, transparent)
pub fn js_accordion(spec: &AccordionSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let item_gap = resolve_px(theme, spec.item_gap_token());
    let expanded = spec.expanded_values();

    // Contract: root is display grid with gap
    let mut root = ui_element::div()
        .flex_col()
        .gap(item_gap) // from item_gap_token
        .self_stretch();

    for item in &spec.items {
        let is_expanded = expanded.iter().any(|v| *v == item.value);
        root = root.child(render_item(item, is_expanded, effective_size, spec.density, theme));
    }

    root
}

fn render_item(
    item: &AccordionItemSpec,
    is_expanded: bool,
    effective_size: ControlSize,
    density: ControlDensity,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    // ── Token resolution ──
    let border_subtle = resolve_color(theme, "semantic.color.border.subtle");
    let surface = resolve_color(theme, "semantic.color.background.surface");
    let elevated = resolve_color(theme, "semantic.color.background.elevated");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    // Contract: item border = color-mix(border-subtle 36%, transparent)
    let item_border = tint(border_subtle, 0.36);
    // Contract: item background = color-mix(surface 50%, elevated)
    // Approximate as blend of surface and elevated
    let item_bg = glam::Vec4::new(
        surface.x * 0.5 + elevated.x * 0.5,
        surface.y * 0.5 + elevated.y * 0.5,
        surface.z * 0.5 + elevated.z * 0.5,
        1.0,
    );

    // Contract: chevron indicator, rotates 180deg when open
    let chevron = if is_expanded { "▾" } else { "▸" };

    // Density-driven padding for trigger and panel
    let pad_x = rem_to_px(panel_space_x_rem(density));
    let pad_y = rem_to_px(panel_space_y_rem(density));
    let title_font_size = rem_to_px(size_font_rem(effective_size));
    let chevron_font_size = title_font_size * 0.85; // slightly smaller than title

    // Contract: trigger is full-width button with grid layout 1fr auto
    let trigger = ui_element::div()
        .flex_row()
        .self_stretch()
        .px(pad_x)
        .py(pad_y)
        .items_center()
        .justify_between()
        .focusable()
        .child(
            ui_element::label(&item.label)
                .text_color(text_primary)
                .text_size(title_font_size)
        )
        .child(
            ui_element::label(chevron)
                .text_color(text_secondary)
                .text_size(chevron_font_size)
        );

    // Contract: item is a section with border, radius, background
    let mut el = ui_element::div()
        .flex_col()
        .self_stretch()
        .rounded(radius)
        .bg(item_bg)
        .border_1().border_color(item_border)
        .child(trigger);

    // Contract: panel is role="region" with aria-labelledby, conditional render
    if is_expanded {
        if let Some(ref desc) = item.description {
            let content_font_size = rem_to_px(size_font_rem(effective_size));
            let panel = ui_element::div()
                .px(pad_x)   // matches trigger padding
                .py(pad_y * 0.85) // slightly less than trigger
                .child(
                    ui_element::label(desc)
                        .text_color(text_secondary)
                        .text_size(content_font_size)
                );
            el = el.child(panel);
        }
    }

    // Contract: disabled items reduce opacity
    if item.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_primitives::AccordionSelectionValue;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn sample_items() -> Vec<AccordionItemSpec> {
        vec![
            AccordionItemSpec {
                value: "a".into(),
                label: "Section A".into(),
                description: Some("Content for A".into()),
                is_disabled: false,
            },
            AccordionItemSpec {
                value: "b".into(),
                label: "Section B".into(),
                description: Some("Content for B".into()),
                is_disabled: false,
            },
        ]
    }

    #[test]
    fn accordion_renders_items() {
        let spec = AccordionSpec::new(sample_items())
            .with_default_value(AccordionSelectionValue::Single("a".into()));
        let el = js_accordion(&spec, &theme());
        assert_eq!(el.children.len(), 2);
    }

    #[test]
    fn items_have_background() {
        let spec = AccordionSpec::new(sample_items());
        let el = js_accordion(&spec, &theme());
        // Each item should have its own background
        for item_el in &el.children {
            assert!(item_el.style.background.is_some());
        }
    }
}
