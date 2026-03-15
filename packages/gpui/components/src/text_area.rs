//! PugTextArea — real GPUI component backed by TextAreaSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TextAreaSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI text area component backed by `TextAreaSpec`.
pub struct PugTextArea {
    spec: TextAreaSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl PugTextArea {
    pub fn new(spec: TextAreaSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }
}

impl IntoElement for PugTextArea {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let surface_bg = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let value = spec.current_value();
        let is_empty = value.is_empty();
        let display_text = if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            value.to_string()
        };
        let text_col = if is_empty { text_secondary } else { text_primary };

        let row_height = spec.rows as f32 * 20.0;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-textarea-{}", suffix)
        } else {
            "pug-textarea".to_string()
        };

        let mut el = div()
            .id(SharedString::from(id_str))
            .min_h(px(row_height))
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .text_sm()
            .text_color(text_col)
            .child(display_text);

        if spec.is_disabled {
            el = el.opacity(0.48);
        }

        el.into_any_element()
    }
}
