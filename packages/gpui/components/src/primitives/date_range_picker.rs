//! DateRangePicker — real GPUI component backed by DateRangePickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CalendarWeekStart, ControlDensity, ControlSize, DateRangePickerSpec, DateRangeValue,
    SemanticControlSizeRole,
};

use super::calendar::Calendar;
use super::icon::Icon;
use crate::presentation::{
    date_picker_indicator_font_rem, rem_to_px, resolve_semantic_size, size_font_rem,
    size_height_offset_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI date range picker backed by `DateRangePickerSpec`.
///
/// Shows a trigger button with start-end date text. When open, displays
/// a `Calendar` in range mode below.
pub struct DateRangePicker {
    spec: DateRangePickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DateRangePicker {
    type Target = DateRangePickerSpec;
    fn deref(&self) -> &DateRangePickerSpec {
        &self.spec
    }
}

impl DateRangePicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DateRangePickerSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

    pub fn from_spec(spec: DateRangePickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: DateRangeValue) -> Self {
        self.spec.value = Some(v);
        self
    }
    pub fn default_value(mut self, v: DateRangeValue) -> Self {
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

impl IntoElement for DateRangePicker {
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
        let accent = resolve_color(theme, "color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        // Contract: hover = color-mix(surface 86%, elevated)
        let hover_bg = color_mix(surface_bg, elevated_bg, 0.86);

        // Value display mirrors Svelte's `valueLabel`: show range text only when
        // a start exists; partial range (start chosen, end pending) renders the
        // literal `"<start> – End date"` (en-dash + literal "End date"); a
        // missing start always falls back to the placeholder.
        let range = spec.current_value();
        let display_text = match &range.start {
            Some(start) => match &range.end {
                Some(end) => format!("{} – {}", start, end),
                None => format!("{} – End date", start),
            },
            None => spec.placeholder.clone(),
        };
        let is_placeholder = range.start.is_none();
        let is_open = spec.current_open();
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-date-range-picker-{}", suffix)
        } else {
            "poodle-date-range-picker".to_string()
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

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        // Contract §2 + Svelte: disclosure chevron indicator (`▾`),
        // text-secondary, per-size indicator font-size.
        let indicator_px = rem_to_px(date_picker_indicator_font_rem(effective_size));
        trigger = trigger
            .child(div().text_color(text_col).flex_1().child(display_text))
            .child(
                Icon::new("chevron-down", theme)
                    .with_px_size(indicator_px)
                    .with_color(text_secondary),
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

        let mut wrapper = div().flex().flex_col().gap(resolve_px(theme, "space.inline.xs")).child(trigger);

        // Range calendar dropdown when open
        if is_open {
            let mut cal_spec =
                poodle_specs::CalendarSpec::new().with_mode(poodle_specs::CalendarMode::Range);
            cal_spec.week_starts_on = spec.week_starts_on.clone();
            cal_spec.range_value = Some(range.clone());

            if let Some(ref start) = range.start {
                cal_spec.visible_month = Some(start.clone());
            }

            let calendar = Calendar::from_spec(cal_spec, theme);
            let overlay = div()
                .rounded(control_radius)
                // Svelte: color-mix(elevated 98%, panel)
                .bg(color_mix(elevated_bg, panel_bg, 0.98))
                .border_1()
                // Svelte: color-mix(border-default 72%, transparent)
                .border_color(Hsla { a: border.a * 0.72, ..border })
                .shadow(crate::theme_ext::elevation_overlay_shadow())
                .overflow_hidden()
                .child(calendar);
            wrapper = wrapper.child(overlay);
        }

        wrapper.into_any_element()
    }
}
