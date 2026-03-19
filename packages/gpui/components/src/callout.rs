//! PugCallout — real GPUI component backed by CallOutSpec (contract: callout).

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::CallOutSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI call-out component backed by `CallOutSpec`.
pub struct PugCallout {
    spec: CallOutSpec,
    theme: GpuiThemeProvider,
}

impl PugCallout {
    pub fn new(spec: CallOutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugCallout {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");

        let mut el = div()
            .w_full()
            .px(inline_padding)
            .py(px(10.0))
            .rounded(radius)
            .bg(fill.opacity(0.1))
            .border_l(px(3.0))
            .border_color(border)
            .flex()
            .flex_col()
            .gap(px(4.0));

        if let Some(ref title) = spec.title {
            el = el.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(title.clone()),
            );
        }

        if let Some(ref content) = spec.content {
            el = el.child(
                div()
                    .text_sm()
                    .child(content.clone()),
            );
        }

        el.into_any_element()
    }
}
