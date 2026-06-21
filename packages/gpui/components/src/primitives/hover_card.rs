//! HoverCard — real GPUI component backed by HoverCardSpec.

use gpui::*;
use gpui::StatefulInteractiveElement;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{HoverCardSpec, OverlayPlacement};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

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
    fn deref(&self) -> &HoverCardSpec {
        &self.spec
    }
}

impl HoverCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: HoverCardSpec::new(),
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_open_change: None,
        }
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
    pub fn open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
    pub fn placement(mut self, v: OverlayPlacement) -> Self {
        self.spec.placement = v;
        self
    }

    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Called when the hover card open state should change (e.g., Escape to close).
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for HoverCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let placement_id = match spec.placement {
            OverlayPlacement::Top => 0_usize,
            OverlayPlacement::TopStart => 1_usize,
            OverlayPlacement::TopEnd => 2_usize,
            OverlayPlacement::Right => 3_usize,
            OverlayPlacement::RightStart => 4_usize,
            OverlayPlacement::RightEnd => 5_usize,
            OverlayPlacement::Bottom => 6_usize,
            OverlayPlacement::BottomStart => 7_usize,
            OverlayPlacement::BottomEnd => 8_usize,
            OverlayPlacement::Left => 9_usize,
            OverlayPlacement::LeftStart => 10_usize,
            OverlayPlacement::LeftEnd => 11_usize,
        };

        // Contract: padding var(--poodle-space-panel-y) var(--poodle-space-panel-x)
        let panel_x = resolve_px(theme, "space.panel.x");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let radius = resolve_radius(theme, "radius.surface");

        // Svelte: --poodle-treatment-surface-elevated-fill = color-mix(elevated 98%, panel)
        let panel_bg = resolve_color(theme, "color.background.panel");
        let fill = color_mix(elevated_bg, panel_bg, 0.98);
        // Svelte: color-mix(border-default 72%, transparent)
        let border = Hsla {
            a: border_default.a * 0.72,
            ..border_default
        };

        // Contract: 0.5rem (8px) gap between trigger and surface
        let trigger_gap = px(rem_to_px(0.5));
        let mut wrapper = div().flex().flex_col().gap(trigger_gap);

        // Trigger (always rendered)
        // Contract §8 Trigger Focus: outline border-width-focus solid
        // accent.focusRing, outline-offset 0.125rem. GPUI has no CSS outline;
        // approximate via a focus border-color + ring shadow (same convention
        // as Button). Known delta: no outline-offset separation.
        if let Some(trigger) = self.trigger {
            let focus_ring_color = resolve_color(theme, "color.accent.focusRing");
            let mut trigger_wrapper = div()
                .id(("poodle-hover-card-trigger", placement_id))
                .focusable()
                .rounded(radius)
                .focus(move |s| {
                    s.border_color(focus_ring_color)
                        .shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color))
                });
            trigger_wrapper = trigger_wrapper.child(trigger);
            if let Some(ref handler) = self.on_open_change {
                let hover_handler = handler.clone();
                trigger_wrapper = trigger_wrapper.on_hover(move |hovered, window, cx| {
                    hover_handler(*hovered, window, cx);
                });
            }
            wrapper = wrapper.child(trigger_wrapper);
        }

        // Surface content (shown when open)
        // Accessibility semantics (contract section 6):
        // Trigger: role="button", tabindex="0", aria-expanded={open}, aria-controls={surface id}
        // Surface: role="dialog", tabindex="-1", aria-label from spec
        // Escape key clears timers and closes the surface
        if spec.current_open() {
            let panel_y = resolve_px(theme, "space.panel.y");
            let menu_min_w = resolve_px(theme, "size.menu.minWidth");
            let hover_card_max_w = resolve_px(theme, "size.hoverCard.maxWidth");
            let mut surface = div()
                .id(("poodle-hover-card-surface", placement_id))
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

            surface = surface
                .border_1()
                .border_color(border)
                // Contract: elevation-popover shadow
                .shadow(crate::theme_ext::elevation_overlay_shadow())
                // Contract: min-width 14rem, max-width min(22rem, 90vw)
                .min_w(menu_min_w)
                .max_w(hover_card_max_w);

            if let Some(content) = self.content {
                surface = surface.child(content);
            }

            if let Some(ref handler) = self.on_open_change {
                let hover_handler = handler.clone();
                surface = surface.on_hover(move |hovered, window, cx| {
                    hover_handler(*hovered, window, cx);
                });
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
