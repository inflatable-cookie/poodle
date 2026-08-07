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
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_render::segmented_control;
use poodle_specs::{ChoiceOption, EyebrowSpec, SegmentedControlSpec};

/// Build a node-tier SegmentedControl whose change handler records the picked
/// value under `segmented-value` (drained into specimen state next render).
fn node_segmented_control(spec: SegmentedControlSpec, state: &AppState) -> AnyElement {
    let events = state.node_events.clone();
    let on_change: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "segmented-value".to_string(),
            value: value.to_string(),
        });
    });
    let node = segmented_control(&spec, &state.theme, Some(on_change));
    poodle_gpui_node_backend::to_gpui(&node)
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
    let view_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("grid", "Grid"),
        ChoiceOption::new("list", "List"),
        ChoiceOption::new("table", "Table"),
    ];

    let selected_value = state
        .specimens
        .text
        .get("segmented-value")
        .map(|s| s.as_str())
        .unwrap_or("grid")
        .to_string();

    let default_spec =
        SegmentedControlSpec::new(view_options.clone()).with_default_value(&selected_value);

    // --- With disabled option: All / Active / Archived / Draft (disabled), defaultValue="all" ---
    let status_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("all", "All"),
        ChoiceOption::new("active", "Active"),
        ChoiceOption::new("archived", "Archived"),
        ChoiceOption::new("draft", "Draft").with_disabled(true),
    ];

    let disabled_opt_spec = SegmentedControlSpec::new(status_options).with_default_value("all");

    // --- Fully disabled: Grid / List / Table, defaultValue="list", isDisabled ---
    let mut fully_disabled_spec =
        SegmentedControlSpec::new(view_options).with_default_value("list");
    fully_disabled_spec.is_disabled = true;

    // --- Equal width: carries an aria label ("Time range") ---
    let mut equal_width_spec = SegmentedControlSpec::new(vec![
        ChoiceOption::new("day", "Day"),
        ChoiceOption::new("week", "Week"),
        ChoiceOption::new("month", "Month"),
        ChoiceOption::new("year", "Year"),
    ])
    .with_default_value("week")
    .with_equal_width(true);
    equal_width_spec.aria_label = Some("Time range".to_string());

    // --- Content fit: per-option aria labels + group label "Timeline window" ---
    let mut content_fit_spec = SegmentedControlSpec::new(vec![
        ChoiceOption::new("1h", "1h").with_aria_label("Last 1 hour"),
        ChoiceOption::new("6h", "6h").with_aria_label("Last 6 hours"),
        ChoiceOption::new("24h", "24h").with_aria_label("Last 24 hours"),
    ])
    .with_default_value("24h")
    .with_size(poodle_specs::ControlSize::Xs)
    .with_equal_width(false);
    content_fit_spec.aria_label = Some("Timeline window".to_string());

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
        .into_any_element();

    let make_opts = || {
        vec![
            ChoiceOption::new("grid", "Grid"),
            ChoiceOption::new("list", "List"),
            ChoiceOption::new("table", "Table"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "segmented-control",
        examples,
        move |size, theme: &GpuiThemeProvider| {
            node_segmented_control_static(
                SegmentedControlSpec::new(make_opts())
                    .with_default_value("grid")
                    .with_size(size),
                theme,
            )
        },
        move |density, theme: &GpuiThemeProvider| {
            node_segmented_control_static(
                SegmentedControlSpec::new(make_opts())
                    .with_default_value("grid")
                    .with_density(density),
                theme,
            )
        },
    )
}
