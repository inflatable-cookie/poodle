use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, TriStateSwitch};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CheckState, ControlSize, EyebrowSpec, TriStateSwitchSpec, TriStateValue};
use std::sync::Arc;

fn state_from_key(value: usize) -> CheckState {
    match value {
        0 => CheckState::Unchecked,
        2 => CheckState::Checked,
        _ => CheckState::Mixed,
    }
}

fn state_label(state: CheckState) -> &'static str {
    match state {
        CheckState::Unchecked => "excluded",
        CheckState::Mixed => "default",
        CheckState::Checked => "included",
    }
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

    let filter_state = state_from_key(state.specimens.selected("tri-state-filter"));
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
                        TriStateSwitchSpec::new().with_state(filter_state),
                        theme,
                    )
                    .on_change(filter_change(state)),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", state_label(filter_state))),
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
                        .with_included_label("Show"),
                    theme,
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
                                .with_state(CheckState::Mixed)
                                .with_size(ControlSize::Xs),
                            theme,
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_state(CheckState::Mixed)
                                .with_size(ControlSize::Sm),
                            theme,
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_state(CheckState::Mixed)
                                .with_size(ControlSize::Md),
                            theme,
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_state(CheckState::Mixed)
                                .with_size(ControlSize::Lg),
                            theme,
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_state(CheckState::Mixed)
                                .with_size(ControlSize::Xl),
                            theme,
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
                                .with_state(CheckState::Unchecked)
                                .with_size(ControlSize::Xs),
                            theme,
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_state(CheckState::Checked)
                                .with_size(ControlSize::Sm),
                            theme,
                        ))
                        .child(TriStateSwitch::from_spec(
                            TriStateSwitchSpec::new()
                                .with_state(CheckState::Mixed)
                                .with_size(ControlSize::Md),
                            theme,
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
                        .with_state(CheckState::Checked)
                        .with_disabled(true),
                    theme,
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
                            .with_state(filter_state)
                            .with_excluded_color("#ef4444")
                            .with_default_color("#64748b")
                            .with_included_color("#22c55e"),
                        theme,
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
                        .with_state(CheckState::Mixed)
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                TriStateSwitch::from_spec(
                    TriStateSwitchSpec::new()
                        .with_state(CheckState::Mixed)
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
