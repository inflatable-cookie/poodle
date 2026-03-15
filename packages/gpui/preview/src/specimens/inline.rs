use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_components::PugInline;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.default");

    let chip = |label: &str, opacity: f32| {
        div()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(4.0))
            .bg(color_to_hsla(accent).opacity(opacity))
            .border_1()
            .border_color(color_to_hsla(border))
            .text_xs()
            .child(label.to_string())
    };

    div().flex().flex_col().gap(px(12.0))
        .child(
            div().text_xs().child("Default gap (8px)"),
        )
        .child(
            PugInline::new(theme)
                .child(chip("Alpha", 0.15))
                .child(chip("Beta", 0.25))
                .child(chip("Gamma", 0.35))
        )
        .child(
            div().text_xs().child("Large gap (spacing.lg)"),
        )
        .child(
            PugInline::new(theme)
                .with_gap("semantic.spacing.lg")
                .child(chip("One", 0.2))
                .child(chip("Two", 0.3))
                .child(chip("Three", 0.4))
        )
}
