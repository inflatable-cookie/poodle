//! PugDateRangePicker — real GPUI component backed by DateRangePickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::DateRangePickerSpec;

use crate::range_calendar::PugRangeCalendar;
use crate::theme_ext::resolve_color;

/// A real GPUI date range picker backed by `DateRangePickerSpec`.
///
/// Shows a trigger button with start–end date text. When open, displays
/// a `PugRangeCalendar` below.
pub struct PugDateRangePicker {
    spec: DateRangePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl PugDateRangePicker {
    pub fn new(spec: DateRangePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

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
}

impl IntoElement for PugDateRangePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let range = spec.current_value();
        let display_text = match (&range.start, &range.end) {
            (Some(start), Some(end)) => format!("{} – {}", start, end),
            (Some(start), None) => format!("{} – …", start),
            (None, Some(end)) => format!("… – {}", end),
            (None, None) => spec.placeholder.clone(),
        };
        let is_placeholder = range.start.is_none() && range.end.is_none();
        let is_open = spec.current_open();
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-date-range-picker-{}", suffix)
        } else {
            "pug-date-range-picker".to_string()
        };

        // Trigger button
        let mut trigger = div()
            .id(SharedString::from(id_str))
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(elevated_bg)
            .border_1()
            .border_color(if is_open { accent } else { border })
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .text_sm();

        if is_disabled {
            trigger = trigger.opacity(0.48);
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
        }

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        trigger = trigger
            .child(div().text_color(text_col).child(display_text))
            .child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child("📅"),
            );

        if let Some(handler) = self.on_toggle {
            if !is_disabled {
                let next_open = !is_open;
                trigger = trigger.on_click(move |_event, window, cx| {
                    handler(&next_open, window, cx);
                });
            }
        }

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        // Range calendar dropdown when open
        if is_open {
            let mut cal_spec = pug_gpui_primitives::RangeCalendarSpec::new();
            cal_spec.week_starts_on = spec.week_starts_on.clone();
            cal_spec.value = Some(range.clone());

            if let Some(ref start) = range.start {
                cal_spec.visible_month = Some(start.clone());
            }

            let calendar = PugRangeCalendar::new(cal_spec, theme);
            wrapper = wrapper.child(calendar);
        }

        wrapper.into_any_element()
    }
}
