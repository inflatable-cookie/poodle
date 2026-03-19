//! PugSurface — real GPUI component backed by SurfaceSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SurfaceSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI surface component backed by `SurfaceSpec`.
pub struct PugSurface {
    spec: SurfaceSpec,
    theme: GpuiThemeProvider,
    /// Child content rendered inside the surface.
    content: Option<AnyElement>,
}

impl PugSurface {
    pub fn new(spec: SurfaceSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
        }
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for PugSurface {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let bg = resolve_color(theme, spec.resolved_background_token());
        let padding = spec.resolved_padding();

        let mut el = div().rounded(control_radius).bg(bg);

        // Border
        if let Some(border_token) = spec.resolved_border_color() {
            let border_color = resolve_color(theme, border_token);
            el = el.border_1().border_color(border_color);
        }

        // Padding
        if let Some(px_token) = padding.horizontal {
            let px_val = theme.resolve_space(px_token);
            el = el.px(px(px_val));
        }
        if let Some(py_token) = padding.vertical {
            let py_val = theme.resolve_space(py_token);
            el = el.py(px(py_val));
        }

        // Content
        if let Some(content) = self.content {
            el = el.child(content);
        }

        el.into_any_element()
    }
}
