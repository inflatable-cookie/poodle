use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, HoverCard};
use poodle_specs::{EyebrowSpec, HoverCardSpec, OverlayPlacement};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let border_subtle = theme.resolve_color("color.border.subtle");

    div().flex().flex_col().gap(px(24.0))
        .child(
            div()
                .p(px(12.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(color_to_hsla(border_subtle).opacity(0.6))
                .bg(color_to_hsla(theme.resolve_color("color.background.panel")).opacity(0.8))
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("Representative open state. Native hover enter/leave behavior is still pending in the GPUI preview stack."),
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (top placement)"), theme))
                .child(
                    HoverCard::from_spec(HoverCardSpec::new().with_open(true), theme)
                        .with_trigger(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(accent))
                                .underline()
                                .cursor_pointer()
                                .child("@clay".to_string())
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(4.0))
                                .max_w(px(256.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Clay".to_string())
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Design systems engineer working on Poodle. Loves component architecture and accessibility.".to_string())
                                )
                        )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Bottom placement"), theme))
                .child(
                    HoverCard::from_spec(
                        HoverCardSpec::new()
                            .with_open(true)
                            .with_placement(OverlayPlacement::Bottom),
                        theme,
                    )
                    .with_trigger(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(accent))
                            .underline()
                            .cursor_pointer()
                            .child("poodle/svelte-primitives".to_string())
                    )
                    .with_content(
                        div().flex().flex_col().gap(px(4.0))
                            .max_w(px(256.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("svelte-primitives".to_string())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child("Core primitive components for the Poodle design system. 64 components, 94% coverage.".to_string())
                            )
                    )
                )
        )
}
