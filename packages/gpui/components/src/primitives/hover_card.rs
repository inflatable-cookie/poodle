//! HoverCard — real GPUI component backed by HoverCardSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{HoverCardSpec, OverlayPlacement};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI hover card component backed by `HoverCardSpec`.
pub struct HoverCard {
    spec: HoverCardSpec,
    theme: GpuiThemeProvider,
    content: Option<AnyElement>,
}

impl std::ops::Deref for HoverCard {
    type Target = HoverCardSpec;
    fn deref(&self) -> &HoverCardSpec { &self.spec }
}

impl HoverCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: HoverCardSpec::new(), theme: theme.clone(), content: None }
    }

    pub fn from_spec(spec: HoverCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }


    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for HoverCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        if !spec.is_open {
            return div().into_any_element();
        }

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let radius = resolve_radius(theme, "semantic.radius.surface");

        let mut el = div()
            .px(inline_padding)
            .py(px(10.0))
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border.opacity(0.3));

        if let Some(content) = self.content {
            el = el.child(content);
        }

        el.into_any_element()
    }
}
