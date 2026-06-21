use crate::app_state::AppState;
use crate::specimens::overlay_state;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, Popover};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, OverlayPlacement, PopoverSpec, PopoverSurfaceWidth,
};

/// One labelled popover example. The popover renders its open surface inline
/// (the established GPUI specimen pattern); `on_open_change` toggles the
/// preview's overlay state so the trigger flow stays interactive.
fn popover_case(
    state: &AppState,
    theme: &GpuiThemeProvider,
    root: &WeakEntity<PreviewRoot>,
    key: &'static str,
    spec: PopoverSpec,
    trigger_label: &str,
    content: AnyElement,
) -> Popover {
    let is_open = state.specimens.is_on(key);
    Popover::from_spec(spec.with_open(is_open), theme)
        .on_open_change({
            let root = root.clone();
            move |open, _window, cx| {
                overlay_state::set_toggle_via_entity(&root, key, open, cx);
            }
        })
        .with_trigger(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label(trigger_label),
                theme,
            )
            .with_id(format!("{key}-trigger")),
        )
        .with_content(content)
}

fn labelled(theme: &GpuiThemeProvider, label: &str, body: Popover) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(body)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");
    let root = cx.weak_entity();

    // Heading + paragraph content block (matches the Svelte specimen body).
    let heading_paragraph = |heading: &str, body: &str| -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(color_to_hsla(text_primary))
                    .child(heading.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child(body.to_string()),
            )
            .into_any_element()
    };

    let paragraph = |body: &str| -> AnyElement {
        div()
            .text_xs()
            .text_color(color_to_hsla(text_secondary))
            .child(body.to_string())
            .into_any_element()
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default (bottom-start) — contract §13 ---
        .child(labelled(
            theme,
            "Default (bottom-start)",
            popover_case(
                state,
                theme,
                &root,
                "popover-default",
                PopoverSpec::new().with_aria_label("Quick settings"),
                "Open popover",
                heading_paragraph(
                    "Quick settings",
                    "Adjust your display preferences or notification settings from this panel.",
                ),
            ),
        ))
        // --- Top placement — contract §13 ---
        .child(labelled(
            theme,
            "Top placement",
            popover_case(
                state,
                theme,
                &root,
                "popover-top",
                PopoverSpec::new()
                    .with_placement(OverlayPlacement::Top)
                    .with_aria_label("Help tip"),
                "Show help",
                paragraph("Popovers can be anchored to any side of their trigger element."),
            ),
        ))
        // --- Placements (left / right) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Placement (left / right)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(48.0))
                        .child(popover_case(
                            state,
                            theme,
                            &root,
                            "popover-left",
                            PopoverSpec::new()
                                .with_placement(OverlayPlacement::Left)
                                .with_aria_label("Left popover"),
                            "Left",
                            paragraph("Anchored to the left of the trigger."),
                        ))
                        .child(popover_case(
                            state,
                            theme,
                            &root,
                            "popover-right",
                            PopoverSpec::new()
                                .with_placement(OverlayPlacement::Right)
                                .with_aria_label("Right popover"),
                            "Right",
                            paragraph("Anchored to the right of the trigger."),
                        )),
                ),
        )
        // --- Surface width: trigger ---
        .child(labelled(
            theme,
            "Surface width: trigger",
            popover_case(
                state,
                theme,
                &root,
                "popover-surface-trigger",
                PopoverSpec::new()
                    .with_surface_width(PopoverSurfaceWidth::Trigger)
                    .with_aria_label("Trigger-width popover"),
                "Match trigger width",
                paragraph("The surface stretches to the trigger's width."),
            ),
        ))
        // --- Surface width: fixed (min/max overrides) ---
        .child(labelled(
            theme,
            "Surface width: fixed",
            popover_case(
                state,
                theme,
                &root,
                "popover-surface-fixed",
                PopoverSpec::new()
                    .with_surface_min_width_rem(20.0)
                    .with_surface_max_width_rem(20.0)
                    .with_aria_label("Fixed-width popover"),
                "Fixed 20rem",
                paragraph("Surface min-width and max-width are pinned to 20rem."),
            ),
        ))
        // --- Disabled (trigger cannot open) ---
        .child(labelled(
            theme,
            "Disabled",
            popover_case(
                state,
                theme,
                &root,
                "popover-disabled",
                PopoverSpec::new()
                    .with_disabled(true)
                    .with_aria_label("Disabled popover"),
                "Disabled trigger",
                paragraph("A disabled trigger blocks opening."),
            ),
        ))
}
