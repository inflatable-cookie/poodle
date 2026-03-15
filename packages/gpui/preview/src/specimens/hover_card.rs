use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::HoverCardSpec;
use pug_gpui_components::PugHoverCard;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("semantic.color.border.default");

    let closed_spec = HoverCardSpec::new();

    let open_spec = HoverCardSpec::new()
        .with_open(true);

    div().flex().flex_col().gap(px(12.0))
        .child(
            div().text_xs().child("Trigger (card closed)"),
        )
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(
                    div()
                        .px(px(12.0)).py(px(6.0))
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .text_sm()
                        .child("Hover target")
                )
                .child(PugHoverCard::new(closed_spec, theme)
                    .with_content(div().text_sm().child("Hidden card")))
        )
        .child(
            div().text_xs().child("Open state"),
        )
        .child(
            PugHoverCard::new(open_spec, theme)
                .with_content(
                    div().flex().flex_col().gap(px(4.0))
                        .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child("Hover Card"))
                        .child(div().text_xs().child("Additional details shown on hover."))
                )
        )
}
