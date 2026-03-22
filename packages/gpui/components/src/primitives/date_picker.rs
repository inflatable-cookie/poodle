//! DatePicker — real GPUI component backed by DatePickerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CalendarWeekStart, DatePickerSpec, IconSize, IconSpec};

use super::calendar::Calendar;
use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI date picker component backed by `DatePickerSpec`.
///
/// Renders a trigger button showing the current date value or placeholder.
/// When open, shows a `Calendar` below the trigger.
pub struct DatePicker {
    spec: DatePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DatePicker {
    type Target = DatePickerSpec;
    fn deref(&self) -> &DatePickerSpec { &self.spec }
}

impl DatePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DatePickerSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None, on_select: None }
    }

    pub fn from_spec(spec: DatePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
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

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for DatePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        // Contract: hover = color-mix(surface 84%, elevated)
        let hover_bg = color_mix(surface_bg, elevated_bg, 0.84);

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
            .text_size(px(14.0));

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

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        trigger = trigger
            .child(div().text_color(text_col).flex_1().child(display_text))
            .child(
                // Contract: calendar icon indicator (not chevron)
                Icon::from_spec(
                    IconSpec::new("calendar").with_size(IconSize::Sm),
                    theme,
                )
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

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        // Calendar dropdown when open
        if is_open {
            let mut cal_spec = pug_primitives::CalendarSpec::new()
                .with_week_start(spec.week_starts_on.clone());

            if let Some(val) = spec.current_value() {
                cal_spec = cal_spec.with_value(val);
                cal_spec = cal_spec.with_visible_month(val);
            }

            let panel_bg = resolve_color(theme, "semantic.color.background.panel");
            let surface_raw = resolve_color(theme, "semantic.color.background.elevated");
            let panel_border = resolve_color(theme, "semantic.color.border.default");
            // Calendar surface container
            let cal_surface = div()
                .rounded(resolve_radius(theme, "semantic.radius.surface"))
                .bg(color_mix(surface_raw, panel_bg, 0.98))
                .border_1()
                .border_color(color_mix(panel_border, panel_bg, 0.72))
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
                .p(px(12.0))
                .child(Calendar::from_spec(cal_spec, theme));
            wrapper = wrapper.child(cal_surface);
        }

        wrapper.into_any_element()
    }
}
