use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{GridSpec, PaddingScale};
use pug_gpui_components::PugGrid;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("semantic.color.border.default");

    let cell = || {
        div()
            .h(px(32.0))
            .rounded(px(3.0))
            .border_1()
            .border_color(color_to_hsla(border))
    };

    div().child(
        PugGrid::new(
            GridSpec::new()
                .with_columns("1fr 1fr 1fr")
                .with_gap(PaddingScale::Sm),
            theme,
        )
            .with_child(cell())
            .with_child(cell())
            .with_child(cell())
            .with_child(cell())
            .with_child(cell())
            .with_child(cell())
    )
}
