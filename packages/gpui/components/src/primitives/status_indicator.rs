//! StatusIndicator — real GPUI component backed by StatusIndicatorSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, SemanticControlSizeRole, StatusIndicatorSpec, StatusTone,
};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::resolve_color;

/// A real GPUI status indicator (colored dot + optional label).
pub struct StatusIndicator {
    spec: StatusIndicatorSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for StatusIndicator {
    type Target = StatusIndicatorSpec;
    fn deref(&self) -> &StatusIndicatorSpec {
        &self.spec
    }
}

impl StatusIndicator {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: StatusIndicatorSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: StatusIndicatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn status(mut self, v: StatusTone) -> Self {
        self.spec.status = v;
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn typography(mut self, v: poodle_specs::InlineTypographyMode) -> Self {
        self.spec.typography = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = Some(v);
        self
    }
    pub fn size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = Some(v);
        self
    }
}

impl IntoElement for StatusIndicator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let status_color = resolve_color(theme, spec.status_color_token());
        let label_color = resolve_color(theme, spec.label_color_token());

        // Contract §8: dot/gap/label metrics resolve from the effective size
        // (size override → size_role against the inherited scale) and density.
        let effective_size =
            resolve_semantic_size(spec.size.unwrap_or(ControlSize::Md), spec.size_role);
        let effective_density = spec.density.unwrap_or(ControlDensity::Default);

        let gap = px(rem_to_px(spec.gap_rem_for(effective_size, effective_density)));
        let dot_size = px(rem_to_px(spec.dot_size_rem_for(effective_size)));
        let label_font_size = px(rem_to_px(spec.label_font_size_rem_for(effective_size)));

        let mut row = div().flex().items_center().gap(gap);

        // Contract: dot 0.5625rem with full circle radius and shadow ring
        row = row.child(
            div()
                .w(dot_size)
                .h(dot_size)
                .rounded(px(999.0))
                .bg(status_color)
                .flex_shrink_0()
                // Svelte: box-shadow 0 0 0 0.125rem at 18% opacity (no blur)
                .shadow(vec![gpui::BoxShadow {
                    color: Hsla {
                        a: status_color.a * 0.18,
                        ..status_color
                    },
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(rem_to_px(0.125)),
                }]),
        );

        // Contract: label font 0.75rem, weight 600, line-height 1.3
        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_size(label_font_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .line_height(relative(1.3))
                    .text_color(label_color)
                    .child(label.clone()),
            );
        }

        row.into_any_element()
    }
}
