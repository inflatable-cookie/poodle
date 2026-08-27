use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Accordion, Eyebrow};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{
    AccordionItemSpec, AccordionSelectionMode, AccordionSelectionValue, AccordionSpec, EyebrowSpec,
};
use std::sync::Arc;

fn content_node(text: impl Into<String>, color: poodle_node::ColorValue) -> Node {
    let mut node = Node::text(text);
    node.style.text_size = Some(14.0);
    node.style.line_height = Some(1.5);
    node.style.descriptor.text_color = Some(color);
    node
}

fn parse_single_value(raw: &str) -> AccordionSelectionValue {
    if raw.is_empty() {
        AccordionSelectionValue::Single(None)
    } else {
        AccordionSelectionValue::Single(Some(raw.to_string()))
    }
}

fn parse_multiple_value(raw: &str) -> AccordionSelectionValue {
    if raw.is_empty() {
        AccordionSelectionValue::Multiple(vec![])
    } else {
        AccordionSelectionValue::Multiple(
            raw.split(',')
                .filter(|part| !part.is_empty())
                .map(str::to_string)
                .collect(),
        )
    }
}

fn single_value_text(value: &AccordionSelectionValue) -> String {
    match value {
        AccordionSelectionValue::Single(Some(value)) => value.clone(),
        AccordionSelectionValue::Single(None) => String::new(),
        AccordionSelectionValue::Multiple(_) => String::new(),
    }
}

fn multiple_value_text(value: &AccordionSelectionValue) -> String {
    match value {
        AccordionSelectionValue::Multiple(values) => values.join(","),
        _ => String::new(),
    }
}

/// The item set both the Examples pane and the axis representatives use.
fn axis_items() -> Vec<AccordionItemSpec> {
    vec![
        AccordionItemSpec::new("getting-started", "Getting started"),
        AccordionItemSpec::new("api-reference", "API reference"),
        AccordionItemSpec::new("accessibility", "Accessibility"),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let single_items = axis_items();
    let single_content: Vec<(&str, &str)> = vec![
        ("getting-started", "Install the package with your preferred package manager, then import individual components as needed. Each component is tree-shakeable and ships with its own styles scoped via CSS custom properties."),
        ("api-reference", "Every component accepts an ariaLabel prop for accessible naming. Most interactive components support controlled and uncontrolled modes via value/defaultValue pairs, and emit granular events like valueChange, openChange, or requestClose."),
        ("accessibility", "All components follow WAI-ARIA authoring practices. Focus is trapped inside modal overlays, arrow keys navigate composite widgets, and Escape dismisses dismissible layers. Screen reader announcements use live regions where appropriate."),
    ];

    let single_raw = state
        .specimens
        .text
        .get("accordion-single")
        .cloned()
        .unwrap_or_else(|| "getting-started".to_string());
    let single_value = parse_single_value(&single_raw);

    let single_spec = AccordionSpec::new(single_items)
        .with_selection_mode(AccordionSelectionMode::Single)
        .with_collapsible(true)
        .with_value(single_value);

    let mut single_accordion =
        Accordion::from_spec(single_spec, theme, "specimen-accordion-single").on_value_change(
            Arc::new({
                let events = state.node_events.clone();
                move |value| {
                    events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: "accordion-single".to_string(),
                        value: single_value_text(&value),
                    });
                }
            }),
        );

    for (value, text) in &single_content {
        single_accordion =
            single_accordion.with_content(*value, content_node(text.to_string(), text_secondary));
    }

    let multi_items = vec![
        AccordionItemSpec::new("design", "Design tokens"),
        AccordionItemSpec::new("keyboard", "Keyboard shortcuts"),
        AccordionItemSpec::new("known-issues", "Known issues"),
    ];

    let multi_content: Vec<(&str, &str)> = vec![
        ("design", "Components consume semantic tokens like --poodle-color-text-primary and --poodle-size-control-height rather than hard-coded values. Switching themes at runtime updates every component instantly without re-rendering."),
        ("keyboard", "Enter or Space toggles the focused panel. Tab moves focus between enabled headers and then out of the accordion entirely."),
        ("known-issues", "Animation on panel expand/collapse is not yet implemented. The component does not support nested accordions. Horizontal orientation is planned but not available in this release."),
    ];

    let multi_raw = state
        .specimens
        .text
        .get("accordion-multi")
        .cloned()
        .unwrap_or_else(|| "design,keyboard".to_string());
    let multi_value = parse_multiple_value(&multi_raw);

    let multi_spec = AccordionSpec::new(multi_items)
        .with_selection_mode(AccordionSelectionMode::Multiple)
        .with_collapsible(true)
        .with_value(multi_value);

    let mut multi_accordion = Accordion::from_spec(multi_spec, theme, "specimen-accordion-multi")
        .on_value_change(Arc::new({
            let events = state.node_events.clone();
            move |value| {
                events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                    key: "accordion-multi".to_string(),
                    value: multiple_value_text(&value),
                });
            }
        }));

    for (value, text) in &multi_content {
        multi_accordion =
            multi_accordion.with_content(*value, content_node(text.to_string(), text_secondary));
    }

    let group = |title: &str, body: AnyElement| {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(title.to_string()),
                theme,
            ))
            .child(body)
    };
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Single selection",
            single_accordion.into_any_element(),
        ))
        .child(group(
            "Multiple selection",
            multi_accordion.into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "accordion",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Accordion::from_spec(
                    AccordionSpec::new(axis_items())
                        .with_selection_mode(AccordionSelectionMode::Single)
                        .with_collapsible(true)
                        .with_value(AccordionSelectionValue::Single(Some(
                            "getting-started".into(),
                        )))
                        .with_size(size),
                    theme,
                    format!("accordion-axis-{}", size_key(size)),
                )
                .with_content(
                    "getting-started",
                    content_node(
                        "Install the package, then import components as needed.",
                        theme.resolve_color("color.text.secondary"),
                    ),
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Accordion::from_spec(
                    AccordionSpec::new(axis_items())
                        .with_selection_mode(AccordionSelectionMode::Single)
                        .with_collapsible(true)
                        .with_value(AccordionSelectionValue::Single(Some(
                            "getting-started".into(),
                        )))
                        .with_density(density),
                    theme,
                    format!("accordion-axis-{}", density_key(density)),
                )
                .with_content(
                    "getting-started",
                    content_node(
                        "Install the package, then import components as needed.",
                        theme.resolve_color("color.text.secondary"),
                    ),
                )
                .into_any_element()
            }),
    )
}
