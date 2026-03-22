//! ZonedDateTimePicker — real GPUI component backed by ZonedDateTimePickerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, ZonedDateTimePickerSpec};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI zoned date-time picker component backed by `ZonedDateTimePickerSpec`.
pub struct ZonedDateTimePicker {
    spec: ZonedDateTimePickerSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ZonedDateTimePicker {
    type Target = ZonedDateTimePickerSpec;
    fn deref(&self) -> &ZonedDateTimePickerSpec { &self.spec }
}

impl ZonedDateTimePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ZonedDateTimePickerSpec::new(), theme: theme.clone(), on_toggle: None }
    }

    pub fn from_spec(spec: ZonedDateTimePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn time_zone(mut self, v: impl Into<String>) -> Self { self.spec.time_zone = Some(v.into()); self }
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ZonedDateTimePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let display_value = spec
            .value
            .as_deref()
            .unwrap_or("Select date & time...");
        let tz_display = spec
            .time_zone
            .as_deref()
            .unwrap_or("");
        let is_placeholder = spec.value.is_none();
        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        let mut trigger = div()
            .id(SharedString::from("pug-zoned-dt-picker"))
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
            .text_size(px(14.0))
            .child(div().flex_1().text_color(text_col).child(display_value.to_string()));

        if !tz_display.is_empty() {
            trigger = trigger.child(
                div()
                    .text_size(px(12.0))
                    .text_color(text_secondary)
                    .child(tz_display.to_string()),
            );
        }

        trigger = trigger.child(
            Icon::from_spec(
                IconSpec::new("calendar").with_size(IconSize::Sm),
                theme,
            )
            .with_color(icon_muted),
        );

        trigger = trigger.focus(move |s| s.border_color(focus_ring));

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

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        if spec.is_open {
            let time_display = spec
                .value
                .as_deref()
                .unwrap_or("--:--");

            let tz_overlay_display = spec
                .time_zone
                .as_deref()
                .unwrap_or("Select timezone...");
            let tz_has_value = spec.time_zone.is_some();

            let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

            let mut weekday_row = div().flex().gap(px(4.0));
            for day in &weekdays {
                weekday_row = weekday_row.child(
                    div()
                        .flex_1()
                        .text_size(px(11.0))
                        .text_color(text_secondary)
                        .child(day.to_string()),
                );
            }

            // Calendar section
            let calendar_section = div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child("Select date"),
                )
                .child(weekday_row)
                .child(div().min_h(px(180.0)));

            // Time section
            let time_section = div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .py(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child("Time"),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(text_primary)
                        .child(time_display.to_string()),
                );

            // Timezone section
            let timezone_section = div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .py(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child("Time zone"),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(if tz_has_value { text_primary } else { text_secondary })
                        .child(tz_overlay_display.to_string()),
                );

            let overlay = div()
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .p(px(16.0))
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
