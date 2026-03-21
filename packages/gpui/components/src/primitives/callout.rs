//! Callout — real GPUI component backed by CallOutSpec (contract: callout).

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CallOutSpec, StatusTone};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI call-out component backed by `CallOutSpec`.
pub struct Callout {
    spec: CallOutSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Callout {
    type Target = CallOutSpec;
    fn deref(&self) -> &CallOutSpec { &self.spec }
}

impl Callout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CallOutSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: CallOutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: StatusTone) -> Self { self.spec.tone = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = Some(v.into()); self }

}

impl IntoElement for Callout {
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
