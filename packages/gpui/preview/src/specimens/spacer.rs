use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_components::PugSpacer;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.default");

    let block = |label: &str, opacity: f32| {
        div()
            .px(px(10.0))
            .py(px(6.0))
            .rounded(px(4.0))
            .bg(color_to_hsla(accent).opacity(opacity))
            .border_1()
            .border_color(color_to_hsla(border))
            .text_xs()
            .child(label.to_string())
    };

    div().flex().flex_col().gap(px(12.0))
        .child(
            div().text_xs().child("Spacer pushes trailing item to the end"),
        )
        .child(
            div().flex().flex_row().items_center().w_full()
                .border_1()
                .border_color(color_to_hsla(border))
                .rounded(px(4.0))
                .p(px(6.0))
                .child(block("Leading", 0.2))
                .child(PugSpacer::new())
                .child(block("Trailing", 0.4))
        )
}
