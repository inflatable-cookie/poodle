//! Switch — real GPUI component backed by SwitchSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::SwitchSpec;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px};

/// A real GPUI switch/toggle component backed by `SwitchSpec`.
pub struct Switch {
    spec: SwitchSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Switch {
    type Target = SwitchSpec;
    fn deref(&self) -> &SwitchSpec { &self.spec }
}

impl Switch {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SwitchSpec::new(), theme: theme.clone(), id_suffix: None, on_change: None }
    }

    pub fn from_spec(spec: SwitchSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn checked(mut self, v: bool) -> Self { self.spec.checked = Some(v); self }
    pub fn default_checked(mut self, v: bool) -> Self { self.spec.default_checked = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn read_only(mut self, v: bool) -> Self { self.spec.is_read_only = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Switch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract: gap = space-inline-sm
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let is_checked = spec.current_checked();
        let is_interactive = !spec.is_disabled && !spec.is_read_only;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-switch-{}", suffix)
        } else {
            format!(
                "pug-switch-{}",
                spec.label.as_deref().unwrap_or("anon")
            )
        };

        // Contract: track = 2.125rem (34px) wide x 1.25rem (20px) tall
        let track_w = px(34.0);
        let track_h = px(20.0);
        let track_radius = px(10.0); // half of height = pill
        let track_padding = px(2.0); // 0.125rem

        // Contract: thumb = 0.875rem (14px) diameter
        let thumb_size = px(14.0);
        let thumb_radius = px(7.0); // half = circle

        // Contract: thumb travel = translateX(0.875rem) = 14px
        let knob_offset = if is_checked { px(14.0) + track_padding } else { track_padding };
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        // Contract: checked track = accent-base 24% + surface background
        let track_bg = if is_checked {
            color_mix(accent, surface_bg, 0.24)
        } else {
            color_mix(surface_bg, gpui::transparent_black(), 0.86)
        };

        // Contract: checked track border = accent-base 58% + border-default
        let track_border = if is_checked {
            color_mix(accent, border, 0.58)
        } else {
            border
        };

        // Contract: checked thumb = accent-base, unchecked = text-primary
        let knob_color = if is_checked { accent } else { text_primary };

        // Contract: track inset shadow = inset 0 0 0 1px white/8%
        let inset_shadow_color = hsla(0.0, 0.0, 1.0, 0.08);

        let track = div()
            .w(track_w)
            .h(track_h)
            .rounded(track_radius)
            .bg(track_bg)
            .border_1()
            .border_color(track_border)
            .relative()
            .flex_shrink_0()
            // Contract: inset shadow simulated via inner border highlight
            .shadow(vec![gpui::BoxShadow {
                color: inset_shadow_color,
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: px(1.0),
            }])
            .child(
                div()
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(knob_color)
                    // Contract: thumb shadow = 0 1px 2px rgba(0,0,0,0.2), 0 0 1px rgba(0,0,0,0.1)
                    .shadow(vec![
                        gpui::BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.2),
                            offset: point(px(0.0), px(1.0)),
                            blur_radius: px(2.0),
                            spread_radius: px(0.0),
                        },
                        gpui::BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.1),
                            offset: point(px(0.0), px(0.0)),
                            blur_radius: px(1.0),
                            spread_radius: px(0.0),
                        },
                    ])
                    .absolute()
                    .top(track_padding)
                    .left(knob_offset),
            );

        // Row: track + label
        let mut row = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .items_center()
            .gap(inline_gap)
            .focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            row = row.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        } else if spec.is_read_only {
            row = row.cursor_default();
        } else {
            row = row.cursor_pointer();
        }

        row = row.child(track);

        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        // Click + keyboard handlers
        if let Some(handler) = self.on_change {
            if is_interactive {
                let next = !is_checked;
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                row = row
                    .on_click(move |_event, window, cx| {
                        handler(&next, window, cx);
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&next, window, cx);
                        }
                    });
            }
        }

        row.into_any_element()
    }
}
