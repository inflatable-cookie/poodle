//! Popover — real GPUI component backed by PopoverSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{OverlayPlacement, PopoverInitialFocus, PopoverSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI popover component backed by `PopoverSpec`.
///
/// Renders a trigger element with an optional floating content panel.
/// The parent controls the `open` state.
pub struct Popover {
    spec: PopoverSpec,
    theme: GpuiThemeProvider,
    /// The trigger element that opens the popover.
    trigger: Option<AnyElement>,
    /// The floating content shown when open.
    content: Option<AnyElement>,
}

impl std::ops::Deref for Popover {
    type Target = PopoverSpec;
    fn deref(&self) -> &PopoverSpec { &self.spec }
}

impl Popover {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PopoverSpec::new(), theme: theme.clone(), trigger: None, content: None }
    }

    pub fn from_spec(spec: PopoverSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }
    pub fn dismiss_on_outside_interact(mut self, v: bool) -> Self { self.spec.dismiss_on_outside_interact = v; self }
    pub fn initial_focus(mut self, v: PopoverInitialFocus) -> Self { self.spec.initial_focus = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for Popover {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let surface_raw = resolve_color(theme, spec.surface_fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let border_raw = resolve_color(theme, "semantic.color.border.default");
        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");
        // Contract: radius-surface
        let radius = resolve_radius(theme, "semantic.radius.surface");

        // Contract: bg 98% surface, border 72% border-default
        let surface_bg = color_mix(surface_raw, panel, 0.98);
        let border = color_mix(border_raw, panel, 0.72);

        let mut wrapper = div().flex().flex_col().gap(px(spec.offset as f32));

        // Trigger
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }

        // Floating content (shown when open)
        if spec.current_open() {
            if let Some(content) = self.content {
                wrapper = wrapper.child(
                    div()
                        .rounded(radius)
                        .bg(surface_bg)
                        .border_1()
                        .border_color(border)
                        // Contract: elevation-popover shadow
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
                        .px(panel_x)
                        .py(panel_y)
                        // Contract: min-width 12rem (192px), max-width 24rem (384px)
                        .min_w(px(192.0))
                        .max_w(px(384.0))
                        .child(content),
                );
            }
        }

        wrapper.into_any_element()
    }
}
