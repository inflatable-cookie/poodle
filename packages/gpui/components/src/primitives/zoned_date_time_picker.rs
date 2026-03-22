//! ZonedDateTimePicker — real GPUI component backed by ZonedDateTimePickerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::ZonedDateTimePickerSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI zoned date-time picker component backed by `ZonedDateTimePickerSpec`.
pub struct ZonedDateTimePicker {
    spec: ZonedDateTimePickerSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for ZonedDateTimePicker {
    type Target = ZonedDateTimePickerSpec;
    fn deref(&self) -> &ZonedDateTimePickerSpec { &self.spec }
}

impl ZonedDateTimePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ZonedDateTimePickerSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: ZonedDateTimePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn time_zone(mut self, v: impl Into<String>) -> Self { self.spec.time_zone = Some(v.into()); self }
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }

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
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap(inline_gap)
            .text_sm()
            .child(div().flex_1().text_color(text_col).child(display_value.to_string()));

        if !tz_display.is_empty() {
            trigger = trigger.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(tz_display.to_string()),
            );
        }

        trigger = trigger.child(
            div()
                .text_xs()
                .text_color(text_secondary)
                .child(if spec.is_open { "\u{25b4}" } else { "\u{25be}" }),
        );

        trigger = trigger.focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            trigger = trigger.cursor_pointer();
        }

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        if spec.is_open {
            let overlay = div()
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .p(px(16.0))
                .text_sm()
                .text_color(text_primary)
                .child("Calendar + time picker + timezone selector");

            wrapper = wrapper.child(overlay);
        }

        wrapper.into_any_element()
    }
}
