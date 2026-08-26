//! ToggleGroup specimen — migrated to the node tier (g12.019 Batch B).
//!
//! Every ToggleGroup below renders through `poodle_render::toggle_group`
//! (`Spec + Context → Node`) interpreted by `poodle_gpui_node_backend::to_gpui`.
//! The old hand-written `poodle_gpui_components::ToggleGroup` no longer
//! renders this specimen; everything around the groups (layout, Eyebrow
//! headings, captions) is unchanged.
//!
//! The change handler receives the owned resulting `ToggleGroupValue`. The
//! specimen stores that result in the text map; it does not rebuild
//! membership from an activated option.

use crate::node_compat::Eyebrow;
use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::toggle_group::ToggleGroupValue;

use poodle_render::{toggle_group, RenderContext, ToggleGroupHandlers};
use poodle_specs::{EyebrowSpec, ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

fn specimen_text(value: ToggleGroupValue) -> String {
    match value {
        ToggleGroupValue::Single(Some(value)) => value,
        ToggleGroupValue::Single(None) => String::new(),
        ToggleGroupValue::Multiple(values) => values.join(","),
    }
}

fn node_toggle_group(spec: ToggleGroupSpec, key: &'static str, state: &AppState) -> AnyElement {
    let events = state.node_events.clone();
    let on_value_change: Arc<dyn Fn(ToggleGroupValue) + Send + Sync> = Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: specimen_text(value),
        });
    });
    let node = toggle_group(
        &spec,
        &RenderContext::new(&state.theme),
        ToggleGroupHandlers::new(key).on_value_change(on_value_change),
    );
    poodle_gpui_node_backend::to_gpui(&node)
}

fn node_toggle_group_static(
    spec: ToggleGroupSpec,
    instance_id: impl Into<String>,
    state: &AppState,
) -> AnyElement {
    let node = toggle_group(
        &spec,
        &RenderContext::new(&state.theme),
        ToggleGroupHandlers::new(instance_id),
    );
    poodle_gpui_node_backend::to_gpui(&node)
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
                .child(node_toggle_group(
                    ToggleGroupSpec::new(multi_options)
                        .with_aria_label("Filter tags")
                        .with_value(
                            multi_value
                                .split(',')
                                .filter(|value| !value.is_empty())
                                .map(str::to_string)
                                .collect(),
                        )
                        .with_selection_mode(ToggleGroupSelectionMode::Multiple),
                    "toggle-group-multiple",
                    state,
                ))
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
                    "toggle-group-disabled",
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
                    "toggle-group-disabled-item",
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
                poodle_gpui_node_backend::to_gpui(&toggle_group(
                    &spec,
                    &RenderContext::new(theme),
                    ToggleGroupHandlers::new(format!("specimen-size-{size:?}")),
                ))
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                let spec = ToggleGroupSpec::new(make_opts())
                    .with_default_value(vec!["grid".to_string()])
                    .with_density(density);
                poodle_gpui_node_backend::to_gpui(&toggle_group(
                    &spec,
                    &RenderContext::new(theme),
                    ToggleGroupHandlers::new(format!("specimen-density-{density:?}")),
                ))
            }),
    )
}
