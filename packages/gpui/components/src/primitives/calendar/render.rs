//! Calendar — full render body (nav header, weekday row, day grid).
//!
//! Split out of `calendar/mod.rs` (god-file decomposition); the date math
//! lives in `poodle-headless`, so this is a cohesive element-tree builder.
//! Behavior unchanged.

use gpui::*;
use poodle_specs::{
    CalendarMode, DateRangeValue,
    IconSize, IconSpec,
};

use crate::primitives::icon::Icon;
use crate::presentation::{
    calendar_cell_size_rem, calendar_day_font_rem, calendar_nav_size_rem, rem_to_px,
    resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

use super::Calendar;

impl Calendar {
    pub(super) fn render(self) -> AnyElement {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        // Per-size calendar metrics, all from the contract size table:
        //   cell-size (grid column + day min-height), nav button, day font,
        //   month-label font. These are calendar-specific scales.
        let cell_size = px(rem_to_px(calendar_cell_size_rem(effective_size)));
        let nav_btn_size = px(rem_to_px(calendar_nav_size_rem(effective_size)));
        let day_font = px(rem_to_px(calendar_day_font_rem(effective_size)));
        // Month-label font scales per size (xs 0.6875 … xl 0.9375rem) and
        // matches `size_font_rem`.
        let month_label_font = px(rem_to_px(size_font_rem(effective_size)));

        let control_radius = resolve_radius(theme, "radius.control");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let gap_sm = resolve_px(theme, "space.inline.sm");

        let accent = resolve_color(theme, "color.accent.base");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_inverse = resolve_color(theme, "color.text.inverse");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border = resolve_color(theme, "color.border.default");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        // Outside-month days dim to the muted opacity token (0.72), not a
        // raw literal. Svelte: `.poodle-calendar__day[data-current-month=false] { opacity: 0.72 }`.
        let outside_opacity = resolve_opacity(theme, "state.opacity.muted");
        let body_size = day_font;
        let white = Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 };

        // Svelte: day hover bg = color-mix(accent 14%, transparent)
        let hover_bg = Hsla { a: accent.a * 0.14, ..accent };
        // Svelte: day hover border = color-mix(accent 46%, border-default)
        let hover_border = color_mix(accent, border, 0.46);
        // Svelte: today cell border = color-mix(accent 44%, border-default)
        let today_border = color_mix(accent, border, 0.44);
        // Svelte: selected / range-endpoint hover = color-mix(accent 88%, white 8%).
        // Closest faithful 2-color blend (matches button.rs danger-hover pattern).
        let selected_hover_bg = color_mix(accent, white, 0.88);

        let selected_date = spec.current_value().map(|s| s.to_string());
        let selected_day = selected_date.as_deref().and_then(Self::parse_day);

        // Range-mode state: resolve start / end ISO strings once. Cells
        // compare their own YYYY-MM-DD against these to decide whether
        // they are the range start, end, or an interior day.
        let is_range_mode = spec.mode == CalendarMode::Range;
        let (range_start_iso, range_end_iso) = if is_range_mode {
            let range = spec.current_range_value();
            (range.start.clone(), range.end.clone())
        } else {
            (None, None)
        };

        // Determine which month to show
        let (year, month) = spec
            .effective_visible_month()
            .and_then(Self::parse_year_month)
            .unwrap_or((2026, 1));

        // Compute "today" for highlighting (YYYY-MM-DD)
        // Simple approach: check if today is in the visible month
        let today_day: Option<u32> = {
            // Use a static today value for consistency within a single render
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let days_since_epoch = now / 86400;
            // Approximate date calculation
            let (ty, tm, td) = Self::days_to_ymd(days_since_epoch as i64);
            if ty == year && tm == month {
                Some(td)
            } else {
                None
            }
        };

        let days_count = Self::days_in_month(year, month);
        let first_dow = Self::first_day_of_week(year, month);

        // Determine week-start offset (Sunday = 0, Monday = 1)
        let week_start_offset: u32 = match spec.week_starts_on {
            poodle_specs::CalendarWeekStart::Monday => 1,
            _ => 0,
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-calendar-{}", suffix)
        } else {
            "poodle-calendar".to_string()
        };

        let month_names = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let month_name = month_names
            .get(month as usize - 1)
            .copied()
            .unwrap_or("")
            .to_string();
        let year_label = format!("{}", year);

        let surface_radius = resolve_radius(theme, "radius.surface");

        // Build the calendar container.
        // Svelte uses `width: fit-content`; we derive the equivalent from the
        // grid: 7 columns of `cell_size`, 6 inter-column gaps (0.125rem), plus
        // the root's 0.75rem padding on each side. Tracks cell size per size.
        let grid_gap = px(rem_to_px(0.125));
        let root_pad = px(rem_to_px(0.75));
        let cell_size_rem = calendar_cell_size_rem(effective_size);
        let calendar_width =
            px(rem_to_px(cell_size_rem * 7.0 + 0.125 * 6.0 + 0.75 * 2.0));

        let mut cal = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .flex_col()
            .gap(gap_sm)
            .p(root_pad)
            .w(calendar_width)
            .rounded(surface_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border);

        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        cal = cal.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if spec.is_disabled {
            cal = cal
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Compute prev/next month strings for navigation
        let prev_month_str = {
            let (py, pm) = if month == 1 {
                (year - 1, 12)
            } else {
                (year, month - 1)
            };
            format!("{:04}-{:02}", py, pm)
        };
        let next_month_str = {
            let (ny, nm) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
            format!("{:04}-{:02}", ny, nm)
        };

        // Contract: nav header with prev/next month buttons and centered month label
        // Svelte: nav hover = color-mix(surface 82%, elevated)
        let nav_btn_hover = color_mix(surface_bg, elevated_bg, 0.82);

        let mut prev_btn = div()
            .id("poodle-cal-prev")
            .w(nav_btn_size)
            .h(nav_btn_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded(control_radius)
            // Svelte: border 1px border-default, bg surface
            .border_1()
            .border_color(border)
            .bg(surface_bg)
            .cursor_pointer()
            .hover(move |s| s.bg(nav_btn_hover))
            .child(
                Icon::from_spec(IconSpec::new("chevron-left").with_size(IconSize::Sm), theme)
                    .with_color(icon_muted),
            );

        if let Some(ref handler) = self.on_navigate {
            let handler = handler.clone();
            let prev = prev_month_str.clone();
            prev_btn = prev_btn.on_click(move |_event, window, cx| {
                handler(&prev, window, cx);
            });
        }

        let mut next_btn = div()
            .id("poodle-cal-next")
            .w(nav_btn_size)
            .h(nav_btn_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded(control_radius)
            // Svelte: border 1px border-default, bg surface
            .border_1()
            .border_color(border)
            .bg(surface_bg)
            .cursor_pointer()
            .hover(move |s| s.bg(nav_btn_hover))
            .child(
                Icon::from_spec(
                    IconSpec::new("chevron-right").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_muted),
            );

        if let Some(ref handler) = self.on_navigate {
            let handler = handler.clone();
            let next = next_month_str.clone();
            next_btn = next_btn.on_click(move |_event, window, cx| {
                handler(&next, window, cx);
            });
        }

        // Month Label = composed Month Trigger + Year Trigger (contract §2).
        // Each is a button-like control with a dashed-underline edit
        // affordance (Svelte `.poodle-calendar__month-button` /
        // `.poodle-calendar__year-button`). Double-click-to-edit and the
        // inline select/input editors are preview-loop interaction; here we
        // render the editable controls at the current month/year.
        let border_width = px(rem_to_px(0.0625));
        // Svelte: underline = color-mix(text-secondary 72%, transparent)
        let trigger_underline = Hsla { a: text_secondary.a * 0.72, ..text_secondary };
        // Svelte hover: underline = color-mix(accent 72%, transparent), text-primary
        let trigger_underline_hover = Hsla { a: accent.a * 0.72, ..accent };

        let make_trigger = |id: &str, label: String, disabled: bool| {
            let mut t = div()
                .id(SharedString::from(format!("poodle-cal-{}", id)))
                .text_size(month_label_font)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .border_b(border_width)
                .border_color(trigger_underline)
                .child(label);
            if disabled {
                t = t.cursor(CursorStyle::OperationNotAllowed);
            } else {
                t = t.cursor(CursorStyle::IBeam).hover(move |s| {
                    s.text_color(text_primary)
                        .border_color(trigger_underline_hover)
                });
            }
            t
        };

        let month_label_control = div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .gap(px(rem_to_px(0.375)))
            .child(make_trigger("month-trigger", month_name, spec.is_disabled))
            .child(make_trigger("year-trigger", year_label, spec.is_disabled));

        let nav_header = div()
            .flex()
            .items_center()
            .justify_between()
            .py(gap_sm)
            .child(prev_btn)
            .child(month_label_control)
            .child(next_btn);

        cal = cal.child(nav_header);

        // Keyboard navigation: arrow keys move day selection, PageUp/PageDown change month
        if !spec.is_disabled {
            let key_select = self.on_select.clone();
            let key_navigate = self.on_navigate.clone();
            let key_year = year;
            let key_month = month;
            let key_days = days_count;
            let key_selected = selected_date.clone();
            let key_prev = prev_month_str.clone();
            let key_next = next_month_str.clone();

            cal = cal.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                match key {
                    "left" | "right" | "up" | "down" => {
                        let current_day = key_selected
                            .as_deref()
                            .and_then(|s| {
                                let parts: Vec<&str> = s.split('-').collect();
                                if parts.len() == 3 {
                                    let sy = parts[0].parse::<i32>().ok()?;
                                    let sm = parts[1].parse::<u32>().ok()?;
                                    let sd = parts[2].parse::<u32>().ok()?;
                                    if sy == key_year && sm == key_month {
                                        Some(sd)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(1);

                        let delta: i32 = match key {
                            "left" => -1,
                            "right" => 1,
                            "up" => -7,
                            "down" => 7,
                            _ => 0,
                        };

                        let new_day = current_day as i32 + delta;
                        if new_day >= 1 && new_day <= key_days as i32 {
                            if let Some(ref handler) = key_select {
                                let date_str =
                                    format!("{:04}-{:02}-{:02}", key_year, key_month, new_day);
                                handler(&date_str, window, cx);
                            }
                        } else if new_day < 1 {
                            if let Some(ref handler) = key_navigate {
                                handler(&key_prev, window, cx);
                            }
                        } else {
                            if let Some(ref handler) = key_navigate {
                                handler(&key_next, window, cx);
                            }
                        }
                    }
                    "pageup" => {
                        if let Some(ref handler) = key_navigate {
                            handler(&key_prev, window, cx);
                        }
                    }
                    "pagedown" => {
                        if let Some(ref handler) = key_navigate {
                            handler(&key_next, window, cx);
                        }
                    }
                    _ => {}
                }
            });
        }

        // Weekday headers row
        // Contract: weekday font 0.6875rem (11px), weight 600, uppercase.
        // Row height is a fixed 1.5rem (no token; contract-exact rem).
        let weekday_row_height = px(rem_to_px(1.5));
        let mut header_row = div().flex().gap(grid_gap);
        for i in 0..7u32 {
            let idx = ((i + week_start_offset) % 7) as usize;
            header_row = header_row.child(
                div()
                    .w(cell_size)
                    .h(weekday_row_height)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(caption_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_secondary)
                    .child(super::WEEKDAYS_SUN[idx].to_uppercase()),
            );
        }
        cal = cal.child(header_row);

        // Day grid — build rows of 7
        let start_offset = (first_dow + 7 - week_start_offset) % 7;
        let total_cells = start_offset + days_count;
        let rows = (total_cells + 6) / 7;

        // Compute previous month's days for leading outside-month cells
        let (prev_year, prev_month) = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };
        let prev_month_days = Self::days_in_month(prev_year, prev_month);

        for row in 0..rows {
            let mut day_row = div().flex().gap(grid_gap);
            for col in 0..7u32 {
                let cell_idx = row * 7 + col;
                if cell_idx < start_offset {
                    // Outside-month cell (previous month)
                    let outside_day = prev_month_days - (start_offset - cell_idx - 1);
                    day_row = day_row.child(
                        div()
                            .w(cell_size)
                            .h(cell_size)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(body_size)
                            // Svelte: color text-secondary at muted opacity (0.72)
                            .text_color(text_secondary)
                            .opacity(outside_opacity)
                            .child(format!("{}", outside_day)),
                    );
                } else if cell_idx >= start_offset + days_count {
                    // Outside-month cell (next month)
                    let outside_day = cell_idx - start_offset - days_count + 1;
                    day_row = day_row.child(
                        div()
                            .w(cell_size)
                            .h(cell_size)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(body_size)
                            // Svelte: color text-secondary at muted opacity (0.72)
                            .text_color(text_secondary)
                            .opacity(outside_opacity)
                            .child(format!("{}", outside_day)),
                    );
                } else {
                    let day_num = cell_idx - start_offset + 1;
                    let date_iso = format!("{:04}-{:02}-{:02}", year, month, day_num);
                    let is_selected = !is_range_mode && selected_day == Some(day_num);
                    let is_today = today_day == Some(day_num);

                    // Range-mode state for this cell
                    let is_range_start =
                        is_range_mode && range_start_iso.as_deref() == Some(&date_iso);
                    let is_range_end = is_range_mode && range_end_iso.as_deref() == Some(&date_iso);
                    let is_in_range = is_range_mode
                        && match (range_start_iso.as_deref(), range_end_iso.as_deref()) {
                            (Some(s), Some(e)) => date_iso.as_str() > s && date_iso.as_str() < e,
                            _ => false,
                        };
                    let is_range_edge = is_range_start || is_range_end;

                    let cell_id = SharedString::from(format!("poodle-cal-day-{}", day_num));
                    let mut cell = div()
                        .id(cell_id)
                        .focusable()
                        .w(cell_size)
                        .h(cell_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(control_radius)
                        .text_size(body_size);

                    if is_selected || is_range_edge {
                        // Solid accent pill for selected / range edge days.
                        cell = cell
                            .bg(accent)
                            .text_color(text_inverse)
                            .font_weight(FontWeight::SEMIBOLD);
                        // Svelte: selected / range-endpoint hover lightens the
                        // accent (color-mix accent 88%, white 8%).
                        if !spec.is_disabled {
                            cell = cell.hover(move |s| s.bg(selected_hover_bg));
                        }
                    } else if is_in_range {
                        // Svelte: in-range bg = color-mix(accent 16%, transparent)
                        let in_range_bg = Hsla { a: accent.a * 0.16, ..accent };
                        cell = cell.bg(in_range_bg).text_color(text_primary);
                        if is_today {
                            cell = cell
                                .border_1()
                                .border_color(today_border)
                                .font_weight(FontWeight::SEMIBOLD);
                        }
                    } else {
                        cell = cell.text_color(text_primary);
                        // Contract: today = border ring around cell
                        if is_today {
                            cell = cell
                                .border_1()
                                .border_color(today_border)
                                .font_weight(FontWeight::SEMIBOLD);
                        }
                        // Svelte: hover border = color-mix(accent 46%, border-default)
                        //         hover bg    = color-mix(accent 14%, transparent)
                        cell = cell.hover(move |s| s.border_color(hover_border).bg(hover_bg));
                    }

                    if spec.is_disabled {
                        cell = cell.cursor(CursorStyle::OperationNotAllowed);
                    } else {
                        cell = cell.cursor_pointer();
                    }

                    // Wire click handler — range mode computes a new
                    // DateRangeValue and fires on_range_select; single
                    // mode fires on_select with the ISO date.
                    if !spec.is_disabled {
                        if is_range_mode {
                            if let Some(ref handler) = self.on_range_select {
                                let handler = handler.clone();
                                let current_start = range_start_iso.clone();
                                let current_end = range_end_iso.clone();
                                let date_clicked = date_iso.clone();
                                cell = cell.on_click(move |_event, window, cx| {
                                    let next = compute_next_range(
                                        current_start.as_deref(),
                                        current_end.as_deref(),
                                        &date_clicked,
                                    );
                                    handler(&next, window, cx);
                                });
                            }
                        } else if let Some(ref handler) = self.on_select {
                            let handler = handler.clone();
                            let date_str = date_iso.clone();
                            cell = cell.on_click(move |_event, window, cx| {
                                handler(&date_str, window, cx);
                            });
                        }
                    }

                    cell = cell.child(format!("{}", day_num));
                    day_row = day_row.child(cell);
                }
            }
            cal = cal.child(day_row);
        }

        cal.into_any_element()
    }
}

fn compute_next_range(
    current_start: Option<&str>,
    current_end: Option<&str>,
    clicked: &str,
) -> DateRangeValue {
    match (current_start, current_end) {
        (None, _) => DateRangeValue::new(Some(clicked.to_string()), None),
        (Some(start), None) => {
            if clicked >= start {
                DateRangeValue::new(Some(start.to_string()), Some(clicked.to_string()))
            } else {
                DateRangeValue::new(Some(clicked.to_string()), Some(start.to_string()))
            }
        }
        (Some(_), Some(_)) => DateRangeValue::new(Some(clicked.to_string()), None),
    }
}

