use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{DateRangeValue, EyebrowSpec, RangeCalendarSpec};
use flint_gpui_components::{Eyebrow, RangeCalendar};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let mut spec = RangeCalendarSpec::new();
                    spec.aria_label = Some("Select a date range".to_string());
                    RangeCalendar::from_spec(spec, theme).with_id("default")
                })
        )
        // --- With pre-selected range ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With pre-selected range"), theme))
                .child({
                    let range = DateRangeValue::new(
                        Some("2026-03-05".to_string()),
                        Some("2026-03-12".to_string()),
                    );
                    let mut spec = RangeCalendarSpec::new()
                        .with_default_value(range);
                    spec.aria_label = Some("Pre-selected range".to_string());
                    RangeCalendar::from_spec(spec, theme).with_id("preselected")
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = RangeCalendarSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range calendar".to_string());
                    RangeCalendar::from_spec(spec, theme).with_id("disabled")
                })
        )
}
