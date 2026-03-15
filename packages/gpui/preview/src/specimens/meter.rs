use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ProgressSpec;
use pug_gpui_components::PugProgress;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(4.0))
        .child(div().flex().justify_between()
            .child(div().text_xs().child("Storage"))
            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("7.2 / 10 GB")))
        .child(
            PugProgress::new(ProgressSpec::new().with_value(0.72), theme)
        )
}
