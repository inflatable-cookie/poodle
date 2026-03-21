use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{TooltipSpec, ButtonSpec, ButtonVariant};
use pug_gpui_components::{Tooltip, Button};
use pug_gpui::GpuiThemeProvider;
use pug_primitives::OverlayPlacement;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // ── Default ──────────────────────────────────────────────────────
    // Contract: "Hover me" button (secondary variant), content="Save your changes", placement=top
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
    // Contract: 2x2 grid of Top, Bottom, Left, Right tooltips with ghost buttons
    let top_spec = TooltipSpec::new()
        .with_content("Top tooltip")
        .with_placement(OverlayPlacement::Top)
        .with_default_open(true);

    let top_tooltip = Tooltip::from_spec(top_spec, theme)
        .with_trigger(
            Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Top"),
                theme,
            ).with_id("tooltip-top-trigger")
        );

    let bottom_spec = TooltipSpec::new()
        .with_content("Bottom tooltip")
        .with_placement(OverlayPlacement::Bottom)
        .with_default_open(true);

    let bottom_tooltip = Tooltip::from_spec(bottom_spec, theme)
        .with_trigger(
            Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Bottom"),
                theme,
            ).with_id("tooltip-bottom-trigger")
        );

    let left_spec = TooltipSpec::new()
        .with_content("Left tooltip")
        .with_placement(OverlayPlacement::Left)
        .with_default_open(true);

    let left_tooltip = Tooltip::from_spec(left_spec, theme)
        .with_trigger(
            Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Left"),
                theme,
            ).with_id("tooltip-left-trigger")
        );

    let right_spec = TooltipSpec::new()
        .with_content("Right tooltip")
        .with_placement(OverlayPlacement::Right)
        .with_default_open(true);

    let right_tooltip = Tooltip::from_spec(right_spec, theme)
        .with_trigger(
            Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Right"),
                theme,
            ).with_id("tooltip-right-trigger")
        );

    div().flex().flex_col().gap(px(16.0))
        // Default
        .child(section_label("DEFAULT", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(12.0)).items_center()
                .child(default_tooltip)
        )

        // Placements
        .child(section_label("PLACEMENTS", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(12.0)).items_center()
                .child(top_tooltip)
                .child(bottom_tooltip)
                .child(left_tooltip)
                .child(right_tooltip)
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
