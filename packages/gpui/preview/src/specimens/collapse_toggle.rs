use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{CollapseToggle, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CollapseDirection, CollapseToggleSpec, EyebrowSpec};
use std::sync::Arc;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let label_color = color_to_hsla(text_secondary);

    let left_collapsed = state.specimens.is_on("ct-left-collapsed");
    let right_collapsed = state.specimens.is_on("ct-right-collapsed");
    let up_collapsed = state.specimens.is_on("ct-up-collapsed");
    let down_collapsed = state.specimens.is_on("ct-down-collapsed");

    let toggle_handler = |key: &'static str| {
        let events = state.node_events.clone();
        Arc::new(move |value: bool| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: key.to_string(),
                value,
            });
        }) as Arc<dyn Fn(bool) + Send + Sync>
    };

    // Helper: labeled toggle with direction name + state
    let labeled = |toggle: CollapseToggle, name: &str, collapsed: bool| -> Div {
        let state_str = if collapsed {
            "(collapsed)"
        } else {
            "(expanded)"
        };
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(toggle)
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(label_color)
                    .child(format!("{} {}", name, state_str)),
            )
    };

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Directions (interactive, all directions) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Directions"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(24.0))
                        .items_center()
                        .flex_wrap()
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Left)
                                    .with_collapsed(left_collapsed),
                                theme,
                            )
                            .with_id("interactive-left")
                            .on_toggle(toggle_handler("ct-left-collapsed")),
                            "Left",
                            left_collapsed,
                        ))
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Right)
                                    .with_collapsed(right_collapsed),
                                theme,
                            )
                            .with_id("interactive-right")
                            .on_toggle(toggle_handler("ct-right-collapsed")),
                            "Right",
                            right_collapsed,
                        ))
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Up)
                                    .with_collapsed(up_collapsed),
                                theme,
                            )
                            .with_id("interactive-up")
                            .on_toggle(toggle_handler("ct-up-collapsed")),
                            "Up",
                            up_collapsed,
                        ))
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Down)
                                    .with_collapsed(down_collapsed),
                                theme,
                            )
                            .with_id("interactive-down")
                            .on_toggle(toggle_handler("ct-down-collapsed")),
                            "Down",
                            down_collapsed,
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
                .child(
                    div()
                        .flex()
                        .gap(px(24.0))
                        .items_center()
                        .child(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Left)
                                    .with_disabled(true),
                                theme,
                            )
                            .with_id("disabled-left"),
                        )
                        .child(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Right)
                                    .with_disabled(true),
                                theme,
                            )
                            .with_id("disabled-right"),
                        ),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "collapse-toggle",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                CollapseToggle::from_spec(
                    CollapseToggleSpec::new().with_direction(CollapseDirection::Left),
                    theme,
                )
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                CollapseToggle::from_spec(
                    CollapseToggleSpec::new().with_direction(CollapseDirection::Left),
                    theme,
                )
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
            }),
    )
}
