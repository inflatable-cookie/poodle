use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{RangeCalendarSpec, DateRangeValue};
use pug_gpui_components::PugRangeCalendar;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let range = DateRangeValue::new(
        Some("2026-03-10".to_string()),
        Some("2026-03-18".to_string()),
    );

    let mut spec = RangeCalendarSpec::new()
        .with_default_value(range);
    spec.visible_month = Some("2026-03".to_string());

    div().child(PugRangeCalendar::new(spec, theme).with_id("specimen"))
}
