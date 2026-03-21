use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{AccordionSpec, AccordionItemSpec, AccordionSelectionValue};
use pug_gpui_components::Accordion;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Single selection items (from contract) ---
    let single_items = vec![
        AccordionItemSpec::new("getting-started", "Getting started")
            .with_description("Follow these steps to set up your project and start building."),
        AccordionItemSpec::new("api-reference", "API reference")
            .with_description("Complete documentation for all available endpoints and methods."),
        AccordionItemSpec::new("accessibility", "Accessibility")
            .with_description("Guidelines for building accessible components with proper ARIA support."),
    ];

    // Track single-selection expanded state
    let single_expanded = {
        let mut vals = Vec::new();
        for item in &single_items {
            let key = format!("accordion-single-{}", item.value);
            if state.specimens.is_on(&key) {
                vals.push(item.value.clone());
            }
        }
        vals
    };

    let mut single_spec = AccordionSpec::new(single_items)
        .with_allow_multiple(false)
        .with_collapsible(true);

    if !single_expanded.is_empty() {
        single_spec = single_spec.with_value(AccordionSelectionValue::Single(
            single_expanded.into_iter().next().unwrap(),
        ));
    }

    // --- Multiple selection items (from contract) ---
    let multi_items = vec![
        AccordionItemSpec::new("design-tokens", "Design tokens")
            .with_description("Tokens define the visual language of your application."),
        AccordionItemSpec::new("keyboard-shortcuts", "Keyboard shortcuts")
            .with_description("Common shortcuts for navigating and interacting with components."),
        AccordionItemSpec::new("known-issues", "Known issues")
            .with_description("Current limitations and workarounds for known bugs."),
    ];

    // Track multiple-selection expanded state
    let multi_expanded = {
        let mut vals = Vec::new();
        for item in &multi_items {
            let key = format!("accordion-multi-{}", item.value);
            if state.specimens.is_on(&key) {
                vals.push(item.value.clone());
            }
        }
        vals
    };

    let mut multi_spec = AccordionSpec::new(multi_items)
        .with_allow_multiple(true)
        .with_collapsible(true);

    if !multi_expanded.is_empty() {
        multi_spec = multi_spec.with_value(AccordionSelectionValue::Multiple(multi_expanded));
    }

    div().flex().flex_col().gap(px(16.0))
        // --- Single selection ---
        .child(section_label("SINGLE SELECTION", text_secondary))
        .child(
            Accordion::from_spec(single_spec, theme)
                .with_id("specimen-accordion-single")
                .on_toggle(cx.listener(|this, value: &str, _w, cx| {
                    let key = format!("accordion-single-{}", value);
                    // Single mode: clear all others, toggle this one
                    let is_on = this.state.specimens.is_on(&key);
                    // Clear all single keys
                    for item_val in &["getting-started", "api-reference", "accessibility"] {
                        let k = format!("accordion-single-{}", item_val);
                        if this.state.specimens.is_on(&k) {
                            this.state.specimens.toggle(&k);
                        }
                    }
                    // If it wasn't on, turn it on
                    if !is_on {
                        this.state.specimens.toggle(&key);
                    }
                    cx.notify();
                })),
        )
        // --- Multiple selection ---
        .child(section_label("MULTIPLE SELECTION", text_secondary))
        .child(
            Accordion::from_spec(multi_spec, theme)
                .with_id("specimen-accordion-multi")
                .on_toggle(cx.listener(|this, value: &str, _w, cx| {
                    let key = format!("accordion-multi-{}", value);
                    this.state.specimens.toggle(&key);
                    cx.notify();
                })),
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
