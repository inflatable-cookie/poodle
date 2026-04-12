use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, Tooltip};
use poodle_specs::OverlayPlacement;
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec, TooltipSpec};

use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");

    // ── Default ──────────────────────────────────────────────────────
    let default_spec = TooltipSpec::new()
        .with_content("Save your changes")
        .with_placement(OverlayPlacement::Top)
        .with_default_open(true);

    let default_trigger = Button::from_spec(
        ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_label("Hover me"),
        theme,
    )
    .with_id("tooltip-default-trigger");

    let default_tooltip = Tooltip::from_spec(default_spec, theme).with_trigger(default_trigger);

    // ── Placements ───────────────────────────────────────────────────
    let top_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Top tooltip")
            .with_placement(OverlayPlacement::Top)
            .with_default_open(true),
        theme,
    )
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Top"),
            theme,
        )
        .with_id("tooltip-top-trigger"),
    );

    let bottom_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Bottom tooltip")
            .with_placement(OverlayPlacement::Bottom)
            .with_default_open(true),
        theme,
    )
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Bottom"),
            theme,
        )
        .with_id("tooltip-bottom-trigger"),
    );

    let left_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Left tooltip")
            .with_placement(OverlayPlacement::Left)
            .with_default_open(true),
        theme,
    )
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Left"),
            theme,
        )
        .with_id("tooltip-left-trigger"),
    );

    let right_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Right tooltip")
            .with_placement(OverlayPlacement::Right)
            .with_default_open(true),
        theme,
    )
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Right"),
            theme,
        )
        .with_id("tooltip-right-trigger"),
    );

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .p(px(12.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(color_to_hsla(border_subtle).opacity(0.6))
                .bg(color_to_hsla(theme.resolve_color("color.background.panel")).opacity(0.8))
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("Representative open state. Native hover-triggered tooltip behavior is still pending in the GPUI preview stack."),
        )
        // Default
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
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(default_tooltip),
                ),
        )
        // Placements
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
                        .child(top_tooltip)
                        .child(bottom_tooltip)
                        .child(left_tooltip)
                        .child(right_tooltip),
                ),
        )
}
