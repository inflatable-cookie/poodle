use gpui::*;
use poodle_specs::{TooltipSpec, ButtonSpec, ButtonVariant, EyebrowSpec};
use poodle_gpui_components::{Tooltip, Button, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::OverlayPlacement;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
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
    ).with_id("tooltip-default-trigger");

    let default_tooltip = Tooltip::from_spec(default_spec, theme)
        .with_trigger(default_trigger);

    // ── Placements ───────────────────────────────────────────────────
    let top_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Top tooltip")
            .with_placement(OverlayPlacement::Top)
            .with_default_open(true),
        theme,
    ).with_trigger(
        Button::from_spec(
            ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Top"),
            theme,
        ).with_id("tooltip-top-trigger")
    );

    let bottom_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Bottom tooltip")
            .with_placement(OverlayPlacement::Bottom)
            .with_default_open(true),
        theme,
    ).with_trigger(
        Button::from_spec(
            ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Bottom"),
            theme,
        ).with_id("tooltip-bottom-trigger")
    );

    let left_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Left tooltip")
            .with_placement(OverlayPlacement::Left)
            .with_default_open(true),
        theme,
    ).with_trigger(
        Button::from_spec(
            ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Left"),
            theme,
        ).with_id("tooltip-left-trigger")
    );

    let right_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Right tooltip")
            .with_placement(OverlayPlacement::Right)
            .with_default_open(true),
        theme,
    ).with_trigger(
        Button::from_spec(
            ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Right"),
            theme,
        ).with_id("tooltip-right-trigger")
    );

    div().flex().flex_col().gap(px(24.0))
        // Default
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(12.0)).items_center()
                        .child(default_tooltip)
                )
        )
        // Placements
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Placements"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(12.0)).items_center()
                        .child(top_tooltip)
                        .child(bottom_tooltip)
                        .child(left_tooltip)
                        .child(right_tooltip)
                )
        )
}
