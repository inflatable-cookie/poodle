use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::ProgressSpec;
use pug_gpui_components::PugProgress;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(8.0))
        .child(
            div().flex().items_center().gap(px(4.0)).text_xs().text_color(color_to_hsla(text_secondary))
                .child("Home").child(div().child("/")).child("Projects").child(div().child("/")).child(div().child("Details"))
        )
        .child(div().text_lg().child("Page Header Title"))
        .child(
            PugProgress::new(ProgressSpec::new().with_indeterminate(true), theme)
        )
}
