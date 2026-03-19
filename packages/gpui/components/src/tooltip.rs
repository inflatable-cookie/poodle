//! PugTooltip — real GPUI component backed by TooltipSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TooltipSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI tooltip component backed by `TooltipSpec`.
///
/// Renders the tooltip bubble when open. The parent provides the trigger element
/// and controls the `open` state.
pub struct PugTooltip {
    spec: TooltipSpec,
    theme: GpuiThemeProvider,
    /// The trigger element that the tooltip wraps.
    trigger: Option<AnyElement>,
}

impl PugTooltip {
    pub fn new(spec: TooltipSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
        }
    }

    /// Set the trigger element that the tooltip wraps.
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
}

impl IntoElement for PugTooltip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_padding_y = resolve_px(theme, "semantic.space.control.y");
        let tooltip_radius = resolve_radius(theme, "semantic.radius.surface");

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
                        .px(control_padding_x)
                        .py(control_padding_y)
                        .rounded(tooltip_radius)
                        .bg(fill)
                        .shadow_sm()
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_inverse)
                                .child(content.clone()),
                        ),
                );
            }
        }

        wrapper.into_any_element()
    }
}
