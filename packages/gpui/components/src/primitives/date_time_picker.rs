//! DateTimePicker — real GPUI component backed by DateTimePickerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CalendarWeekStart, DateTimePickerSpec, DateTimeValue};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI date-time picker component backed by `DateTimePickerSpec`.
///
/// Combines date and time display in a trigger button. Shows both values
/// from `current_value()` or falls back to the placeholder.
pub struct DateTimePicker {
    spec: DateTimePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DateTimePicker {
    type Target = DateTimePickerSpec;
    fn deref(&self) -> &DateTimePickerSpec { &self.spec }
}

impl DateTimePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DateTimePickerSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None }
    }

    pub fn from_spec(spec: DateTimePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: DateTimeValue) -> Self { self.spec.value = Some(v); self }
    pub fn default_value(mut self, v: DateTimeValue) -> Self { self.spec.default_value = v; self }
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
}

impl IntoElement for DateTimePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");

        let value = spec.current_value();
        let has_value = value.date.is_some() || value.time.is_some();

        let display_text = if has_value {
            let date_part = value.date.as_deref().unwrap_or("—");
            let time_part = value.time.as_deref().unwrap_or("—");
            format!("{} {}", date_part, time_part)
        } else {
            spec.placeholder.clone()
        };

        let is_open = spec.open.unwrap_or(spec.default_open);
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-datetime-picker-{}", suffix)
        } else {
            "pug-datetime-picker".to_string()
        };

        let mut trigger = div()
            .id(SharedString::from(id_str))
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(elevated_bg)
            .border_1()
            .border_color(if is_open { accent } else { border })
            .flex()
            .items_center()
            .justify_between()
            .gap(inline_gap)
            .text_sm();

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
