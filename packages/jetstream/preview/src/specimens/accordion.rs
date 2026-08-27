//! Accordion specimen.

use crate::compat::js_accordion;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    AccordionItemSpec, AccordionSelectionMode, AccordionSelectionValue, AccordionSpec,
    ControlDensity, ControlSize,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
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
            ]).with_default_value(AccordionSelectionValue::Single(Some("getting-started".into()))), theme, "jetstream-accordion-single")
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
            ]).with_selection_mode(AccordionSelectionMode::Multiple)
              .with_default_value(AccordionSelectionValue::Multiple(vec!["tokens".into(), "theming".into()])), theme, "jetstream-accordion-multiple")
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
            ]).with_collapsible(true), theme, "jetstream-accordion-collapsed")
        ))
        // Disabled item — js_accordion reduces opacity on the disabled item.
        .child(group("Disabled item", secondary,
            js_accordion(&AccordionSpec::new(vec![
                AccordionItemSpec {
                    value: "open".into(),
                    label: "Available".into(),
                    description: Some("This item is interactive and expanded.".into()),
                    is_disabled: false,
                },
                AccordionItemSpec {
                    value: "locked".into(),
                    label: "Locked section".into(),
                    description: Some("Requires admin access.".into()),
                    is_disabled: true,
                },
            ]).with_default_value(AccordionSelectionValue::Single(Some("open".into()))), theme, "jetstream-accordion-disabled")
        ))
        // Sizes (xs–xl) — intrinsic dimensions resolve from the size token.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(size_variant(theme, ControlSize::Xs, "xs"))
                .child(size_variant(theme, ControlSize::Sm, "sm"))
                .child(size_variant(theme, ControlSize::Md, "md"))
                .child(size_variant(theme, ControlSize::Lg, "lg"))
                .child(size_variant(theme, ControlSize::Xl, "xl"))
        ))
        // Densities — inline spacing only; height unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(density_variant(theme, ControlDensity::Compact, "compact"))
                .child(density_variant(theme, ControlDensity::Default, "default"))
                .child(density_variant(theme, ControlDensity::Comfortable, "comfortable"))
        ))
}

fn variant_items(label: &str) -> Vec<AccordionItemSpec> {
    vec![AccordionItemSpec {
        value: "section".into(),
        label: format!("Section ({label})"),
        description: Some("Resolves all dimensions from the size/density tokens.".into()),
        is_disabled: false,
    }]
}

fn size_variant(theme: &JetstreamThemeProvider, size: ControlSize, label: &str) -> El {
    js_accordion(
        &AccordionSpec::new(variant_items(label))
            .with_size(size)
            .with_default_value(AccordionSelectionValue::Single(Some("section".into()))),
        theme,
        &format!("jetstream-accordion-size-{label}"),
    )
}

fn density_variant(theme: &JetstreamThemeProvider, density: ControlDensity, label: &str) -> El {
    js_accordion(
        &AccordionSpec::new(variant_items(label))
            .with_density(density)
            .with_default_value(AccordionSelectionValue::Single(Some("section".into()))),
        theme,
        &format!("jetstream-accordion-density-{label}"),
    )
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
