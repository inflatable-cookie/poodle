//! DateTimePicker — real GPUI component backed by DateTimePickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CalendarWeekStart, ControlDensity, ControlSize, DateTimePickerSpec, DateTimeValue, IconSize,
    IconSpec, SemanticControlSizeRole,
};

use super::calendar::Calendar;
use super::icon::Icon;
use super::time_field::TimeField;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

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
    fn deref(&self) -> &DateTimePickerSpec {
        &self.spec
    }
}

impl DateTimePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DateTimePickerSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
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
    pub fn value(mut self, v: DateTimeValue) -> Self {
        self.spec.value = Some(v);
        self
    }
    pub fn default_value(mut self, v: DateTimeValue) -> Self {
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
}

impl IntoElement for DateTimePicker {
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
        let has_value = value.date.is_some() || value.time.is_some();

        // Contract §4: partial values surface the prompt for the missing part.
        let display_text = match (value.date.as_deref(), value.time.as_deref()) {
            (Some(d), Some(t)) => format!("{} {}", d, t),
            (Some(d), None) => format!("{} {}", d, "Select time"),
            (None, Some(t)) => format!("{} {}", "Select date", t),
            (None, None) => spec.placeholder.clone(),
        };

        let is_open = spec.open.unwrap_or(spec.default_open);
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-datetime-picker-{}", suffix)
        } else {
            "poodle-datetime-picker".to_string()
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
            .text_size(body_size);

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

        let mut container = div().flex().flex_col().gap(inline_gap).child(trigger);

        if is_open {
            // Contract §8 surface padding: space.panel-y / space.panel-x
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

            // Composed Calendar (single mode), seeded from the picker's date.
            let mut cal_spec = poodle_specs::CalendarSpec::new()
                .with_week_start(spec.week_starts_on.clone());
            if let Some(ref date) = value.date {
                cal_spec = cal_spec.with_value(date.clone());
                cal_spec = cal_spec.with_visible_month(date.clone());
            }
            cal_spec.is_disabled = is_disabled;
            let calendar = Calendar::from_spec(cal_spec, theme);

            // Time Label — contract: label-family, 0.6875rem, weight 600,
            // 0.04em tracking, uppercase.
            let time_label = div()
                .text_size(time_label_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_secondary)
                .child("TIME");

            // Composed TimeInput (TimeField), seeded from the picker's time.
            let mut time_spec = poodle_specs::TimeFieldSpec::new();
            time_spec.value = value.time.clone();
            time_spec.is_disabled = is_disabled;
            let time_input = TimeField::from_spec(time_spec, theme);

            // Time Section — label above composed TimeInput.
            let time_section = div()
                .flex()
                .flex_col()
                .gap(time_section_gap)
                .child(time_label)
                .child(time_input);

            // Body — vertical stack of Calendar + Time Section.
            let body = div()
                .flex()
                .flex_col()
                .gap(body_gap)
                .child(calendar)
                .child(time_section);

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
