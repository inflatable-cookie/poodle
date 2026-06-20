//! DateTimeZonePicker — real GPUI component backed by DateTimeZonePickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, DateTimeZonePickerSpec, IconSize, IconSpec,
    SemanticControlSizeRole,
};

use super::calendar::Calendar;
use super::icon::Icon;
use super::time_field::TimeField;
use super::time_zone_select::TimeZoneSelect;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI date-time-zone picker component backed by `DateTimeZonePickerSpec`.
pub struct DateTimeZonePicker {
    spec: DateTimeZonePickerSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DateTimeZonePicker {
    type Target = DateTimeZonePickerSpec;
    fn deref(&self) -> &DateTimeZonePickerSpec {
        &self.spec
    }
}

impl DateTimeZonePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DateTimeZonePickerSpec::new(),
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    pub fn from_spec(spec: DateTimeZonePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn time_zone(mut self, v: impl Into<String>) -> Self {
        self.spec.time_zone = Some(v.into());
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
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

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for DateTimeZonePicker {
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

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let panel_bg = resolve_color(theme, "color.background.panel");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let accent = resolve_color(theme, "color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let hover_bg = color_mix(surface_bg, elevated_bg, 0.86);

        let is_open = spec.is_open;
        let is_disabled = spec.is_disabled;

        // Trigger value: contract anatomy is Value + Indicator only, so the
        // timezone is folded into the formatted value string rather than shown
        // as a separate inline segment.
        let has_value = spec.value.is_some() || spec.time_zone.is_some();
        let display_value = match (spec.value.as_deref(), spec.time_zone.as_deref()) {
            (Some(v), Some(tz)) => format!("{} {}", v, tz),
            (Some(v), None) => v.to_string(),
            (None, Some(tz)) => tz.to_string(),
            (None, None) => "Select date, time, and zone".to_string(),
        };
        let text_col = if has_value {
            text_primary
        } else {
            text_secondary
        };

        let focus_ring = resolve_color(theme, "color.accent.focusRing");

        let mut trigger = div()
            .id(SharedString::from("poodle-dt-zone-picker"))
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
            .child(div().flex_1().text_color(text_col).child(display_value))
            .child(
                Icon::from_spec(IconSpec::new("calendar").with_size(IconSize::Sm), theme)
                    .with_color(icon_muted),
            );

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

        let mut wrapper = div().flex().flex_col().gap(inline_gap).child(trigger);

        if is_open {
            // Contract §8 surface padding: space.panel-y / space.panel-x.
            let surface_pad_x = resolve_px(theme, "space.panel.x");
            let surface_pad_y = resolve_px(theme, "space.panel.y");
            // Contract §8 gaps: Body (0.875rem), Fields (0.75rem), Field (0.375rem).
            // Absolute-rem contract values resolved via rem_to_px (same pattern the
            // sibling pickers use); no dedicated semantic tokens exist for these.
            let body_gap = px(rem_to_px(0.875));
            let fields_gap = px(rem_to_px(0.75));
            let field_gap = px(rem_to_px(0.375));
            // Contract §8 Field Label typography.
            let field_label_size = px(rem_to_px(0.6875));

            // Field Label — contract: label-family, 0.6875rem, weight 600,
            // 0.04em tracking, uppercase.
            let field_label = |text: &str| {
                div()
                    .text_size(field_label_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_secondary)
                    .child(text.to_string())
            };

            // Composed Calendar (single mode), seeded from the picker's date value.
            let mut cal_spec = poodle_specs::CalendarSpec::new();
            if let Some(ref date) = spec.value {
                cal_spec = cal_spec.with_value(date.clone());
                cal_spec = cal_spec.with_visible_month(date.clone());
            }
            cal_spec.is_disabled = is_disabled;
            let calendar = Calendar::from_spec(cal_spec, theme);

            // Composed TimeInput (TimeField), seeded from the picker's time value.
            let mut time_spec = poodle_specs::TimeFieldSpec::new();
            time_spec.value = spec.value.clone();
            time_spec.is_disabled = is_disabled;
            let time_field = TimeField::from_spec(time_spec, theme);

            // Time field — contract Field: label "Time" above composed TimeInput.
            let time_field_group = div()
                .flex()
                .flex_col()
                .gap(field_gap)
                .child(field_label("Time"))
                .child(time_field);

            // Composed TimeZoneSelect, seeded from the picker's timezone value.
            let mut tz_spec = poodle_specs::TimeZoneSelectSpec::new();
            tz_spec.value = spec.time_zone.clone();
            tz_spec.is_disabled = is_disabled;
            let tz_select = TimeZoneSelect::from_spec(tz_spec, theme);

            // Time zone field — contract Field: label "Time zone" above composed
            // TimeZoneSelect.
            let tz_field_group = div()
                .flex()
                .flex_col()
                .gap(field_gap)
                .child(field_label("Time zone"))
                .child(tz_select);

            // Fields — vertical stack of Time + Time zone fields.
            let fields = div()
                .flex()
                .flex_col()
                .gap(fields_gap)
                .child(time_field_group)
                .child(tz_field_group);

            // Body — vertical stack of Calendar + Fields.
            let body = div()
                .flex()
                .flex_col()
                .gap(body_gap)
                .child(calendar)
                .child(fields);

            // Surface — established sibling overlay treatment (date_time_picker.rs /
            // date_time_range_picker.rs): elevated 98% over panel, border at 72%
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
                .shadow(crate::theme_ext::elevation_overlay_shadow())
                .child(body);

            wrapper = wrapper.child(overlay);
        }

        wrapper.into_any_element()
    }
}
