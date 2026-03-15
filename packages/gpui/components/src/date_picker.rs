//! PugDatePicker — real GPUI component backed by DatePickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::DatePickerSpec;

use crate::calendar::PugCalendar;
use crate::theme_ext::resolve_color;

/// A real GPUI date picker component backed by `DatePickerSpec`.
///
/// Renders a trigger button showing the current date value or placeholder.
/// When open, shows a `PugCalendar` below the trigger.
pub struct PugDatePicker {
    spec: DatePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugDatePicker {
    pub fn new(spec: DatePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_select: None,
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

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugDatePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let display_text = spec
            .current_value()
            .unwrap_or(&spec.placeholder)
            .to_string();
        let is_placeholder = spec.current_value().is_none();
        let is_open = spec.current_open();
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-date-picker-{}", suffix)
        } else {
            "pug-date-picker".to_string()
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

        // Calendar dropdown when open
        if is_open {
            let mut cal_spec = pug_gpui_primitives::CalendarSpec::new()
                .with_week_start(spec.week_starts_on.clone());

            if let Some(val) = spec.current_value() {
                cal_spec = cal_spec.with_value(val);
                cal_spec = cal_spec.with_visible_month(val);
            }

            let calendar = PugCalendar::new(cal_spec, theme);
            wrapper = wrapper.child(calendar);
        }

        wrapper.into_any_element()
    }
}
