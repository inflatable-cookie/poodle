use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{StackSpec, PaddingScale};
use pug_gpui_components::PugStack;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let item = |label: &str| {
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

    div().child(
        PugStack::new(StackSpec::new().with_gap(PaddingScale::Sm), theme)
            .with_child(item("Item 1"))
            .with_child(item("Item 2"))
            .with_child(item("Item 3"))
    )
}
