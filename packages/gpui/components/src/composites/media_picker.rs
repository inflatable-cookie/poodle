//! MediaPicker — media selection/upload interface backed by MediaPickerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::MediaPickerSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct MediaPicker {
    spec: MediaPickerSpec,
    theme: GpuiThemeProvider,
    content: Option<AnyElement>,
}

impl std::ops::Deref for MediaPicker {
    type Target = MediaPickerSpec;
    fn deref(&self) -> &MediaPickerSpec { &self.spec }
}

impl MediaPicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MediaPickerSpec::new("Select media"), theme: theme.clone(), content: None }
    }
    pub fn from_spec(spec: MediaPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), content: None }
    }
    pub fn with_content(mut self, c: impl IntoElement) -> Self { self.content = Some(c.into_any_element()); self }
}

impl IntoElement for MediaPicker {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let title_color = resolve_color(&self.theme, "semantic.color.text.primary");

        let mut el = div()
            .bg(fill).rounded(radius).shadow_lg()
            .px(px(20.0)).py(px(16.0))
            .flex().flex_col().gap(px(12.0))
            .min_w(px(400.0));
        el = el.child(div().text_color(title_color).font_weight(FontWeight::SEMIBOLD).child(self.spec.title.clone()));
        if let Some(c) = self.content { el = el.child(c); }
        el.into_any_element()
    }
}
