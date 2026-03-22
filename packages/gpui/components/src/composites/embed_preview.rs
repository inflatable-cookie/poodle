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

        // Loading state
        if self.spec.is_loading {
            el = el.child(
                div().flex().items_center().gap(px(8.0)).py(px(8.0))
                    .child(div().text_xs().text_color(desc_color).child("Loading embed..."))
            );
            return el.into_any_element();
        }

        // Empty state — no title, description, or provider
        let has_content = self.spec.title.is_some()
            || self.spec.description.is_some()
            || self.spec.provider.is_some();
        if !has_content {
            el = el.child(
                div().py(px(8.0)).text_xs().text_color(desc_color)
                    .child("No embed data available")
            );
            return el.into_any_element();
        }

        // Normal content rendering
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
