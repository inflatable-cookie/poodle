//! Segmented Control specimen — migrated to the node tier (g12.019 Batch B).
//!
//! Every SegmentedControl below renders through the node tier:
//! `poodle_render::segmented_control` (`Spec + Theme → Node`) interpreted by
//! `poodle_gpui_node_backend::to_gpui`. The old hand-written
//! `poodle_gpui_components::SegmentedControl` no longer renders this specimen;
//! everything around the controls (layout, Eyebrow headings, captions) is
//! unchanged.
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

use poodle_render::segmented_control;
use poodle_specs::{ControlSize, EyebrowSpec, SegmentedControlOption, SegmentedControlSpec};

/// Build a node-tier SegmentedControl whose change handler records the picked
/// value under `key` (drained into specimen state next render).
fn node_segmented_control_keyed(
    spec: SegmentedControlSpec,
    state: &AppState,
    key: &str,
) -> AnyElement {
    let events = state.node_events.clone();
    let key = key.to_string();
    let on_change: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.clone(),
            value: value.to_string(),
        });
    });
    let node = segmented_control(&spec, &state.theme, Some(on_change));
    poodle_gpui_node_backend::to_gpui(&node)
}

/// Default live control: records under `segmented-value`.
fn node_segmented_control(spec: SegmentedControlSpec, state: &AppState) -> AnyElement {
    node_segmented_control_keyed(spec, state, "segmented-value")
}

/// A node-tier SegmentedControl with no change handler (disabled / static /
/// sizes / densities).
fn node_segmented_control_static(
    spec: SegmentedControlSpec,
    theme: &GpuiThemeProvider,
) -> AnyElement {
    let node = segmented_control(&spec, theme, None);
    poodle_gpui_node_backend::to_gpui(&node)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // --- Default: Grid / List / Table, value="grid" ---
    let view_options: Vec<SegmentedControlOption> = vec![
        SegmentedControlOption::new("grid", "Grid"),
        SegmentedControlOption::new("list", "List"),
        SegmentedControlOption::new("table", "Table"),
    ];

    let selected_value = state
        .specimens
        .text
        .get("segmented-value")
        .map(|s| s.as_str())
        .unwrap_or("grid")
        .to_string();

    let default_spec = SegmentedControlSpec::new(view_options.clone())
        .with_default_value(&selected_value)
        .with_instance_id("default");

    // --- With disabled option: All / Active / Archived / Draft (disabled), defaultValue="all" ---
    let status_options: Vec<SegmentedControlOption> = vec![
        SegmentedControlOption::new("all", "All"),
        SegmentedControlOption::new("active", "Active"),
        SegmentedControlOption::new("archived", "Archived"),
        SegmentedControlOption::new("draft", "Draft").with_disabled(true),
    ];

    let disabled_opt_spec = SegmentedControlSpec::new(status_options)
        .with_default_value("all")
        .with_instance_id("disabled-option");

    // --- Fully disabled: Grid / List / Table, defaultValue="list", isDisabled ---
    let mut fully_disabled_spec = SegmentedControlSpec::new(view_options)
        .with_default_value("list")
        .with_instance_id("fully-disabled");
    fully_disabled_spec.is_disabled = true;

    // --- Equal width: carries an aria label ("Time range") ---
    let mut equal_width_spec = SegmentedControlSpec::new(vec![
        SegmentedControlOption::new("day", "Day"),
        SegmentedControlOption::new("week", "Week"),
        SegmentedControlOption::new("month", "Month"),
        SegmentedControlOption::new("year", "Year"),
    ])
    .with_default_value("week")
    .with_equal_width(true)
    .with_instance_id("equal-width");
    equal_width_spec.aria_label = Some("Time range".to_string());

    // --- Content fit: per-option aria labels + group label "Timeline window" ---
    let mut content_fit_spec = SegmentedControlSpec::new(vec![
        SegmentedControlOption::new("1h", "1h").with_aria_label("Last 1 hour"),
        SegmentedControlOption::new("6h", "6h").with_aria_label("Last 6 hours"),
        SegmentedControlOption::new("24h", "24h").with_aria_label("Last 24 hours"),
    ])
    .with_default_value("24h")
    .with_size(poodle_specs::ControlSize::Xs)
    .with_equal_width(false)
    .with_instance_id("content-fit");
    content_fit_spec.aria_label = Some("Timeline window".to_string());

    // --- Icon-only options: Effects / Instruments, live selection ---
    let icon_selected = state
        .specimens
        .text
        .get("segmented-icon-value")
        .map(|s| s.as_str())
        .unwrap_or("effects")
        .to_string();
    let mut icon_only_spec = SegmentedControlSpec::new(vec![
        SegmentedControlOption::new("effects", "Effects")
            .with_icon("audio-waveform")
            .with_icon_only(true),
        SegmentedControlOption::new("instruments", "Instruments")
            .with_icon("piano")
            .with_icon_only(true),
    ])
    .with_default_value(&icon_selected)
    .with_size(ControlSize::Sm)
    .with_equal_width(false)
    .with_instance_id("icon-only");
    icon_only_spec.aria_label = Some("Plugin kind".to_string());

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(node_segmented_control(default_spec, state))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Selected: {}", selected_value)),
                ),
        )
        // --- With disabled option ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With disabled option"),
                    theme,
                ))
                .child(node_segmented_control_static(disabled_opt_spec, theme)),
        )
        // --- Fully disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Fully disabled"),
                    theme,
                ))
                .child(node_segmented_control_static(fully_disabled_spec, theme)),
        )
        // --- Equal width segments (equalWidth=true, default) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Equal width segments"),
                    theme,
                ))
                .child(
                    div()
                        .w(px(360.0))
                        .child(node_segmented_control_static(equal_width_spec, theme)),
                ),
        )
        // --- Content fit (equalWidth=false): segments size to label, group left-aligns ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Content fit (equalWidth=false)"),
                    theme,
                ))
                .child(node_segmented_control_static(content_fit_spec, theme)),
        )
        // --- Icon-only options (contract §13) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Icon-only options"),
                    theme,
                ))
                .child(node_segmented_control_keyed(
                    icon_only_spec,
                    state,
                    "segmented-icon-value",
                )),
        )
        .into_any_element();

    let make_opts = || {
        vec![
            SegmentedControlOption::new("grid", "Grid"),
            SegmentedControlOption::new("list", "List"),
            SegmentedControlOption::new("table", "Table"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "segmented-control",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                node_segmented_control_static(
                    SegmentedControlSpec::new(make_opts())
                        .with_default_value("grid")
                        .with_size(size)
                        .with_instance_id(format!("size-{size:?}")),
                    theme,
                )
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                node_segmented_control_static(
                    SegmentedControlSpec::new(make_opts())
                        .with_default_value("grid")
                        .with_density(density)
                        .with_instance_id(format!("density-{density:?}")),
                    theme,
                )
            }),
    )
}

#[cfg(test)]
mod icon_only_tests {
    // Explicit imports only: `use super::*` would chain the parent's
    // `use gpui::*` and glob in gpui's `test` proc macro.
    use super::segmented_control;
    use crate::app_state::NodeSpecimenEvent;
    use poodle_gpui::GpuiThemeProvider;
    use poodle_specs::{ControlSize, SegmentedControlOption, SegmentedControlSpec};
    use std::sync::{Arc, Mutex};

    fn icon_only_node(
        selected: &str,
        events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
        theme: &GpuiThemeProvider,
    ) -> poodle_node::Node {
        let events = Arc::clone(events);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(move |value: &str| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: "segmented-icon-value".to_string(),
                value: value.to_string(),
            });
        });
        let mut spec = SegmentedControlSpec::new(vec![
            SegmentedControlOption::new("effects", "Effects")
                .with_icon("audio-waveform")
                .with_icon_only(true),
            SegmentedControlOption::new("instruments", "Instruments")
                .with_icon("piano")
                .with_icon_only(true),
        ])
        .with_default_value(selected)
        .with_size(ControlSize::Sm)
        .with_equal_width(false)
        .with_instance_id("icon-only");
        spec.aria_label = Some("Plugin kind".to_string());
        segmented_control(&spec, theme, Some(on_change))
    }

    fn find_icon<'a>(node: &'a poodle_node::Node, icon: &str) -> &'a poodle_node::Node {
        node.children
            .iter()
            .find(|seg| {
                seg.find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == icon),
                )
                .is_some()
            })
            .expect("icon segment")
    }

    #[test]
    fn icon_only_activation_records_the_picked_value() {
        let events: Arc<Mutex<Vec<NodeSpecimenEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let theme = GpuiThemeProvider::new();
        let node = icon_only_node("effects", &events, &theme);
        let instruments = find_icon(&node, "piano");
        let on_activate = instruments.interaction.on_activate.as_ref().expect(
            "icon-only option has an on_activate handler — without one the section is inert",
        );
        on_activate();
        let queue = events.lock().unwrap();
        match &queue[0] {
            NodeSpecimenEvent::SetText { key, value } => {
                assert_eq!(key, "segmented-icon-value");
                assert_eq!(value, "instruments");
            }
            _ => panic!("event 0 is a SetText"),
        }
    }
}
