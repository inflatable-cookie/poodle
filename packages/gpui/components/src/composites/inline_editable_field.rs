//! InlineEditableField — click-to-edit field backed by InlineEditableFieldSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::InlineEditableFieldSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct InlineEditableField {
    spec: InlineEditableFieldSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for InlineEditableField {
    type Target = InlineEditableFieldSpec;
    fn deref(&self) -> &InlineEditableFieldSpec { &self.spec }
}

impl InlineEditableField {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: InlineEditableFieldSpec::new(""), theme: theme.clone() }
    }
    pub fn from_spec(spec: InlineEditableFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for InlineEditableField {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let text_color = resolve_color(theme, spec.text_color_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let display = if spec.value.is_empty() { spec.placeholder.as_deref().unwrap_or("") } else { &spec.value };
        let placeholder_color = resolve_color(theme, "semantic.color.text.secondary");
        let display_color = if spec.value.is_empty() { placeholder_color } else { text_color };

        let mut el = div()
            .text_size(px(14.0)).text_color(display_color)
            .child(display.to_string());

        if spec.is_editing {
            el = el.border_1().border_color(border).rounded(radius).px(px(8.0)).py(px(4.0));
        }
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
