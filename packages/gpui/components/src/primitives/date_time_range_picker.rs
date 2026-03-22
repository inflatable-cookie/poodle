//! DateTimeRangePicker — real GPUI component backed by DateTimeRangePickerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CalendarWeekStart, DateTimeRangePickerSpec, DateTimeRangeValue, IconSize, IconSpec};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI date-time range picker backed by `DateTimeRangePickerSpec`.
///
/// Shows a trigger button with the start and end date-time values.
pub struct DateTimeRangePicker {
    spec: DateTimeRangePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DateTimeRangePicker {
    type Target = DateTimeRangePickerSpec;
    fn deref(&self) -> &DateTimeRangePickerSpec { &self.spec }
}

impl DateTimeRangePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DateTimeRangePickerSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None }
    }

    pub fn from_spec(spec: DateTimeRangePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: DateTimeRangeValue) -> Self { self.spec.value = Some(v); self }
    pub fn default_value(mut self, v: DateTimeRangeValue) -> Self { self.spec.default_value = v; self }
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = v.into(); self }
    pub fn week_starts_on(mut self, v: CalendarWeekStart) -> Self { self.spec.week_starts_on = v; self }
    pub fn locale(mut self, v: impl Into<String>) -> Self { self.spec.locale = v.into(); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    /// Format a DateTimeValue into a display string.
    fn format_datetime(
        date: Option<&str>,
        time: Option<&str>,
    ) -> Option<String> {
        match (date, time) {
            (Some(d), Some(t)) => Some(format!("{} {}", d, t)),
            (Some(d), None) => Some(d.to_string()),
            (None, Some(t)) => Some(t.to_string()),
            (None, None) => None,
        }
    }
}

impl IntoElement for DateTimeRangePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let hover_bg = color_mix(surface_bg, elevated_bg, 0.84);

        let value = spec.current_value();
        let start_text = Self::format_datetime(
            value.start.date.as_deref(),
            value.start.time.as_deref(),
        );
        let end_text = Self::format_datetime(
            value.end.date.as_deref(),
            value.end.time.as_deref(),
        );

        let has_value = start_text.is_some() || end_text.is_some();
        let display_text = if has_value {
            let s = start_text.as_deref().unwrap_or("…");
            let e = end_text.as_deref().unwrap_or("…");
            format!("{} – {}", s, e)
        } else {
            spec.placeholder.clone()
        };

        let is_open = spec.open.unwrap_or(spec.default_open);
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-datetime-range-picker-{}", suffix)
        } else {
            "pug-datetime-range-picker".to_string()
        };

        let mut trigger = div()
            .id(SharedString::from(id_str))
            .focusable()
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(if is_open { accent } else { border })
            .flex()
            .items_center()
            .justify_between()
            .gap(inline_gap)
            .text_size(px(14.0))
            .min_w(px(288.0)); // Contract: min-width 18rem

        // Focus ring
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        trigger = trigger.focus(move |s| s.border_color(focus_ring));

        if is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(|s| s.bg(hover_bg));
        }

        let text_col = if has_value {
            text_primary
        } else {
            text_secondary
        };

        trigger = trigger
            .child(div().text_color(text_col).flex_1().child(display_text))
            .child(
                Icon::from_spec(
                    IconSpec::new("calendar").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_muted),
            );

        if let Some(handler) = self.on_toggle {
            if !is_disabled {
                let next_open = !is_open;
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                trigger = trigger
                    .on_click(move |_event, window, cx| {
                        handler(&next_open, window, cx);
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&next_open, window, cx);
                        } else if event.keystroke.key == "escape" && is_open {
                            key_handler(&false, window, cx);
                        }
                    });
            }
        }

        let mut container = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(trigger);

        if is_open {
            let section_padding = resolve_px(theme, "semantic.space.stack.md");
            let inner_gap = resolve_px(theme, "semantic.space.stack.sm");

            let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

            // Weekday header row
            let mut weekday_row = div()
                .flex()
                .items_center()
                .gap(px(2.0));
            for day in &weekdays {
                weekday_row = weekday_row.child(
                    div()
                        .flex_1()
                        .flex()
                        .justify_center()
                        .text_size(px(11.0))
                        .text_color(text_secondary)
                        .child(*day),
                );
            }

            // Placeholder grid rows (6 rows x 7 cols)
            let mut grid = div().flex().flex_col().gap(px(2.0));
            for _row in 0..6 {
                let mut row = div().flex().items_center().gap(px(2.0));
                for _col in 0..7 {
                    row = row.child(
                        div()
                            .flex_1()
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(px(12.0))
                            .text_color(text_secondary)
                            .child("—"),
                    );
                }
                grid = grid.child(row);
            }

            // RangeCalendar section
            let calendar_section = div()
                .p(section_padding)
                .flex()
                .flex_col()
                .gap(inner_gap)
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child("Select date range"),
                )
                .child(weekday_row)
                .child(grid);

            // Separator
            let separator = div()
                .w_full()
                .h(px(1.0))
                .bg(border);

            // Time fields row — two TimeField sections side by side
            let start_time_display = value.start.time.as_deref().unwrap_or("--:--");
            let end_time_display = value.end.time.as_deref().unwrap_or("--:--");

            let time_field = |label: &str, time_val: &str| {
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(text_secondary)
                            .child(label.to_string()),
                    )
                    .child(
                        div()
                            .px(inline_padding)
                            .h(px(28.0))
                            .rounded(control_radius)
                            .bg(surface_bg)
                            .border_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .text_size(px(13.0))
                            .text_color(text_primary)
                            .child(time_val.to_string()),
                    )
            };

            let time_section = div()
                .p(section_padding)
                .flex()
                .items_center()
                .gap(inline_gap)
                .child(time_field("Start time", start_time_display))
                .child(time_field("End time", end_time_display));

            // Bottom action bar
            let action_bar = div()
                .p(section_padding)
                .border_t_1()
                .border_color(border)
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(accent)
                        .cursor_pointer()
                        .child("Today"),
                )
                .child(
                    div()
                        .px(inline_padding)
                        .h(px(28.0))
                        .rounded(control_radius)
                        .bg(accent)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .text_color(elevated_bg)
                        .cursor_pointer()
                        .child("Done"),
                );

            let overlay = div()
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.10),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(16.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.06),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(4.0),
                        spread_radius: px(0.0),
                    },
                ])
                .overflow_hidden()
                .child(calendar_section)
                .child(separator)
                .child(time_section)
                .child(action_bar);

            container = container.child(overlay);
        }

        container.into_any_element()
    }
}
