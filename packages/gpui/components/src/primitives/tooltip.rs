//! Tooltip — real GPUI component backed by TooltipSpec.

use gpui::*;
use gpui::StatefulInteractiveElement;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, OverlayPlacement, TooltipSpec};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::presentation::{control_height_rem, rem_to_px};
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};
use super::floating_overlay::floating_overlay;

/// A real GPUI tooltip component backed by `TooltipSpec`.
///
/// Renders the tooltip bubble in a floating overlay when open. The bubble is
/// positioned absolutely so it does not push surrounding layout. The parent
/// controls `open` state and wires `on_open_change` to a hover handler or timer.
pub struct Tooltip {
    spec: TooltipSpec,
    theme: GpuiThemeProvider,
    /// The trigger element that the tooltip wraps.
    trigger: Option<AnyElement>,
    /// Called when the tooltip open state should change from trigger hover.
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Tooltip {
    type Target = TooltipSpec;
    fn deref(&self) -> &TooltipSpec {
        &self.spec
    }
}

impl Tooltip {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: TooltipSpec::new(),
            theme: theme.clone(),
            trigger: None,
            on_open_change: None,
        }
    }

    pub fn from_spec(spec: TooltipSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            on_open_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self {
        self.spec.content = Some(v.into());
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
    pub fn placement(mut self, v: OverlayPlacement) -> Self {
        self.spec.placement = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    /// Set the trigger element that the tooltip wraps.
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Tooltip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Stable element ID derived from content + placement so the trigger
        // div keeps the same ID across frames when state changes.
        let placement_id = match spec.placement {
            OverlayPlacement::Top => 0_u64,
            OverlayPlacement::TopStart => 1_u64,
            OverlayPlacement::TopEnd => 2_u64,
            OverlayPlacement::Right => 3_u64,
            OverlayPlacement::RightStart => 4_u64,
            OverlayPlacement::RightEnd => 5_u64,
            OverlayPlacement::Bottom => 6_u64,
            OverlayPlacement::BottomStart => 7_u64,
            OverlayPlacement::BottomEnd => 8_u64,
            OverlayPlacement::Left => 9_u64,
            OverlayPlacement::LeftStart => 10_u64,
            OverlayPlacement::LeftEnd => 11_u64,
        };
        let mut hasher = DefaultHasher::new();
        spec.content.hash(&mut hasher);
        spec.aria_label.hash(&mut hasher);
        placement_id.hash(&mut hasher);
        let trigger_id = hasher.finish();

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        // Contract: border-radius = calc(control-radius - 0.125rem)
        let tooltip_radius =
            resolve_radius(theme, "radius.control") - px(rem_to_px(spec.radius_inset_rem()));

        // Svelte default = color-mix(elevated 98%, panel)
        let panel_bg = resolve_color(theme, "color.background.panel");
        let fill = color_mix(elevated_bg, panel_bg, 0.98);
        // Svelte: color-mix(border-default 72%, transparent)
        let tooltip_border = Hsla {
            a: border_default.a * 0.72,
            ..border_default
        };

        // ── Build trigger element ─────────────────────────────────────────────
        let trigger_el: AnyElement = if let Some(trigger) = self.trigger {
            let mut trigger_wrapper =
                div().id(("poodle-tooltip-trigger", trigger_id)).child(trigger);
            if let Some(ref handler) = self.on_open_change {
                let hover_handler = handler.clone();
                trigger_wrapper = trigger_wrapper.on_hover(move |hovered, window, cx| {
                    hover_handler(*hovered, window, cx);
                });
            }
            trigger_wrapper.into_any_element()
        } else {
            div().into_any_element()
        };

        // ── Build bubble element (None when closed / no content) ──────────────
        let bubble_el: Option<AnyElement> = if spec.current_open() && spec.has_content() {
            spec.content.as_ref().map(|content| {
                let tooltip_px = px(rem_to_px(spec.padding_x_rem()));
                let tooltip_py = px(rem_to_px(spec.padding_y_rem()));
                let tooltip_font_size = px(rem_to_px(spec.font_size_rem()));
                let tooltip_max_w = px(rem_to_px(spec.max_width_rem()));

                let mut bubble = div()
                    .px(tooltip_px)
                    .py(tooltip_py)
                    .rounded(tooltip_radius)
                    .border_1()
                    .border_color(tooltip_border)
                    // Svelte: 0 0.5rem 1.25rem rgba(0,0,0,0.3), 0 0.125rem 0.375rem rgba(0,0,0,0.2)
                    .shadow(vec![
                        gpui::BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.30),
                            offset: point(px(0.0), px(rem_to_px(0.5))),
                            blur_radius: px(rem_to_px(1.25)),
                            spread_radius: px(0.0),
                        },
                        gpui::BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.20),
                            offset: point(px(0.0), px(rem_to_px(0.125))),
                            blur_radius: px(rem_to_px(0.375)),
                            spread_radius: px(0.0),
                        },
                    ])
                    .text_size(tooltip_font_size)
                    .text_color(text_primary)
                    .max_w(tooltip_max_w)
                    .child(content.clone());

                bubble = bubble.bg(fill);

                bubble.into_any_element()
            })
        } else {
            None
        };

        // ── Floating overlay — positions bubble above the document flow ────────
        // anchor_h: Md control height is the most common trigger size. TooltipSpec
        // has no explicit size prop, so we use the Md baseline (2.25rem).
        let anchor_h = px(rem_to_px(control_height_rem(ControlSize::Md)));

        floating_overlay(trigger_el, bubble_el, spec.placement, anchor_h, anchor_h)
    }
}
