use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Accordion, Eyebrow};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{AccordionItemSpec, AccordionSelectionValue, AccordionSpec, EyebrowSpec};
use std::sync::Arc;

fn content_node(text: impl Into<String>, color: poodle_node::ColorValue) -> Node {
    let mut node = Node::text(text);
    node.style.text_size = Some(14.0);
    node.style.line_height = Some(1.5);
    node.style.descriptor.text_color = Some(color);
    node
}

fn single_toggle(state: &AppState, current: Option<String>) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetToggle {
            key: "accordion-single-__init".to_string(),
            value: true,
        });
        for item in ["getting-started", "api-reference", "accessibility"] {
            events.push(NodeSpecimenEvent::SetToggle {
                key: format!("accordion-single-{item}"),
                value: current.as_deref() != Some(value) && item == value,
            });
        }
    })
}

fn multi_toggle(state: &AppState, current: Vec<String>) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetToggle {
            key: "accordion-multi-__init".to_string(),
            value: true,
        });
        for item in ["design", "keyboard", "known-issues"] {
            let was_open = current.iter().any(|open| open == item);
            events.push(NodeSpecimenEvent::SetToggle {
                key: format!("accordion-multi-{item}"),
                value: if item == value { !was_open } else { was_open },
            });
        }
    })
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

    // --- Single selection items ---
    let single_items = axis_items();

    let single_content: Vec<(&str, &str)> = vec![
        ("getting-started", "Install the package with your preferred package manager, then import individual components as needed. Each component is tree-shakeable and ships with its own styles scoped via CSS custom properties."),
        ("api-reference", "Every component accepts an ariaLabel prop for accessible naming. Most interactive components support controlled and uncontrolled modes via value/defaultValue pairs, and emit granular events like valueChange, openChange, or requestClose."),
        ("accessibility", "All components follow WAI-ARIA authoring practices. Focus is trapped inside modal overlays, arrow keys navigate composite widgets, and Escape dismisses dismissible layers. Screen reader announcements use live regions where appropriate."),
    ];

    // Track single-selection expanded state (default: "getting-started" open)
    let single_key_prefix = "accordion-single-";
    let single_initialized = state.specimens.is_on(&format!("{single_key_prefix}__init"));
    let single_expanded: Option<String> = if !single_initialized {
        Some("getting-started".to_string())
    } else {
        single_items
            .iter()
            .find(|item| {
                state
                    .specimens
                    .is_on(&format!("{single_key_prefix}{}", item.value))
            })
            .map(|item| item.value.clone())
    };

    let mut single_spec = AccordionSpec::new(single_items)
        .with_allow_multiple(false)
        .with_collapsible(true);

    if let Some(ref val) = single_expanded {
        single_spec = single_spec.with_value(AccordionSelectionValue::Single(val.clone()));
    }

    let mut single_accordion = Accordion::from_spec(single_spec, theme)
        .with_id("specimen-accordion-single")
        .on_toggle(single_toggle(state, single_expanded.clone()));

    for (value, text) in &single_content {
        single_accordion =
            single_accordion.with_content(*value, content_node(text.to_string(), text_secondary));
    }

    // --- Multiple selection items ---
    let multi_items = vec![
        AccordionItemSpec::new("design", "Design tokens"),
        AccordionItemSpec::new("keyboard", "Keyboard shortcuts"),
        AccordionItemSpec::new("known-issues", "Known issues"),
    ];

    let multi_content: Vec<(&str, &str)> = vec![
        ("design", "Components consume semantic tokens like --poodle-color-text-primary and --poodle-size-control-height rather than hard-coded values. Switching themes at runtime updates every component instantly without re-rendering."),
        ("keyboard", "Arrow keys move focus between accordion headers. Enter or Space toggles the focused panel. Home and End jump to the first and last header respectively. Tab moves focus out of the accordion entirely."),
        ("known-issues", "Animation on panel expand/collapse is not yet implemented. The component does not support nested accordions. Horizontal orientation is planned but not available in this release."),
    ];

    // Track multiple-selection expanded state (default: "design" + "keyboard" open)
    let multi_key_prefix = "accordion-multi-";
    let multi_initialized = state.specimens.is_on(&format!("{multi_key_prefix}__init"));
    let multi_expanded: Vec<String> = if !multi_initialized {
        vec!["design".to_string(), "keyboard".to_string()]
    } else {
        multi_items
            .iter()
            .filter(|item| {
                state
                    .specimens
                    .is_on(&format!("{multi_key_prefix}{}", item.value))
            })
            .map(|item| item.value.clone())
            .collect()
    };

    let mut multi_spec = AccordionSpec::new(multi_items)
        .with_allow_multiple(true)
        .with_collapsible(true);

    if !multi_expanded.is_empty() {
        multi_spec =
            multi_spec.with_value(AccordionSelectionValue::Multiple(multi_expanded.clone()));
    }

    let mut multi_accordion = Accordion::from_spec(multi_spec, theme)
        .with_id("specimen-accordion-multi")
        .on_toggle(multi_toggle(state, multi_expanded.clone()));

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
                        .with_allow_multiple(false)
                        .with_collapsible(true)
                        .with_value(AccordionSelectionValue::Single("getting-started".into()))
                        .with_size(size),
                    theme,
                )
                .with_id(format!("accordion-axis-{}", size_key(size)))
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
                        .with_allow_multiple(false)
                        .with_collapsible(true)
                        .with_value(AccordionSelectionValue::Single("getting-started".into()))
                        .with_density(density),
                    theme,
                )
                .with_id(format!("accordion-axis-{}", density_key(density)))
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
