use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{CalendarSpec, CalendarWeekStart};
use pug_gpui_components::PugCalendar;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let spec = CalendarSpec::new()
        .with_value("2026-03-15")
        .with_visible_month("2026-03")
        .with_week_start(CalendarWeekStart::Monday);

    div().child(PugCalendar::new(spec, theme).with_id("specimen"))
}
