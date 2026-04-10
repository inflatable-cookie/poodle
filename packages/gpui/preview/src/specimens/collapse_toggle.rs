use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{CollapseDirection, CollapseToggleSpec, ControlDensity, ControlSize, EyebrowSpec};
use poodle_gpui_components::{CollapseToggle, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let label_color = color_to_hsla(text_secondary);

    let left_collapsed = state.specimens.is_on("ct-left-collapsed");
    let right_collapsed = state.specimens.is_on("ct-right-collapsed");
    let up_collapsed = state.specimens.is_on("ct-up-collapsed");
    let down_collapsed = state.specimens.is_on("ct-down-collapsed");

    // Helper: labeled toggle with direction name + state
    let labeled = |toggle: CollapseToggle, name: &str, collapsed: bool| -> Div {
        let state_str = if collapsed { "(collapsed)" } else { "(expanded)" };
        div().flex().items_center().gap(px(6.0))
            .child(toggle)
            .child(
                div().text_size(px(12.0)).text_color(label_color)
                    .child(format!("{} {}", name, state_str))
            )
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Directions (interactive, all directions) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Directions"), theme))
                .child(
                    div().flex().gap(px(24.0)).items_center().flex_wrap()
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Left)
                                    .with_collapsed(left_collapsed),
                                theme,
                            )
                            .with_id("interactive-left")
                            .on_toggle(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("ct-left-collapsed");
                                cx.notify();
                            })),
                            "Left", left_collapsed,
                        ))
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Right)
                                    .with_collapsed(right_collapsed),
                                theme,
                            )
                            .with_id("interactive-right")
                            .on_toggle(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("ct-right-collapsed");
                                cx.notify();
                            })),
                            "Right", right_collapsed,
                        ))
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Up)
                                    .with_collapsed(up_collapsed),
                                theme,
                            )
                            .with_id("interactive-up")
                            .on_toggle(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("ct-up-collapsed");
                                cx.notify();
                            })),
                            "Up", up_collapsed,
                        ))
                        .child(labeled(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Down)
                                    .with_collapsed(down_collapsed),
                                theme,
                            )
                            .with_id("interactive-down")
                            .on_toggle(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("ct-down-collapsed");
                                cx.notify();
                            })),
                            "Down", down_collapsed,
                        ))
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().gap(px(8.0)).items_center()
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("size-xs").size(ControlSize::Xs))
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("size-sm").size(ControlSize::Sm))
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("size-md").size(ControlSize::Md))
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("size-lg").size(ControlSize::Lg))
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("size-xl").size(ControlSize::Xl))
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().gap(px(8.0)).items_center()
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("density-compact").with_density(ControlDensity::Compact))
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("density-default").with_density(ControlDensity::Default))
                        .child(CollapseToggle::from_spec(CollapseToggleSpec::new().with_direction(CollapseDirection::Left), theme).with_id("density-comfortable").with_density(ControlDensity::Comfortable))
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    div().flex().gap(px(24.0)).items_center()
                        .child(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Left)
                                    .with_disabled(true),
                                theme,
                            )
                            .with_id("disabled-left")
                        )
                        .child(
                            CollapseToggle::from_spec(
                                CollapseToggleSpec::new()
                                    .with_direction(CollapseDirection::Right)
                                    .with_disabled(true),
                                theme,
                            )
                            .with_id("disabled-right")
                        )
                )
        )
}
