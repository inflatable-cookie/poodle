//! ToggleGroup specimen — migrated to the node tier (g12.019 Batch B).
//!
//! Every ToggleGroup below renders through `poodle_render::toggle_group`
//! (`Spec + Context → Node`) interpreted by `poodle_gpui_node_backend::to_gpui`.
//! The old hand-written `poodle_gpui_components::ToggleGroup` no longer
//! renders this specimen; everything around the groups (layout, Eyebrow
//! headings, captions) is unchanged.
//!
//! Node interaction closures are context-free (`Arc<dyn Fn(&str) + Send +
//! Sync>`), so instead of `cx.listener` the change handler pushes a
//! `NodeSpecimenEvent::SetText` onto a queue the next render drains into
//! specimen state (see `app_state.rs`).

use crate::node_compat::Eyebrow;
use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_render::RenderContext;
use poodle_render::toggle_group;
use poodle_specs::{EyebrowSpec, ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

/// Build a node-tier ToggleGroup whose change handler records the activated
/// option's value under `key`. Mirrors the old specimen's behavior:
/// `on_change` fires with the option value and the specimen stores it in the
/// text map (`--print-state` reads it back).
fn node_toggle_group(spec: ToggleGroupSpec, key: &'static str, state: &AppState) -> AnyElement {
    let events = state.node_events.clone();
    let on_change: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    });
    let node = toggle_group(&spec, &RenderContext::new(&state.theme), Some(on_change));
    poodle_gpui_node_backend::to_gpui(&node)
}

/// A node-tier ToggleGroup with no handlers (disabled / sizes / densities).
fn node_toggle_group_static(spec: ToggleGroupSpec, state: &AppState) -> AnyElement {
    let node = toggle_group(&spec, &RenderContext::new(&state.theme), None);
    poodle_gpui_node_backend::to_gpui(&node)
}

/// Multi-select transition: the node tier's `on_change` reports the activated
/// option, not the resulting set, so the specimen owns membership. Toggling
/// an active value removes it; anything else is appended.
fn multi_select_transition(current: &[String], activated: &str) -> Vec<String> {
    let mut next = current.to_vec();
    if let Some(index) = next.iter().position(|v| v == activated) {
        next.remove(index);
    } else {
        next.push(activated.to_string());
    }
    next
}

/// The live multiple-selection group as a render node, handler wired: the
/// node tier only attaches `on_activate` when a handler is present, so
/// without this the section is inert. Returned as a node (pre-conversion) so
/// the focused test can invoke an option's `on_activate` directly.
fn multi_select_toggle_group_node(
    options: Vec<ToggleGroupOption>,
    current: Vec<String>,
    events: &Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    theme: &GpuiThemeProvider,
) -> poodle_node::Node {
    let spec = ToggleGroupSpec::new(options)
        .with_aria_label("Filter tags")
        .with_value(current.clone())
        .with_selection_mode(ToggleGroupSelectionMode::Multiple);
    let events = Arc::clone(events);
    let on_change: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(move |value: &str| {
        let next = multi_select_transition(&current, value);
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "toggle-group-multiple".to_string(),
            value: next.join(","),
        });
    });
    toggle_group(&spec, &RenderContext::new(theme), Some(on_change))
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let single_value = state
        .specimens
        .text
        .get("toggle-group-single")
        .cloned()
        .unwrap_or_else(|| "grid".to_string());
    let four_value = state
        .specimens
        .text
        .get("toggle-group-four")
        .cloned()
        .unwrap_or_else(|| "left".to_string());
    let multi_value = state
        .specimens
        .text
        .get("toggle-group-multiple")
        .cloned()
        .unwrap_or_else(|| "design,docs".to_string());

    // --- Single selection: Grid / List / Board ---
    let single_options = vec![
        ToggleGroupOption::new("grid", "Grid"),
        ToggleGroupOption::new("list", "List"),
        ToggleGroupOption::new("board", "Board"),
    ];

    // --- Four options: Left / Center / Right / Justify ---
    let four_options = vec![
        ToggleGroupOption::new("left", "Left"),
        ToggleGroupOption::new("center", "Center"),
        ToggleGroupOption::new("right", "Right"),
        ToggleGroupOption::new("justify", "Justify"),
    ];

    // --- Multiple selection ---
    let multi_options = vec![
        ToggleGroupOption::new("design", "Design"),
        ToggleGroupOption::new("engineering", "Engineering"),
        ToggleGroupOption::new("docs", "Docs"),
    ];

    // --- Disabled ---
    let disabled_options = vec![
        ToggleGroupOption::new("grid", "Grid"),
        ToggleGroupOption::new("list", "List"),
        ToggleGroupOption::new("board", "Board"),
    ];

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single selection"),
                    theme,
                ))
                .child(node_toggle_group(
                    ToggleGroupSpec::new(single_options)
                        .with_aria_label("View mode")
                        .with_value(vec![single_value.clone()]),
                    "toggle-group-single",
                    state,
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("View: {}", single_value)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Four options"),
                    theme,
                ))
                .child(node_toggle_group(
                    ToggleGroupSpec::new(four_options)
                        .with_aria_label("Text alignment")
                        .with_value(vec![four_value]),
                    "toggle-group-four",
                    state,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiple selection"),
                    theme,
                ))
                .child({
                    let current: Vec<String> = multi_value
                        .split(',')
                        .filter(|v| !v.is_empty())
                        .map(str::to_string)
                        .collect();
                    poodle_gpui_node_backend::to_gpui(&multi_select_toggle_group_node(
                        multi_options,
                        current,
                        &state.node_events,
                        &state.theme,
                    ))
                })
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!(
                            "Selected: {}",
                            if multi_value.is_empty() {
                                "none".to_string()
                            } else {
                                multi_value.replace(',', ", ")
                            }
                        )),
                ),
        )
        // --- Disabled group ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(node_toggle_group_static(
                    ToggleGroupSpec::new(disabled_options)
                        .with_aria_label("Disabled toggle group")
                        .with_default_value(vec!["list".to_string()])
                        .with_disabled(true),
                    state,
                )),
        )
        // --- Disabled item ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled item"),
                    theme,
                ))
                .child(node_toggle_group_static(
                    ToggleGroupSpec::new(vec![
                        ToggleGroupOption::new("grid", "Grid"),
                        ToggleGroupOption::new("list", "List").with_disabled(true),
                        ToggleGroupOption::new("board", "Board"),
                    ])
                    .with_aria_label("Toggle group with disabled item")
                    .with_default_value(vec!["grid".to_string()]),
                    state,
                )),
        )
        .into_any_element();

    let make_opts = || {
        vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List"),
            ToggleGroupOption::new("board", "Board"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "toggle-group",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                let spec = ToggleGroupSpec::new(make_opts())
                    .with_default_value(vec!["grid".to_string()])
                    .with_size(size);
                poodle_gpui_node_backend::to_gpui(&toggle_group(&spec, &RenderContext::new(theme), None))
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                let spec = ToggleGroupSpec::new(make_opts())
                    .with_default_value(vec!["grid".to_string()])
                    .with_density(density);
                poodle_gpui_node_backend::to_gpui(&toggle_group(&spec, &RenderContext::new(theme), None))
            }),
    )
}

#[cfg(test)]
mod multi_select_tests {
    // Explicit imports only: `use super::*` would chain the parent's
    // `use gpui::*` and glob in gpui's `test` proc macro, shadowing the
    // built-in `#[test]` (gpui-macros 0.2.2 crashes on current rustc).
    use super::{multi_select_toggle_group_node, multi_select_transition};
    use crate::app_state::NodeSpecimenEvent;
    use poodle_gpui::GpuiThemeProvider;
    use poodle_specs::ToggleGroupOption;
    use std::sync::{Arc, Mutex};

    fn tag_options() -> Vec<ToggleGroupOption> {
        vec![
            ToggleGroupOption::new("design", "Design"),
            ToggleGroupOption::new("engineering", "Engineering"),
            ToggleGroupOption::new("docs", "Docs"),
        ]
    }

    fn activate(node: &poodle_node::Node, value: &str) {
        let item = node
            .find(&|n| n.id.as_deref() == Some(format!("toggle:{value}").as_str()))
            .unwrap_or_else(|| panic!("item {value} exists"));
        let on_activate = item
            .interaction
            .on_activate
            .as_ref()
            .expect("option has an on_activate handler — without one the section is inert");
        on_activate();
    }

    fn drained_text(
        events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
        index: usize,
    ) -> (String, String) {
        let queue = events.lock().unwrap();
        match &queue[index] {
            NodeSpecimenEvent::SetText { key, value } => (key.clone(), value.clone()),
            _ => panic!("event {index} is a SetText"),
        }
    }

    #[test]
    fn activation_adds_and_removes_membership() {
        let events: Arc<Mutex<Vec<NodeSpecimenEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let theme = GpuiThemeProvider::new();
        let node = multi_select_toggle_group_node(
            tag_options(),
            vec!["design".to_string(), "docs".to_string()],
            &events,
            &theme,
        );

        // Add: activating an inactive option appends it to the stored set.
        activate(&node, "engineering");
        assert_eq!(
            drained_text(&events, 0),
            (
                "toggle-group-multiple".to_string(),
                "design,docs,engineering".to_string()
            )
        );

        // Remove: activating an active option drops it from the stored set.
        activate(&node, "design");
        assert_eq!(
            drained_text(&events, 1),
            ("toggle-group-multiple".to_string(), "docs".to_string())
        );
    }

    #[test]
    fn transition_toggles_membership() {
        let current = vec!["design".to_string(), "docs".to_string()];
        assert_eq!(
            multi_select_transition(&current, "engineering"),
            vec!["design", "docs", "engineering"]
        );
        assert_eq!(multi_select_transition(&current, "design"), vec!["docs"]);
    }
}
