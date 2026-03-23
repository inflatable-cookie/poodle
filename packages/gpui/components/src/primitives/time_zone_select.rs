//! TimeZoneSelect — real GPUI component backed by TimeZoneSelectSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{IconSize, IconSpec, TimeZoneSelectSpec};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};
use super::icon::Icon;

/// A real GPUI timezone select dropdown component backed by `TimeZoneSelectSpec`.
pub struct TimeZoneSelect {
    spec: TimeZoneSelectSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TimeZoneSelect {
    type Target = TimeZoneSelectSpec;
    fn deref(&self) -> &TimeZoneSelectSpec { &self.spec }
}

impl TimeZoneSelect {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TimeZoneSelectSpec::new(), theme: theme.clone(), on_toggle: None }
    }

    pub fn from_spec(spec: TimeZoneSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
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

impl IntoElement for TimeZoneSelect {
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

        let trigger_text = spec
            .trigger_text()
            .unwrap_or("Select timezone...")
            .to_string();
        let is_placeholder = spec.value.is_none();
        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let hover_bg = resolve_color(theme, "semantic.color.background.hover");

        let mut trigger = div()
            .id(SharedString::from("poodle-tz-select"))
            .focusable()
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_between()
            .gap(inline_gap)
            .text_size(px(14.0))
            .child(div().text_color(text_col).child(trigger_text))
            .child(
                Icon::from_spec(
                    IconSpec::new(if spec.is_open { "chevron-up" } else { "chevron-down" }).with_size(IconSize::Sm),
                    theme,
                ).with_color(text_secondary),
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
            let timezones = [
                "UTC",
                "America/New_York",
                "America/Chicago",
                "America/Denver",
                "America/Los_Angeles",
                "Europe/London",
                "Europe/Paris",
                "Europe/Berlin",
                "Asia/Tokyo",
                "Asia/Shanghai",
                "Australia/Sydney",
            ];

            let mut dropdown = div()
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
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
                .py(px(4.0))
                .text_size(px(14.0))
                .text_color(text_primary);

            for tz in timezones {
                dropdown = dropdown.child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .cursor_pointer()
                        .hover(move |s| s.bg(hover_bg))
                        .child(tz),
                );
            }

            wrapper = wrapper.child(dropdown);
        }

        wrapper.into_any_element()
    }
}
