//! Surface — real GPUI component backed by SurfaceSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{PaddingScale, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone};

use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

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

        let is_elevated = spec.is_elevated || spec.tone == SurfaceTone::Elevated;

        // Colors from token system
        let panel = resolve_color(theme, "color.background.panel");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_subtle = resolve_color(theme, "color.border.subtle");

        // ── Background ──────────────────────────────────────────
        // Svelte Surface.svelte:
        //   non-elevated: color-mix(surface 96%, transparent) (alpha only)
        //   elevated: color-mix(elevated 96%, panel)
        let surface_bg = resolve_color(theme, "color.background.surface");
        let bg = if is_elevated {
            color_mix(elevated_bg, panel, 0.96)
        } else {
            Hsla { a: surface_bg.a * 0.96, ..surface_bg }
        };

        // Brand-raised treatment: gradient fill over the base color
        let mut el = div().rounded(surface_radius);
        if theme.brand_raised {
            el = el.bg(crate::theme_ext::brand_raised_surface_fill(bg));
        } else {
            el = el.bg(bg);
        }

        // ── Border ──────────────────────────────────────────────
        // Matches Svelte app.css treatment values:
        //   non-elevated subtle: color-mix(border-subtle 30%, transparent)
        //   elevated subtle: color-mix(border-default 22%, transparent)
        //   default: border-default full
        //   none: transparent
        if let Some(border_token) = spec.resolved_border_color() {
            let border_color = resolve_color(theme, border_token);
            let final_border = match spec.border {
                SurfaceBorder::Subtle => {
                    if is_elevated {
                        let border_default = resolve_color(theme, "color.border.default");
                        Hsla {
                            a: border_default.a * 0.22,
                            ..border_default
                        }
                    } else {
                        Hsla {
                            a: border_color.a * 0.30,
                            ..border_color
                        }
                    }
                }
                _ => border_color,
            };
            el = el.border_1().border_color(final_border);
        }

        // ── Shadow ──────────────────────────────────────────────
        // Matches Svelte app.css treatment values:
        //   non-elevated: inset 0 0 0 1px border-subtle at 18%
        //   elevated: elevation-surface token shadow
        if is_elevated {
            el = el.shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.04),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(2.0),
                    spread_radius: px(0.0),
                },
            ]);
        } else {
            // Inset shadow ring: 1px border-subtle at 18%
            el = el.shadow(vec![gpui::BoxShadow {
                color: Hsla {
                    a: border_subtle.a * 0.18,
                    ..border_subtle
                },
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: px(1.0),
            }]);
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
