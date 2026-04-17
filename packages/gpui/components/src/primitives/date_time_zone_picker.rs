//! DateTimeZonePicker — real GPUI component backed by DateTimeZonePickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, DateTimeZonePickerSpec, IconSize, IconSpec,
    SemanticControlSizeRole,
};

use super::icon::Icon;
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
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let panel_bg = resolve_color(theme, "color.background.panel");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let label_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let gap_md = resolve_px(theme, "space.inline.md");

        let display_value = spec.value.as_deref().unwrap_or("Select date & time...");
        let tz_display = spec.time_zone.as_deref().unwrap_or("");
        let is_placeholder = spec.value.is_none();
        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
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
            .border_color(border)
            .flex()
            .items_center()
            .gap(inline_gap)
            .text_size(body_size)
            .child(
                div()
                    .flex_1()
                    .text_color(text_col)
                    .child(display_value.to_string()),
            );

        if !tz_display.is_empty() {
            trigger = trigger.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(tz_display.to_string()),
            );
        }

        trigger = trigger.child(
            Icon::from_spec(IconSpec::new("calendar").with_size(IconSize::Sm), theme)
                .with_color(icon_muted),
        );

        trigger = trigger.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if spec.is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            trigger = trigger.cursor_pointer();
        }

        let is_open = spec.is_open;
        let is_disabled = spec.is_disabled;

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

        if spec.is_open {
            let time_display = spec.value.as_deref().unwrap_or("--:--");

            let tz_overlay_display = spec.time_zone.as_deref().unwrap_or("Select timezone...");
            let tz_has_value = spec.time_zone.is_some();

            let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

            let mut weekday_row = div().flex().gap(inline_gap);
            for day in &weekdays {
                weekday_row = weekday_row.child(
                    div()
                        .flex_1()
                        .text_size(caption_size)
                        .text_color(text_secondary)
                        .child(day.to_string()),
                );
            }

            // Calendar section
            let calendar_section = div()
                .flex()
                .flex_col()
                .gap(gap_md)
                .child(
                    div()
                        .text_size(body_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child("Select date"),
                )
                .child(weekday_row)
                .child(div().min_h(px(rem_to_px(11.25)))); // 11.25rem placeholder height

            // Time section
            let time_section = div()
                .flex()
                .flex_col()
                .gap(inline_gap)
                .py(gap_md)
                .child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child("Time"),
                )
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(text_primary)
                        .child(time_display.to_string()),
                );

            // Timezone section
            let timezone_section = div()
                .flex()
                .flex_col()
                .gap(inline_gap)
                .py(gap_md)
                .child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child("Time zone"),
                )
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(if tz_has_value {
                            text_primary
                        } else {
                            text_secondary
                        })
                        .child(tz_overlay_display.to_string()),
                );

            let overlay = div()
                .rounded(control_radius)
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
                .p(resolve_px(theme, "space.inline.lg"))
                .flex()
                .flex_col()
                .child(calendar_section)
                .child(div().border_b_1().border_color(border))
                .child(time_section)
                .child(div().border_b_1().border_color(border))
                .child(timezone_section);

            wrapper = wrapper.child(overlay);
        }

        wrapper.into_any_element()
    }
}
