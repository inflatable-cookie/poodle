use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue};
use pug_gpui_components::PugDateTimeRangePicker;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let range = DateTimeRangeValue::new(
        DateTimeValue::new(
            Some("2026-03-10".to_string()),
            Some("09:00".to_string()),
        ),
        DateTimeValue::new(
            Some("2026-03-18".to_string()),
            Some("17:00".to_string()),
        ),
    );

    let spec = DateTimeRangePickerSpec::new()
        .with_default_value(range);

    div().child(PugDateTimeRangePicker::new(spec, theme).with_id("specimen"))
}
