//! Accordion specimen.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::accordion::js_accordion;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{AccordionItemSpec, AccordionSelectionValue, AccordionSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Single selection
        .child(group("Single selection", secondary,
            js_accordion(&AccordionSpec::new(vec![
                AccordionItemSpec {
                    value: "getting-started".into(),
                    label: "Getting started".into(),
                    description: Some("Install the package with your preferred package manager, then import individual components as needed.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "api-reference".into(),
                    label: "API reference".into(),
                    description: Some("Complete API documentation for all exported components, hooks, and utilities.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "accessibility".into(),
                    label: "Accessibility".into(),
                    description: Some("Built-in ARIA attributes, keyboard navigation, and screen reader support details.".into()),
                    is_disabled: false,
                },
            ]).with_default_value(AccordionSelectionValue::Single("getting-started".into())), theme)
        ))
        // Multiple selection
        .child(group("Multiple selection", secondary,
            js_accordion(&AccordionSpec::new(vec![
                AccordionItemSpec {
                    value: "tokens".into(),
                    label: "Design tokens".into(),
                    description: Some("Components consume semantic tokens like --poodle-color-text-primary rather than hard-coded values.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "theming".into(),
                    label: "Theming".into(),
                    description: Some("Override token values at any scope to create custom themes without modifying component source.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "density".into(),
                    label: "Density modes".into(),
                    description: Some("Switch between comfortable and compact density for different use cases.".into()),
                    is_disabled: false,
                },
            ]).with_allow_multiple(true)
              .with_default_value(AccordionSelectionValue::Multiple(vec!["tokens".into(), "theming".into()])), theme)
        ))
        // All collapsed
        .child(group("All collapsed", secondary,
            js_accordion(&AccordionSpec::new(vec![
                AccordionItemSpec {
                    value: "a".into(),
                    label: "FAQ Item 1".into(),
                    description: Some("Answer to FAQ item 1.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "b".into(),
                    label: "FAQ Item 2".into(),
                    description: Some("Answer to FAQ item 2.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "c".into(),
                    label: "FAQ Item 3".into(),
                    description: Some("Answer to FAQ item 3.".into()),
                    is_disabled: false,
                },
            ]).with_collapsible(true), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
