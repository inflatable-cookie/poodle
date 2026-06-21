//! Eyebrow — real GPUI component backed by EyebrowSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowElement, EyebrowSize, EyebrowSpacing, EyebrowSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// A real GPUI eyebrow label component backed by `EyebrowSpec`.
pub struct Eyebrow {
    spec: EyebrowSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Eyebrow {
    type Target = EyebrowSpec;
    fn deref(&self) -> &EyebrowSpec {
        &self.spec
    }
}

impl Eyebrow {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EyebrowSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: EyebrowSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self {
        self.spec.content = Some(v.into());
        self
    }

    pub fn element(mut self, v: EyebrowElement) -> Self {
        self.spec.element = v;
        self
    }

    pub fn size(mut self, v: EyebrowSize) -> Self {
        self.spec.size = v;
        self
    }

    pub fn spacing(mut self, v: EyebrowSpacing) -> Self {
        self.spec.spacing = v;
        self
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
}

impl IntoElement for Eyebrow {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_color = resolve_color(theme, spec.text_color_token());
        // Contract: uppercase the content
        let label = spec.content.clone().unwrap_or_default().to_uppercase();

        // Contract §8: size-dependent font-size (0.6875rem xs/sm, 0.85rem md),
        // weight 600, line-height 1.5, label font-family. letter-spacing
        // (0.12em / 0.04em md) has no GPUI text-style channel — accepted delta.
        let font_size = px(rem_to_px(spec.font_size_rem()));
        let font_weight = FontWeight(spec.font_weight() as f32);
        let mut el = div()
            .text_size(font_size)
            .text_color(text_color)
            .font_family(spec.font_family())
            .font_weight(font_weight)
            .line_height(relative(spec.line_height()));

        // Contract §8 spacing="bottom": bottom margin (0.5rem, 0.35rem for xs).
        let mb = spec.margin_bottom_rem();
        if mb > 0.0 {
            el = el.mb(px(rem_to_px(mb)));
        }

        el.child(label).into_any_element()
    }
}
