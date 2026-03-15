//! PugDateTimeRangePicker — real GPUI component backed by DateTimeRangePickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::DateTimeRangePickerSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI date-time range picker backed by `DateTimeRangePickerSpec`.
///
/// Shows a trigger button with the start and end date-time values.
pub struct PugDateTimeRangePicker {
    spec: DateTimeRangePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl PugDateTimeRangePicker {
    pub fn new(spec: DateTimeRangePickerSpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for PugDateTimeRangePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

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

        let text_col = if has_value {
            text_primary
        } else {
            text_secondary
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

        trigger.into_any_element()
    }
}
