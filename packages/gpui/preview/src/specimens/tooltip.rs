use crate::app_state::AppState;
use crate::node_compat::{Button, Eyebrow, Tooltip};
use crate::specimens::overlay_state;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec, OverlayPlacement,
    TooltipSpec,
};

fn render_tooltip_case(
    state: &AppState,
    theme: &GpuiThemeProvider,
    root: &WeakEntity<PreviewRoot>,
    open_key: impl Into<String>,
    hovered_key: impl Into<String>,
    tooltip_spec: TooltipSpec,
    trigger: AnyElement,
) -> AnyElement {
    let open_key = open_key.into();
    let hovered_key = hovered_key.into();
    let is_open = state.specimens.is_on(&open_key);
    let delay_ms = u32::from(tooltip_spec.delay_ms);

    Tooltip::from_spec(tooltip_spec.with_open(is_open), theme)
        .on_open_change({
            let root = root.clone();
            let open_key = open_key.clone();
            let hovered_key = hovered_key.clone();
            move |open, window, cx| {
                overlay_state::sync_hover_intent(
                    &root,
                    open_key.clone(),
                    hovered_key.clone(),
                    open,
                    cx,
                );
                if open {
                    overlay_state::schedule_toggle_if(
                        window,
                        cx,
                        root.clone(),
                        hovered_key.clone(),
                        true,
                        open_key.clone(),
                        true,
                        delay_ms,
                    );
                }
            }
        })
        .with_trigger(trigger)
        .into_any_element()
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let root = cx.weak_entity();
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
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(
                    div().flex().flex_wrap().gap(px(12.0)).items_center().child(
                        render_tooltip_case(
                            state,
                            theme,
                            &root,
                            "tooltip-default-open",
                            "tooltip-default-hovered",
                            TooltipSpec::new()
                                .with_content("Save your changes")
                                .with_placement(OverlayPlacement::Top),
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Hover me"),
                                theme,
                            )
                            .with_id("tooltip-default-trigger")
                            .into_any_element(),
                        ),
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Placements"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(render_tooltip_case(
                            state,
                            theme,
                            &root,
                            "tooltip-top-open",
                            "tooltip-top-hovered",
                            TooltipSpec::new()
                                .with_content("Top tooltip")
                                .with_placement(OverlayPlacement::Top),
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_label("Top"),
                                theme,
                            )
                            .with_id("tooltip-top-trigger")
                            .into_any_element(),
                        ))
                        .child(render_tooltip_case(
                            state,
                            theme,
                            &root,
                            "tooltip-bottom-open",
                            "tooltip-bottom-hovered",
                            TooltipSpec::new()
                                .with_content("Bottom tooltip")
                                .with_placement(OverlayPlacement::Bottom),
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_label("Bottom"),
                                theme,
                            )
                            .with_id("tooltip-bottom-trigger")
                            .into_any_element(),
                        ))
                        .child(render_tooltip_case(
                            state,
                            theme,
                            &root,
                            "tooltip-left-open",
                            "tooltip-left-hovered",
                            TooltipSpec::new()
                                .with_content("Left tooltip")
                                .with_placement(OverlayPlacement::Left),
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_label("Left"),
                                theme,
                            )
                            .with_id("tooltip-left-trigger")
                            .into_any_element(),
                        ))
                        .child(render_tooltip_case(
                            state,
                            theme,
                            &root,
                            "tooltip-right-open",
                            "tooltip-right-hovered",
                            TooltipSpec::new()
                                .with_content("Right tooltip")
                                .with_placement(OverlayPlacement::Right),
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_label("Right"),
                                theme,
                            )
                            .with_id("tooltip-right-trigger")
                            .into_any_element(),
                        )),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "tooltip",
        examples,
        |size: ControlSize, theme: &GpuiThemeProvider| {
            let size_label = match size {
                ControlSize::Xs => "xs",
                ControlSize::Sm => "sm",
                ControlSize::Md => "md",
                ControlSize::Lg => "lg",
                ControlSize::Xl => "xl",
            };
            render_tooltip_case(
                state,
                theme,
                &root,
                format!("tooltip-size-{size_label}-open"),
                format!("tooltip-size-{size_label}-hovered"),
                TooltipSpec::new().with_content(format!("Tooltip at {size_label}")),
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(size)
                        .with_label(format!("Hover ({size_label})")),
                    theme,
                )
                .with_id(format!("tooltip-size-{size_label}-trigger"))
                .into_any_element(),
            )
        },
        |density: ControlDensity, theme: &GpuiThemeProvider| {
            let density_label = match density {
                ControlDensity::Compact => "compact",
                ControlDensity::Default => "default",
                ControlDensity::Comfortable => "comfortable",
            };
            render_tooltip_case(
                state,
                theme,
                &root,
                format!("tooltip-density-{density_label}-open"),
                format!("tooltip-density-{density_label}-hovered"),
                TooltipSpec::new().with_content(format!("Tooltip at {density_label}")),
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_density(density)
                        .with_label(format!("Hover ({density_label})")),
                    theme,
                )
                .with_id(format!("tooltip-density-{density_label}-trigger"))
                .into_any_element(),
            )
        },
    )
}
