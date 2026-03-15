use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{DateTimePickerSpec, DateTimeValue};
use pug_gpui_components::PugDateTimePicker;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let value = DateTimeValue::new(
        Some("2026-03-15".to_string()),
        Some("09:30".to_string()),
    );

    let spec = DateTimePickerSpec::new()
        .with_default_value(value);

    div().child(PugDateTimePicker::new(spec, theme).with_id("specimen"))
}
