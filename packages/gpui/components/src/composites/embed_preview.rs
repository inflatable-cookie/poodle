//! EmbedPreview — preview of embedded content backed by EmbedPreviewSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::EmbedPreviewSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct EmbedPreview {
    spec: EmbedPreviewSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EmbedPreview {
    type Target = EmbedPreviewSpec;
    fn deref(&self) -> &EmbedPreviewSpec { &self.spec }
}

impl EmbedPreview {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: EmbedPreviewSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: EmbedPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for EmbedPreview {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let title_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let desc_color = resolve_color(&self.theme, "semantic.color.text.secondary");

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .px(px(16.0)).py(px(12.0))
            .flex().flex_col().gap(px(4.0));
        if let Some(ref title) = self.spec.title {
            el = el.child(div().text_size(px(14.0)).text_color(title_color).font_weight(FontWeight::MEDIUM).child(title.clone()));
        }
        if let Some(ref desc) = self.spec.description {
            el = el.child(div().text_xs().text_color(desc_color).child(desc.clone()));
        }
        if let Some(ref provider) = self.spec.provider {
            el = el.child(div().text_xs().text_color(desc_color).child(provider.clone()));
        }
        el.into_any_element()
    }
}
