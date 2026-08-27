use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, TriStateSwitch};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, EyebrowSpec, TriStateSwitchSpec, TriStateValue};
use std::sync::Arc;

fn value_from_index(index: usize) -> TriStateValue {
    match index {
        0 => TriStateValue::Excluded,
        2 => TriStateValue::Included,
        _ => TriStateValue::Default,
    }
}

fn value_label(value: TriStateValue) -> &'static str {
    value.as_str()
}

fn filter_value(state: &AppState) -> TriStateValue {
    state
        .specimens
        .selections
        .get("tri-state-filter")
        .map(|&idx| value_from_index(idx))
        .unwrap_or(TriStateValue::Default)
}

fn filter_change(state: &AppState) -> Arc<dyn Fn(TriStateValue) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::Select {
            key: "tri-state-filter".to_string(),
            index: value.index(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let filter_value = filter_value(state);
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
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_value(filter_value)
                            .with_aria_label("Filter mode"),
                        theme,
                        "tri-state-filter",
                    )
                    .on_change(filter_change(state)),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", value_label(filter_value))),
                ),
        )
        // --- Custom labels ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom labels"),
                    theme,
                ))
                .child(TriStateSwitch::from_spec(
                    TriStateSwitchSpec::new()
                        .with_excluded_label("Hide")
                        .with_default_label("All")
                        .with_included_label("Show")
                        .with_aria_label("Visibility filter"),
                    theme,
                    "tri-state-custom-labels",
                )),
        )
        // --- Semantic sizes ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic sizes"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Default)
                                .with_size(ControlSize::Xs)
                                .with_aria_label("Extra small filter"),
                            theme,
                            "tri-state-size-xs",
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Default)
                                .with_size(ControlSize::Sm)
                                .with_aria_label("Small filter"),
                            theme,
                            "tri-state-size-sm",
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Default)
                                .with_size(ControlSize::Md)
                                .with_aria_label("Medium filter"),
                            theme,
                            "tri-state-size-md",
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Default)
                                .with_size(ControlSize::Lg)
                                .with_aria_label("Large filter"),
                            theme,
                            "tri-state-size-lg",
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Default)
                                .with_size(ControlSize::Xl)
                                .with_aria_label("Extra large filter"),
                            theme,
                            "tri-state-size-xl",
                        )),
                ),
        )
        // --- Chrome vs prominent role offset ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Chrome vs prominent role offset"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Excluded)
                                .with_size(ControlSize::Xs)
                                .with_aria_label("Chrome extra small filter"),
                            theme,
                            "tri-state-role-xs",
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Included)
                                .with_size(ControlSize::Sm)
                                .with_aria_label("Prominent small filter"),
                            theme,
                            "tri-state-role-sm",
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_value(TriStateValue::Default)
                                .with_size(ControlSize::Md)
                                .with_aria_label("Control medium filter"),
                            theme,
                            "tri-state-role-md",
                        )),
                ),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(TriStateSwitch::from_spec(
                    TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Included)
                        .with_disabled(true)
                        .with_aria_label("Disabled switch"),
                    theme,
                    "tri-state-disabled",
                )),
        )
        // --- Custom semantic colors ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom semantic colors"),
                    theme,
                ))
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_value(filter_value)
                            .with_excluded_color("#ef4444")
                            .with_default_color("#64748b")
                            .with_included_color("#22c55e")
                            .with_aria_label("Filter mode"),
                        theme,
                        "tri-state-custom-colors",
                    )
                    .on_change(filter_change(state)),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "tri-state-switch",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                TriStateSwitch::from_spec(
                    TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Default)
                        .with_size(size)
                        .with_aria_label(format!("TriStateSwitch size {size:?}")),
                    theme,
                    format!("tri-state-axis-size-{size:?}"),
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                TriStateSwitch::from_spec(
                    TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Default)
                        .with_density(density)
                        .with_aria_label(format!("TriStateSwitch density {density:?}")),
                    theme,
                    format!("tri-state-axis-density-{density:?}"),
                )
                .into_any_element()
            }),
    )
}
