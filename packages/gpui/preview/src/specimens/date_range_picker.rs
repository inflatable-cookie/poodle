use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{DateRangePickerSpec, DateRangeValue};
use pug_gpui_components::PugDateRangePicker;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let range = DateRangeValue::new(
        Some("2026-03-10".to_string()),
        Some("2026-03-18".to_string()),
    );

    let spec = DateRangePickerSpec::new()
        .with_default_value(range)
        .with_open(true);

    div().child(PugDateRangePicker::new(spec, theme).with_id("specimen"))
}
