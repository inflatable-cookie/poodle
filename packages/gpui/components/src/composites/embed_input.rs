//! EmbedInput — URL input for embedding external content backed by EmbedInputSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::EmbedInputSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct EmbedInput {
    spec: EmbedInputSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EmbedInput {
    type Target = EmbedInputSpec;
    fn deref(&self) -> &EmbedInputSpec { &self.spec }
}

impl EmbedInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: EmbedInputSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: EmbedInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for EmbedInput {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let placeholder_color = resolve_color(theme, "semantic.color.text.secondary");
        let display = if spec.value.is_empty() { spec.placeholder.as_deref().unwrap_or("Paste URL...") } else { &spec.value };
        let color = if spec.value.is_empty() { placeholder_color } else { text_color };

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .h(px(36.0)).px(px(12.0))
            .flex().items_center()
            .text_sm().text_color(color)
            .child(display.to_string());
        if spec.is_disabled {
            el = el.opacity(resolve_opacity(theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
