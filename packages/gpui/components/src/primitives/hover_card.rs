//! HoverCard — real GPUI component backed by HoverCardSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{HoverCardSpec, OverlayPlacement};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI hover card component backed by `HoverCardSpec`.
pub struct HoverCard {
    spec: HoverCardSpec,
    theme: GpuiThemeProvider,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for HoverCard {
    type Target = HoverCardSpec;
    fn deref(&self) -> &HoverCardSpec { &self.spec }
}

impl HoverCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: HoverCardSpec::new(), theme: theme.clone(), trigger: None, content: None, on_open_change: None }
    }

    pub fn from_spec(spec: HoverCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_open_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }


    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Called when the hover card open state should change (e.g., Escape to close).
    pub fn on_open_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for HoverCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract: padding var(--poodle-space-panel-y) var(--poodle-space-panel-x)
        let panel_x = resolve_px(theme, "space.panel.x");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let radius = resolve_radius(theme, "radius.surface");

        // Matches Svelte treatment-surface-elevated values
        let fill = Hsla { a: elevated_bg.a * 0.94, ..elevated_bg };
        let border = Hsla { a: border_default.a * 0.22, ..border_default };

        // Contract: 0.5rem (8px) gap between trigger and surface
        let trigger_gap = px(rem_to_px(0.5));
        let mut wrapper = div().flex().flex_col().gap(trigger_gap);

        // Trigger (always rendered)
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }

        // Surface content (shown when open)
        // Accessibility semantics (contract section 6):
        // Trigger: role="button", tabindex="0", aria-expanded={open}, aria-controls={surface id}
        // Surface: role="dialog", tabindex="-1", aria-label from spec
        // Escape key clears timers and closes the surface
        if spec.is_open {
            let panel_y = resolve_px(theme, "space.panel.y");
            let mut surface = div()
                .id("poodle-hover-card-surface")
                .focusable()
                .px(panel_x)
                .py(panel_y)
                .rounded(radius);

            // Brand-raised treatment: gradient fill for elevated surface
            if theme.brand_raised {
                surface = surface.bg(crate::theme_ext::brand_raised_surface_fill(fill));
            } else {
                surface = surface.bg(fill);
            }

            surface = surface.border_1()
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
                // Contract: min-width 14rem, max-width min(22rem, 90vw)
                .min_w(px(rem_to_px(14.0)))
                .max_w(px(rem_to_px(22.0)));

            if let Some(content) = self.content {
                surface = surface.child(content);
            }

            // Escape key to close
            if let Some(ref handler) = self.on_open_change {
                let esc_handler = handler.clone();
                surface = surface.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" {
                        esc_handler(false, window, cx);
                    }
                });
            }

            wrapper = wrapper.child(surface);
        }

        wrapper.into_any_element()
    }
}
