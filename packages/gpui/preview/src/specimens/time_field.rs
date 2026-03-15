use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TimeFieldSpec;
use pug_gpui_components::PugTimeField;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let with_value = TimeFieldSpec::new()
        .with_default_value("09:30");

    let empty = TimeFieldSpec::new();

    div().flex().flex_col().gap(px(8.0))
        .child(PugTimeField::new(with_value, theme).with_id("filled"))
        .child(PugTimeField::new(empty, theme).with_id("empty"))
}
