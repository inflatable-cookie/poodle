use gpui::*;
use poodle_primitives::{CollapseDirection, CollapseToggleSpec, EyebrowSpec};
use poodle_gpui_components::{CollapseToggle, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let left_collapsed = state.specimens.is_on("ct-left-collapsed");
    let right_collapsed = state.specimens.is_on("ct-right-collapsed");
    let up_collapsed = state.specimens.is_on("ct-up-collapsed");
    let down_collapsed = state.specimens.is_on("ct-down-collapsed");

    div().flex().flex_col().gap(px(24.0))
        // --- Directions (interactive, all directions) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Directions"), theme))
                .child(
                    div().flex().gap(px(8.0)).items_center()
                        .child(
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
                            }))
                        )
                        .child(
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
                            }))
                        )
                        .child(
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
                            }))
                        )
                        .child(
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
                            }))
                        )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    div().flex().gap(px(8.0)).items_center()
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
