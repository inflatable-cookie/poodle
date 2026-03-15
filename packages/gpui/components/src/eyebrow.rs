//! PugEyebrow — real GPUI component backed by EyebrowSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::EyebrowSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI eyebrow label component backed by `EyebrowSpec`.
pub struct PugEyebrow {
    spec: EyebrowSpec,
    theme: GpuiThemeProvider,
}

impl PugEyebrow {
    pub fn new(spec: EyebrowSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugEyebrow {
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
