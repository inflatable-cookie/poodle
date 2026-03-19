use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, PopoverSpec, OverlayPlacement};
use pug_gpui_components::{PugButton, PugPopover};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let popover_default_open = state.specimens.is_on("popover-default");
    let popover_top_open = state.specimens.is_on("popover-top");

    div().flex().flex_col().gap(px(16.0))
        // --- Default (bottom-start) ---
        .child(section_label("DEFAULT (BOTTOM-START)", text_secondary))
        .child(
            PugPopover::new(
                PopoverSpec::new()
                    .with_open(popover_default_open)
                    .with_aria_label("Quick settings"),
                theme,
            )
            .with_trigger(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Open popover"),
                    theme,
                )
                .with_id("popover-default-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggle("popover-default");
                    cx.notify();
                }))
            )
            .with_content(
                div().flex().flex_col().gap(px(4.0)).max_w(px(256.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(color_to_hsla(text_primary))
                            .child("Quick settings")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child("Adjust your display preferences or notification settings from this panel.")
                    )
            )
        )
        // --- Top placement ---
        .child(section_label("TOP PLACEMENT", text_secondary))
        .child(
            PugPopover::new(
                PopoverSpec::new()
                    .with_open(popover_top_open)
                    .with_placement(OverlayPlacement::Top)
                    .with_aria_label("Help tip"),
                theme,
            )
            .with_trigger(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Show help"),
                    theme,
                )
                .with_id("popover-top-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggle("popover-top");
                    cx.notify();
                }))
            )
            .with_content(
                div().max_w(px(256.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child("Popovers can be anchored to any side of their trigger element.")
                    )
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
