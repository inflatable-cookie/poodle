//! Popover — real GPUI component backed by PopoverSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{OverlayPlacement, PopoverInitialFocus, PopoverSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

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
    /// Called when the popover open state should change (Escape to close).
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Popover {
    type Target = PopoverSpec;
    fn deref(&self) -> &PopoverSpec { &self.spec }
}

impl Popover {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PopoverSpec::new(), theme: theme.clone(), trigger: None, content: None, on_open_change: None }
    }

    pub fn from_spec(spec: PopoverSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_open_change: None,
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

    /// Called when the popover open state should change (e.g., Escape to close).
    pub fn on_open_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Popover {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let panel_x = resolve_px(theme, "space.panel.x");
        let panel_y = resolve_px(theme, "space.panel.y");
        let radius = resolve_radius(theme, "radius.surface");

        // Matches Svelte treatment-surface-elevated values
        let surface_bg = Hsla { a: elevated_bg.a * 0.94, ..elevated_bg };
        let border = Hsla { a: border_default.a * 0.22, ..border_default };

        let mut wrapper = div().flex().flex_col().gap(px(spec.offset as f32));

        // Trigger
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }

        // Floating content (shown when open)
        // Accessibility semantics (contract section 6):
        // Trigger: role="button", tabindex="0", aria-expanded={open}, aria-controls={surface id}
        // Surface: role="dialog", tabindex per initialFocus strategy
        // aria-label on surface from spec.aria_label when provided
        if spec.current_open() {
            if let Some(content) = self.content {
                let mut surface = div()
                    .id("poodle-popover-surface")
                    .focusable()
                    .rounded(radius);

                // Brand-raised treatment: gradient fill for elevated surface
                if theme.brand_raised {
                    surface = surface.bg(crate::theme_ext::brand_raised_surface_fill(surface_bg));
                } else {
                    surface = surface.bg(surface_bg);
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
                    .px(panel_x)
                    .py(panel_y)
                    // Contract: min-width 14rem, max-width min(24rem, 90vw)
                    .min_w(px(rem_to_px(14.0)))
                    .max_w(px(rem_to_px(24.0)))
                    .child(content);

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
        }

        wrapper.into_any_element()
    }
}
