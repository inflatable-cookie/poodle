//! Surface — real GPUI component backed by SurfaceSpec.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{PaddingScale, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone};

use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI surface component backed by `SurfaceSpec`.
pub struct Surface {
    spec: SurfaceSpec,
    theme: GpuiThemeProvider,
    /// Child content rendered inside the surface.
    content: Option<AnyElement>,
}

impl std::ops::Deref for Surface {
    type Target = SurfaceSpec;
    fn deref(&self) -> &SurfaceSpec { &self.spec }
}

impl Surface {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SurfaceSpec::new(), theme: theme.clone(), content: None }
    }

    pub fn from_spec(spec: SurfaceSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: SurfaceTone) -> Self { self.spec.tone = v; self }
    pub fn border(mut self, v: SurfaceBorder) -> Self { self.spec.border = v; self }
    pub fn padding(mut self, v: PaddingScale) -> Self { self.spec.padding = v; self }
    pub fn elevated(mut self, v: bool) -> Self { self.spec.is_elevated = v; self }
    pub fn role(mut self, v: SurfaceRole) -> Self { self.spec.role = Some(v); self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }


    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for Surface {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract: use radius.surface, not radius.control
        let surface_radius = resolve_radius(theme, spec.radius_token());

        let bg = resolve_color(theme, spec.resolved_background_token());
        let padding = spec.resolved_padding();

        let mut el = div().rounded(surface_radius).bg(bg);

        // Border
        if let Some(border_token) = spec.resolved_border_color() {
            let border_color = resolve_color(theme, border_token);
            el = el.border_1().border_color(border_color);
        }

        // Shadow for elevated surfaces
        if spec.is_elevated || spec.tone == SurfaceTone::Elevated {
            el = el.shadow_md();
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
