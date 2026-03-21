//! Eyebrow — real GPUI component backed by EyebrowSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::EyebrowSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI eyebrow label component backed by `EyebrowSpec`.
pub struct Eyebrow {
    spec: EyebrowSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Eyebrow {
    type Target = EyebrowSpec;
    fn deref(&self) -> &EyebrowSpec { &self.spec }
}

impl Eyebrow {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: EyebrowSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: EyebrowSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = Some(v.into()); self }

}

impl IntoElement for Eyebrow {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_color = resolve_color(theme, spec.text_color_token());
        let label = spec.content.clone().unwrap_or_default();

        div()
            .text_xs()
            .text_color(text_color)
            .font_weight(FontWeight::SEMIBOLD)
            .child(label)
            .into_any_element()
    }
}
