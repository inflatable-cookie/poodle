//! Surface — real GPUI component backed by SurfaceSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{PaddingScale, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI surface component backed by `SurfaceSpec`.
pub struct Surface {
    spec: SurfaceSpec,
    theme: GpuiThemeProvider,
    /// Child content rendered inside the surface.
    content: Option<AnyElement>,
}

impl std::ops::Deref for Surface {
    type Target = SurfaceSpec;
    fn deref(&self) -> &SurfaceSpec {
        &self.spec
    }
}

impl Surface {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SurfaceSpec::new(),
            theme: theme.clone(),
            content: None,
        }
    }

    pub fn from_spec(spec: SurfaceSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: SurfaceTone) -> Self {
        self.spec.tone = v;
        self
    }
    pub fn border(mut self, v: SurfaceBorder) -> Self {
        self.spec.border = v;
        self
    }
    pub fn padding(mut self, v: PaddingScale) -> Self {
        self.spec.padding = v;
        self
    }
    pub fn elevated(mut self, v: bool) -> Self {
        self.spec.is_elevated = v;
        self
    }
    pub fn role(mut self, v: SurfaceRole) -> Self {
        self.spec.role = Some(v);
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }

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
        let padding = spec.resolved_padding();

        let is_elevated = spec.is_elevated_resolved();

        // ── Background ──────────────────────────────────────────
        // Contract §8 (Svelte Surface.svelte):
        //   panel/base: color-mix(background-surface 96%, transparent) (alpha only)
        //   canvas:     color-mix(background-canvas 98%, transparent)  (alpha only)
        //   elevated:   color-mix(background-elevated 96%, background-panel)
        // Base color, second-color, and ratio all resolve from the spec — the
        // CSS color-mix percentages live on `SurfaceSpec`, not as inline literals.
        let base_fill = resolve_color(theme, spec.resolved_background_token());
        let mix_ratio = spec.fill_mix_ratio();
        let bg = match spec.fill_mix_over_token() {
            // Elevated mixes the base color over a second background token.
            Some(over_token) => {
                let over = resolve_color(theme, over_token);
                color_mix(base_fill, over, mix_ratio)
            }
            // Non-elevated tones mix toward transparent → alpha-only scaling.
            None => Hsla {
                a: base_fill.a * mix_ratio,
                ..base_fill
            },
        };

        // Brand-raised treatment: gradient fill over the base color
        let mut el = div().rounded(surface_radius);
        el = el.bg(bg);

        // ── Border ──────────────────────────────────────────────
        // Contract §8: subtle → color-mix(border-subtle 74%, transparent);
        // default → border-default full; none → no border. Width resolves from
        // the `border.width.default` (0.0625rem) token, not a fixed 1px.
        if let Some(border_token) = spec.resolved_border_color() {
            let base_border = resolve_color(theme, border_token);
            let mix = spec.border_mix_ratio();
            let final_border = match spec.border {
                // color-mix(border-subtle 74%, transparent) → alpha scaling.
                SurfaceBorder::Subtle => Hsla {
                    a: base_border.a * mix,
                    ..base_border
                },
                _ => base_border,
            };
            let border_width = spec
                .resolved_border_width()
                .map(|t| resolve_px(theme, t))
                .unwrap_or(px(0.0));
            el = el.border(border_width).border_color(final_border);
        }

        // ── Shadow ──────────────────────────────────────────────
        // Contract §8: base shadow is `none`; elevated resolves the
        // `elevation.surface` token shadow. The previous undocumented inset ring
        // (raw 0.18 alpha) is removed — the contract base shadow is none.
        if is_elevated {
            el = el.shadow(crate::theme_ext::elevation_surface_shadow());
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
