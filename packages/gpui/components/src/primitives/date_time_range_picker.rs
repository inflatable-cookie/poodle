//! DateTimeRangePicker — real GPUI component backed by DateTimeRangePickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CalendarWeekStart, ControlDensity, ControlSize, DateTimeRangePickerSpec, DateTimeRangeValue,
    IconSize, IconSpec, SemanticControlSizeRole,
};

use super::calendar::Calendar;
use super::icon::Icon;
use super::time_field::TimeField;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
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
    fn deref(&self) -> &DateTimeRangePickerSpec {
        &self.spec
    }
}

impl DateTimeRangePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DateTimeRangePickerSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
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
    pub fn value(mut self, v: DateTimeRangeValue) -> Self {
        self.spec.value = Some(v);
        self
    }
    pub fn default_value(mut self, v: DateTimeRangeValue) -> Self {
        self.spec.default_value = v;
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.spec.default_open = v;
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = v.into();
        self
    }
    pub fn week_starts_on(mut self, v: CalendarWeekStart) -> Self {
        self.spec.week_starts_on = v;
        self
    }
    pub fn locale(mut self, v: impl Into<String>) -> Self {
        self.spec.locale = v.into();
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    /// Format a DateTimeValue into a display string.
    fn format_datetime(date: Option<&str>, time: Option<&str>) -> Option<String> {
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
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let base_height = resolve_px(theme, "size.control.height");
        let control_height = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
        let base_pad = resolve_px(theme, "space.inline.md");
        let inline_padding = base_pad + px(rem_to_px(size_padding_x_offset_rem(effective_size)));
        let inline_gap = resolve_px(theme, "space.inline.sm");
        let control_radius = resolve_radius(theme, "radius.control");
        let trigger_min_w = resolve_px(theme, "size.dateTimeRangePicker.minWidth");
        let gap_inline_xs = resolve_px(theme, "space.inline.xs");

        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let panel_bg = resolve_color(theme, "color.background.panel");
        let border = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let accent = resolve_color(theme, "color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let hover_bg = color_mix(surface_bg, elevated_bg, 0.86);

        let value = spec.current_value();
        let start_text =
            Self::format_datetime(value.start.date.as_deref(), value.start.time.as_deref());
        let end_text = Self::format_datetime(value.end.date.as_deref(), value.end.time.as_deref());

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
            format!("poodle-datetime-range-picker-{}", suffix)
        } else {
            "poodle-datetime-range-picker".to_string()
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
            .text_size(body_size)
            .min_w(trigger_min_w);

        // Focus ring
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        trigger = trigger.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            trigger = trigger.cursor_pointer().hover(|s| s.bg(hover_bg));
        }

        let text_col = if has_value {
            text_primary
        } else {
            text_secondary
        };

        trigger = trigger
            .child(div().text_color(text_col).flex_1().child(display_text))
            .child(
                Icon::from_spec(IconSpec::new("calendar").with_size(IconSize::Sm), theme)
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
            .gap(gap_inline_xs)
            .child(trigger);

        if is_open {
            // Contract §8 surface padding: space.panel-y / space.panel-x.
            let surface_pad_x = resolve_px(theme, "space.panel.x");
            let surface_pad_y = resolve_px(theme, "space.panel.y");
            // Contract §8 Body gap (0.875rem) and Time Section gap (0.375rem).
            // Absolute-rem contract values resolved via rem_to_px (same pattern
            // the Calendar primitive uses for its absolute cell sizes); no
            // dedicated semantic tokens exist for these two gaps.
            let body_gap = px(rem_to_px(0.875));
            let time_section_gap = px(rem_to_px(0.375));
            // Contract §8 Time Label typography.
            let time_label_size = px(rem_to_px(0.6875));

            // Composed Calendar in range mode, seeded from the start/end dates.
            let mut cal_spec = poodle_specs::CalendarSpec::new()
                .with_mode(poodle_specs::CalendarMode::Range)
                .with_week_start(spec.week_starts_on.clone());
            cal_spec.range_value = Some(poodle_specs::DateRangeValue::new(
                value.start.date.clone(),
                value.end.date.clone(),
            ));
            if let Some(ref start_date) = value.start.date {
                cal_spec.visible_month = Some(start_date.clone());
            }
            cal_spec.is_disabled = is_disabled;
            let calendar = Calendar::from_spec(cal_spec, theme);

            // A composed Time Section: contract Time Label + real TimeInput.
            let time_section = |label: &str, time_val: Option<String>| {
                let mut time_spec = poodle_specs::TimeFieldSpec::new();
                time_spec.value = time_val;
                time_spec.is_disabled = is_disabled;
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(time_section_gap)
                    .child(
                        // Contract: label-family, 0.6875rem, weight 600,
                        // 0.04em tracking, uppercase.
                        div()
                            .text_size(time_label_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_secondary)
                            .child(label.to_string()),
                    )
                    .child(TimeField::from_spec(time_spec, theme))
            };

            // Paired start/end time inputs.
            let time_row = div()
                .flex()
                .items_start()
                .gap(inline_gap)
                .child(time_section("START TIME", value.start.time.clone()))
                .child(time_section("END TIME", value.end.time.clone()));

            // Body — vertical stack of range Calendar + paired Time Sections.
            let body = div()
                .flex()
                .flex_col()
                .gap(body_gap)
                .child(calendar)
                .child(time_row);

            // Surface — established sibling overlay treatment (date_picker.rs /
            // date_range_picker.rs): elevated 98% over panel, border at 72%
            // alpha, two-layer overlay shadow.
            let overlay = div()
                .px(surface_pad_x)
                .py(surface_pad_y)
                .rounded(resolve_radius(theme, "radius.surface"))
                // Svelte: color-mix(elevated 98%, panel)
                .bg(color_mix(elevated_bg, panel_bg, 0.98))
                .border_1()
                // Svelte: color-mix(border-default 72%, transparent)
                .border_color(Hsla { a: border.a * 0.72, ..border })
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
                .child(body);

            container = container.child(overlay);
        }

        container.into_any_element()
    }
}
