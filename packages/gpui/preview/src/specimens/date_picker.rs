use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::DatePickerSpec;
use pug_gpui_components::PugDatePicker;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let closed = DatePickerSpec::new()
        .with_default_value("2026-03-15");

    let open = DatePickerSpec::new()
        .with_default_value("2026-03-15")
        .with_default_open(true);

    div().flex().flex_col().gap(px(12.0))
        .child(PugDatePicker::new(closed, theme).with_id("closed"))
        .child(PugDatePicker::new(open, theme).with_id("open"))
}
