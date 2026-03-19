use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{CollapseDirection, CollapseToggleSpec};
use pug_gpui_components::PugCollapseToggle;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let left_collapsed = state.specimens.is_on("ct-left-collapsed");
    let right_collapsed = state.specimens.is_on("ct-right-collapsed");
    let up_collapsed = state.specimens.is_on("ct-up-collapsed");
    let down_collapsed = state.specimens.is_on("ct-down-collapsed");

    div().flex().flex_col().gap(px(16.0))
        // --- Expanded (all directions, default state) ---
        .child(section_label("EXPANDED", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Left),
                        theme,
                    )
                    .with_id("expanded-left")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Right),
                        theme,
                    )
                    .with_id("expanded-right")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Up),
                        theme,
                    )
                    .with_id("expanded-up")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Down),
                        theme,
                    )
                    .with_id("expanded-down")
                )
        )
        // --- Collapsed ---
        .child(section_label("COLLAPSED", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Left)
                            .with_collapsed(true),
                        theme,
                    )
                    .with_id("collapsed-left")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Right)
                            .with_collapsed(true),
                        theme,
                    )
                    .with_id("collapsed-right")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Up)
                            .with_collapsed(true),
                        theme,
                    )
                    .with_id("collapsed-up")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Down)
                            .with_collapsed(true),
                        theme,
                    )
                    .with_id("collapsed-down")
                )
        )
        // --- Interactive (all directions, toggleable) ---
        .child(section_label("ALL DIRECTIONS (INTERACTIVE)", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(
                    PugCollapseToggle::new(
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
                    PugCollapseToggle::new(
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
                    PugCollapseToggle::new(
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
                    PugCollapseToggle::new(
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
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Left)
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled-left")
                )
                .child(
                    PugCollapseToggle::new(
                        CollapseToggleSpec::new()
                            .with_direction(CollapseDirection::Right)
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled-right")
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
