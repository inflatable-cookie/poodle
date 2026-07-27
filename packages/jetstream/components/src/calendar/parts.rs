//! calendar — helper builders. Split out of `calendar/mod.rs` (god-file
//! decomposition); unchanged.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};



/// Build an outside-month (adjacent-month) day cell. Contract §8 outside-month:
/// `color: text-secondary`, `opacity: 0.72` (the muted opacity token).
pub(super) fn outside_cell(
    cell_size_px: f32,
    control_radius: f32,
    day_font_px: f32,
    text_secondary: Color,
    outside_opacity: f32,
    day: u32,
) -> JsEl {
    ui_element::div()
        .aria_role(jetstream_ui::accesskit::Role::Cell)
        .w(cell_size_px)
        .h(cell_size_px)
        .items_center()
        .justify_center()
        .rounded(control_radius)
        .text_size(day_font_px)
        .text_color(text_secondary)
        .text_align_center()
        .opacity(outside_opacity)
        .child(ui_element::label(day.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use crate::calendar::js_calendar;
    use crate::presentation::rem_to_px;
    use crate::theme_ext::{resolve_color, resolve_opacity};
    use poodle_jetstream::JetstreamThemeProvider;
    use poodle_specs::{CalendarMode, CalendarSpec};
    use poodle_specs::{ControlSize, DateRangeValue};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Recursively find the first descendant (incl. self) matching `pred`.
    fn find<'a>(el: &'a JsEl, pred: &dyn Fn(&JsEl) -> bool) -> Option<&'a JsEl> {
        if pred(el) {
            return Some(el);
        }
        for c in &el.children {
            if let Some(found) = find(c, pred) {
                return Some(found);
            }
        }
        None
    }

    fn spec_with_month(m: &str) -> CalendarSpec {
        CalendarSpec::new().with_visible_month(m)
    }

    #[test]
    fn renders_month_and_year_header_controls() {
        // March 2026: month name and year both rendered as header triggers.
        let el = js_calendar(&spec_with_month("2026-03"), &theme());
        let tree = probe(&el, 320.0, 360.0);
        assert!(
            tree.has_text("March"),
            "month-name trigger missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("2026"),
            "year trigger missing: {:?}",
            tree.texts()
        );
        // Both triggers exist as distinct ids in the JsEl tree.
        assert!(
            find(&el, &|e| e.id.as_deref() == Some("poodle-cal-month-trigger")).is_some(),
            "month-trigger id missing"
        );
        assert!(
            find(&el, &|e| e.id.as_deref() == Some("poodle-cal-year-trigger")).is_some(),
            "year-trigger id missing"
        );
    }

    #[test]
    fn month_trigger_has_dashed_underline_affordance() {
        // Edit affordance = bottom border (border_b_1) on the month trigger.
        let el = js_calendar(&spec_with_month("2026-03"), &theme());
        let month_trigger = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-month-trigger"))
            .expect("month-trigger present");
        assert!(
            month_trigger.style.border_widths[2] > 0.0,
            "month trigger should carry a bottom-border underline affordance"
        );
    }

    #[test]
    fn outside_month_day_uses_muted_opacity_not_point_four() {
        // The fixed bug: outside-month days must use the muted opacity token
        // (0.72), NOT the old hardcoded 0.4.
        let th = theme();
        let muted = resolve_opacity(&th, "state.opacity.muted");
        let secondary: Color = resolve_color(&th, "color.text.secondary").into();
        let el = js_calendar(&spec_with_month("2026-03"), &th);

        // March 2026 (Monday-start) leads with several outside cells; find a
        // dimmed (opacity < 1.0) leaf carrying the secondary text color.
        let outside = find(&el, &|e| {
            e.style.opacity < 1.0
                && e.style.opacity > 0.0
                && e.style.text_color == Some(secondary)
        })
        .expect("an outside-month day cell with reduced opacity");

        assert!(
            (outside.style.opacity - muted).abs() < 0.001,
            "outside opacity {} != muted token {muted}",
            outside.style.opacity
        );
        assert!(
            (muted - 0.72).abs() < 0.001,
            "muted token should be 0.72, got {muted}"
        );
        assert!(
            (outside.style.opacity - 0.4).abs() > 0.001,
            "outside opacity must NOT be the old 0.4 value"
        );
    }

    #[test]
    fn selected_day_gets_selected_treatment() {
        // A preselected single date paints the accent fill + inverse text.
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let inverse: Color = resolve_color(&th, "color.text.inverse").into();
        let spec = CalendarSpec::new()
            .with_visible_month("2026-03")
            .with_value("2026-03-14");
        let el = js_calendar(&spec, &th);

        let selected = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-14"))
            .expect("day-14 cell present");
        assert_eq!(
            selected.style.background,
            Some(accent),
            "selected day should fill with accent"
        );
        assert_eq!(
            selected.style.text_color,
            Some(inverse),
            "selected day should use inverse text"
        );
        // And the accent fill is probe-visible in the laid-out tree.
        let tree = probe(&el, 320.0, 360.0);
        assert!(
            tree.has_background(
                crate::render_probe::ProbeColor {
                    r: accent.r,
                    g: accent.g,
                    b: accent.b,
                    a: accent.a,
                },
                0.01
            ),
            "selected accent fill missing from rendered tree"
        );
    }

    #[test]
    fn range_endpoints_get_accent_fill() {
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_visible_month("2026-03")
            .with_default_range_value(DateRangeValue::new(
                Some("2026-03-10".into()),
                Some("2026-03-20".into()),
            ));
        let el = js_calendar(&spec, &th);

        let start = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-10"))
            .expect("range start present");
        let end = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-20"))
            .expect("range end present");
        assert_eq!(start.style.background, Some(accent), "range start = accent");
        assert_eq!(end.style.background, Some(accent), "range end = accent");

        // An interior day gets the tinted in-range fill (lower alpha), not the
        // solid accent.
        let mid = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-15"))
            .expect("in-range day present");
        let mid_bg = mid.style.background.expect("in-range fill set");
        assert!(mid_bg.a < accent.a, "in-range alpha should be tinted");
    }

    #[test]
    fn per_size_scales_day_font() {
        // Day-cell font follows the calendar day-font scale, distinct per size.
        let th = theme();
        let xs = js_calendar(
            &CalendarSpec::new().with_visible_month("2026-03").with_size(ControlSize::Xs),
            &th,
        );
        let xl = js_calendar(
            &CalendarSpec::new().with_visible_month("2026-03").with_size(ControlSize::Xl),
            &th,
        );
        let xs_day = find(&xs, &|e| e.id.as_deref() == Some("poodle-cal-day-1")).unwrap();
        let xl_day = find(&xl, &|e| e.id.as_deref() == Some("poodle-cal-day-1")).unwrap();
        assert!(
            xl_day.style.text_size.unwrap() > xs_day.style.text_size.unwrap(),
            "xl day font should exceed xs day font"
        );
        assert_eq!(xs_day.style.text_size, Some(rem_to_px(0.6875)));
        assert_eq!(xl_day.style.text_size, Some(rem_to_px(0.875)));
    }

    #[test]
    fn exact_week_count_no_trailing_blank_row() {
        // Feb 2026 (Monday-start) fits in fewer than 6 weeks; the grid must
        // not render a 6th all-outside row.
        let el = js_calendar(&spec_with_month("2026-02"), &theme());
        // Day rows = children after [nav header, weekday row]. Each day row has
        // exactly 7 children.
        let day_rows: Vec<&JsEl> = el
            .children
            .iter()
            .filter(|c| c.children.len() == 7 && c.children.iter().all(|d| d.children.len() <= 1))
            .collect();
        // Feb 2026: 28 days, starts on a Sunday → Monday-start offset 6 →
        // (6 + 28) = 34 cells → 5 rows. Weekday header also has 7 children, so
        // expect 5 day rows + 1 weekday row = 6 such rows. Assert <= 6 (i.e.
        // not the old fixed 6 day rows = 7 total).
        assert!(
            day_rows.len() <= 6,
            "should not render a fixed 6 day rows for short months, got {} seven-wide rows",
            day_rows.len()
        );
    }
}

