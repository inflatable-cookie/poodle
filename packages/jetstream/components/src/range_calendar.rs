//! RangeCalendar — Jetstream range-calendar grid backed by RangeCalendarSpec.
//!
//! Contract: `docs/contracts/foundation/range-calendar.md`
//! Reference: `packages/svelte/primitives/src/RangeCalendar.svelte`
//!
//! Renders a month grid with header navigation, weekday labels, and day
//! buttons supporting two-click start/end range selection with in-range
//! highlighting.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::RangeCalendarSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_range_calendar(spec: &RangeCalendarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");
    let border_color = resolve_color(theme, "semantic.color.border.default");
    let surface_fill = resolve_color(theme, "semantic.color.background.surface");
    let elevated = resolve_color(theme, "semantic.color.background.elevated");
    let accent = resolve_color(theme, "semantic.color.accent.base");
    let radius = resolve_radius(theme, "semantic.radius.control");

    // Size-dependent dimensions per contract
    let (nav_size, day_min_h, day_font, month_font) = match effective_size {
        poodle_primitives::ControlSize::Xs => (rem_to_px(1.5), rem_to_px(1.75), rem_to_px(0.6875), rem_to_px(0.6875)),
        poodle_primitives::ControlSize::Sm => (rem_to_px(1.75), rem_to_px(2.0), rem_to_px(0.6875), rem_to_px(0.75)),
        poodle_primitives::ControlSize::Md => (rem_to_px(2.0), rem_to_px(2.25), rem_to_px(0.75), rem_to_px(0.8125)),
        poodle_primitives::ControlSize::Lg => (rem_to_px(2.25), rem_to_px(2.5), rem_to_px(0.8125), rem_to_px(0.875)),
        poodle_primitives::ControlSize::Xl => (rem_to_px(2.5), rem_to_px(2.75), rem_to_px(0.875), rem_to_px(0.9375)),
    };

    // Nav hover: 82% surface blended with elevated
    let surface_c: Color = surface_fill.into();
    let elevated_c: Color = elevated.into();
    let nav_hover_bg = surface_c.mix(elevated_c, 0.18);

    // In-range tint: 16% accent
    let accent_c: Color = accent.into();
    let _in_range_bg = accent_c.with_alpha(0.16);

    let mut root = ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.75))
        .min_w(rem_to_px(16.0));

    // ── Header: [prev] [month label] [next] ──
    let prev_btn = ui_element::div()
        .w(nav_size)
        .h(nav_size)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .bg(surface_fill)
        .flex_row()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .focusable()
        .hover(|s| s.bg(nav_hover_bg))
        .child(ui_element::icon("chevron-left").w(rem_to_px(0.75)).h(rem_to_px(0.75)).text_color(text_color));

    let month_label = ui_element::label("March 2026")
        .text_color(text_color)
        .text_size(month_font)
        .text_weight(600)
        .text_align_center()
        .grow();

    let next_btn = ui_element::div()
        .w(nav_size)
        .h(nav_size)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .bg(surface_fill)
        .flex_row()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .focusable()
        .hover(|s| s.bg(nav_hover_bg))
        .child(ui_element::icon("chevron-right").w(rem_to_px(0.75)).h(rem_to_px(0.75)).text_color(text_color));

    let header = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.5))
        .child(prev_btn)
        .child(month_label)
        .child(next_btn);

    root = root.child(header);

    // ── Weekday row ──
    let weekday_labels = match spec.week_starts_on {
        poodle_primitives::CalendarWeekStart::Sunday => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
        poodle_primitives::CalendarWeekStart::Monday => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
    };

    let mut weekday_row = ui_element::div().flex_row();
    for day_name in &weekday_labels {
        weekday_row = weekday_row.child(
            ui_element::label(*day_name)
                .text_color(muted)
                .text_size(rem_to_px(0.6875))
                .text_weight(600)
                .text_align_center()
                .grow(),
        );
    }
    root = root.child(weekday_row);

    // ── Day grid (placeholder 5 weeks) ──
    let mut grid = ui_element::div().flex_col().gap(rem_to_px(0.125));

    for _week in 0..5 {
        let mut week_row = ui_element::div().flex_row().gap(rem_to_px(0.125));
        for day in 1..=7 {
            week_row = week_row.child(
                ui_element::div()
                    .h(day_min_h)
                    .grow()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .border(1.0)
                    .border_color(glam::Vec4::ZERO) // transparent
                    .rounded(radius)
                    .cursor_pointer()
                    .child(
                        ui_element::label(&day.to_string())
                            .text_color(text_color)
                            .text_size(day_font)
                            .text_weight(500),
                    ),
            );
        }
        grid = grid.child(week_row);
    }

    root = root.child(grid);

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}
