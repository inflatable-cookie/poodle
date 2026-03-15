use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BoxSpec, PaddingScale};
use pug_gpui_components::PugBox;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.default");

    let swatch = |opacity: f32| {
        div()
            .w(px(60.0))
            .h(px(40.0))
            .rounded(px(4.0))
            .bg(color_to_hsla(accent).opacity(opacity))
            .border_1()
            .border_color(color_to_hsla(border))
    };

    div().flex().gap(px(8.0))
        .child(
            PugBox::new(BoxSpec::new().with_padding(PaddingScale::Sm), theme)
                .with_child(swatch(0.2))
        )
        .child(
            PugBox::new(BoxSpec::new().with_padding(PaddingScale::Md), theme)
                .with_child(swatch(0.4))
        )
        .child(
            PugBox::new(BoxSpec::new().with_padding(PaddingScale::Lg), theme)
                .with_child(swatch(0.6))
        )
}
