//! Popover — real GPUI component backed by PopoverSpec.

use gpui::*;
use gpui::StatefulInteractiveElement;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, OverlayPlacement, PopoverInitialFocus, PopoverSpec};

use crate::presentation::{control_height_rem, rem_to_px};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};
use super::floating_overlay::floating_overlay;

/// A real GPUI popover component backed by `PopoverSpec`.
///
/// Renders a trigger element with an optional floating content panel.
/// The panel is positioned absolutely so it does not push surrounding layout.
/// The parent controls `open` state.
///
/// # Known GPUI deltas
/// - `role="dialog"` / `aria-expanded` on trigger: GPUI native rendering does
///   not carry HTML ARIA attributes. These are documented-only deltas.
/// - `dismiss_on_outside_interact`: GPUI has no window-level outside-click
///   interceptor for arbitrary elements. Escape-to-close is wired instead.
/// - `initialFocus`: GPUI focus management (cx.focus) is entity-scoped. The
///   parent is responsible for focusing a child element after opening.
pub struct Popover {
    spec: PopoverSpec,
    theme: GpuiThemeProvider,
    /// The trigger element that opens the popover.
    trigger: Option<AnyElement>,
    /// The floating content shown when open.
    content: Option<AnyElement>,
    /// Called when the popover open state should change (click to toggle, Escape to close).
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Popover {
    type Target = PopoverSpec;
    fn deref(&self) -> &PopoverSpec {
        &self.spec
    }
}

impl Popover {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: PopoverSpec::new(),
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_open_change: None,
        }
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
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.spec.default_open = v;
        self
    }
    pub fn placement(mut self, v: OverlayPlacement) -> Self {
        self.spec.placement = v;
        self
    }
    pub fn dismiss_on_outside_interact(mut self, v: bool) -> Self {
        self.spec.dismiss_on_outside_interact = v;
        self
    }
    pub fn initial_focus(mut self, v: PopoverInitialFocus) -> Self {
        self.spec.initial_focus = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
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

    /// Called when the popover open state should change (click to toggle, Escape to close).
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Popover {
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

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let panel_x = resolve_px(theme, "space.panel.x");
        let panel_y = resolve_px(theme, "space.panel.y");
        let menu_min_w = resolve_px(theme, "size.menu.minWidth");
        let popover_max_w = resolve_px(theme, "size.popover.maxWidth");
        let radius = resolve_radius(theme, "radius.surface");

        // Svelte Popover.svelte: background = background-elevated (no mix)
        //         border = color-mix(border-subtle 74%, transparent)
        let surface_bg = elevated_bg;
        let border = Hsla {
            a: border_subtle.a * 0.74,
            ..border_subtle
        };

        // ── Build trigger element ─────────────────────────────────────────────
        // Contract §6: trigger carries role="button", aria-expanded, aria-controls.
        // GPUI note: aria-* attributes are not expressible on native GPU elements;
        // documented delta — the trigger is a focusable click target only.
        let trigger_el: AnyElement = if let Some(trigger) = self.trigger {
            let mut trigger_wrapper =
                div().id(("poodle-popover-trigger", placement_id)).child(trigger);
            if let Some(ref handler) = self.on_open_change {
                let click_handler = handler.clone();
                let next_open = !spec.current_open();
                trigger_wrapper =
                    trigger_wrapper.on_click(move |_event, window, cx| {
                        click_handler(next_open, window, cx);
                    });
            }
            trigger_wrapper.into_any_element()
        } else {
            div().into_any_element()
        };

        // ── Build surface element (None when closed) ──────────────────────────
        // Contract §6: surface carries role="dialog", tabindex, aria-label.
        // GPUI note: same delta as trigger — ARIA not expressible.
        let surface_el: Option<AnyElement> = if spec.current_open() {
            self.content.map(|content| {
                let mut surface = div()
                    .id("poodle-popover-surface")
                    .focusable()
                    .rounded(radius)
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
                    // Contract: min-width 14rem, max-width min(24rem, 90vw)
                    .min_w(menu_min_w)
                    .max_w(popover_max_w)
                    .child(content);

                // Brand-raised treatment: gradient fill for elevated surface
                if theme.brand_raised {
                    surface = surface.bg(crate::theme_ext::brand_raised_surface_fill(surface_bg));
                } else {
                    surface = surface.bg(surface_bg);
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

                surface.into_any_element()
            })
        } else {
            None
        };

        // ── Floating overlay — positions surface above the document flow ───────
        // anchor_h: Md control height is the most common trigger size.
        // dismiss_on_outside_interact: see struct-level doc comment for delta note.
        let anchor_h = px(rem_to_px(control_height_rem(ControlSize::Md)));

        floating_overlay(trigger_el, surface_el, spec.placement, anchor_h, anchor_h)
    }
}
