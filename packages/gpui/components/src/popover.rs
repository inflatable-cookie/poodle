//! PugPopover — real GPUI component backed by PopoverSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::PopoverSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI popover component backed by `PopoverSpec`.
///
/// Renders a trigger element with an optional floating content panel.
/// The parent controls the `open` state.
pub struct PugPopover {
    spec: PopoverSpec,
    theme: GpuiThemeProvider,
    /// The trigger element that opens the popover.
    trigger: Option<AnyElement>,
    /// The floating content shown when open.
    content: Option<AnyElement>,
}

impl PugPopover {
    pub fn new(spec: PopoverSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
        }
    }

    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for PugPopover {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let surface_bg = resolve_color(theme, spec.surface_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");

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
                        .rounded(px(8.0))
                        .bg(surface_bg)
                        .border_1()
                        .border_color(border)
                        .shadow_lg()
                        .p(px(12.0))
                        .child(content),
                );
            }
        }

        wrapper.into_any_element()
    }
}
