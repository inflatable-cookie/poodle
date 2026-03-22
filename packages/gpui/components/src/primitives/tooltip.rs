//! Tooltip — real GPUI component backed by TooltipSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{OverlayPlacement, TooltipSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

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

        let fill = resolve_color(theme, spec.fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        // Contract: text color = text-primary (tooltip uses dark elevated bg)
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let border_raw = resolve_color(theme, "semantic.color.border.default");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        // Contract: radius-sm (smaller than control)
        let tooltip_radius = resolve_radius(theme, "semantic.radius.control") - px(2.0);
        // Contract: border = 72% border-default
        let tooltip_border = color_mix(border_raw, panel, 0.72);

        let mut wrapper = div().flex().flex_col().gap(stack_gap);

        // Trigger
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }

        // Tooltip bubble (shown when open)
        if spec.current_open() && spec.has_content() {
            if let Some(ref content) = spec.content {
                wrapper = wrapper.child(
                    div()
                        .px(px(8.0))  // 0.5rem
                        .py(px(4.0))  // 0.25rem
                        .rounded(tooltip_radius)
                        .bg(fill)
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
                        // Contract: font 0.75rem (12px), max-width 20rem (320px)
                        .text_size(px(12.0))
                        .text_color(text_primary)
                        .max_w(px(320.0))
                        .child(content.clone()),
                );
            }
        }

        wrapper.into_any_element()
    }
}
