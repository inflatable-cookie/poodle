//! Tooltip — real GPUI component backed by TooltipSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{OverlayPlacement, TooltipSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI tooltip component backed by `TooltipSpec`.
///
/// Renders the tooltip bubble when open. The parent provides the trigger element
/// and controls the `open` state.
pub struct Tooltip {
    spec: TooltipSpec,
    theme: GpuiThemeProvider,
    /// The trigger element that the tooltip wraps.
    trigger: Option<AnyElement>,
}

impl std::ops::Deref for Tooltip {
    type Target = TooltipSpec;
    fn deref(&self) -> &TooltipSpec { &self.spec }
}

impl Tooltip {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TooltipSpec::new(), theme: theme.clone(), trigger: None }
    }

    pub fn from_spec(spec: TooltipSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = Some(v.into()); self }
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    /// Set the trigger element that the tooltip wraps.
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
}

impl IntoElement for Tooltip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let stack_gap = resolve_px(theme, "space.stack.sm");
        // Contract: border-radius = calc(control-radius - 0.125rem)
        let tooltip_radius = resolve_radius(theme, "radius.control") - px(rem_to_px(spec.radius_inset_rem()));

        // Matches Svelte treatment-surface-elevated values
        let fill = Hsla { a: elevated_bg.a * 0.94, ..elevated_bg };
        let tooltip_border = Hsla { a: border_default.a * 0.22, ..border_default };

        let mut wrapper = div().flex().flex_col().gap(stack_gap);

        // Trigger
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }

        // Tooltip bubble (shown when open)
        if spec.current_open() && spec.has_content() {
            if let Some(ref content) = spec.content {
                // Contract: padding 0.375rem 0.5rem — resolved from spec
                let tooltip_px = px(rem_to_px(spec.padding_x_rem()));
                let tooltip_py = px(rem_to_px(spec.padding_y_rem()));
                let tooltip_font_size = px(rem_to_px(spec.font_size_rem()));
                let tooltip_max_w = px(rem_to_px(spec.max_width_rem()));

                let mut tooltip_bubble = div()
                        .px(tooltip_px)
                        .py(tooltip_py)
                        .rounded(tooltip_radius);

                // Brand-raised treatment: gradient fill for elevated surface
                if theme.brand_raised {
                    tooltip_bubble = tooltip_bubble.bg(crate::theme_ext::brand_raised_surface_fill(fill));
                } else {
                    tooltip_bubble = tooltip_bubble.bg(fill);
                }

                wrapper = wrapper.child(
                    tooltip_bubble
                        .border_1()
                        .border_color(tooltip_border)
                        // Contract: elevation-tooltip shadow
                        .shadow(vec![
                            gpui::BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.12),
                                offset: point(px(0.0), px(2.0)),
                                blur_radius: px(8.0),
                                spread_radius: px(0.0),
                            },
                        ])
                        // Contract: font 0.6875rem, max-width 16rem
                        .text_size(tooltip_font_size)
                        .text_color(text_primary)
                        .max_w(tooltip_max_w)
                        .child(content.clone()),
                );
            }
        }

        wrapper.into_any_element()
    }
}
