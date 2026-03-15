use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{ScrollShellSpec, Direction, PaddingScale};
use pug_gpui_components::PugScrollShell;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border = theme.resolve_color("semantic.color.border.default");

    let row = |label: &str| {
        div()
            .h(px(24.0))
            .rounded(px(3.0))
            .border_1()
            .border_color(color_to_hsla(border))
            .px(px(8.0))
            .flex()
            .items_center()
            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(label.to_string()))
    };

    div().h(px(100.0)).child(
        PugScrollShell::new(
            ScrollShellSpec::new()
                .with_direction(Direction::Vertical)
                .with_padding(PaddingScale::Sm),
            theme,
        )
            .with_child(row("Scrollable row 1"))
            .with_child(row("Scrollable row 2"))
            .with_child(row("Scrollable row 3"))
            .with_child(row("Scrollable row 4"))
            .with_child(row("Scrollable row 5"))
    )
}
