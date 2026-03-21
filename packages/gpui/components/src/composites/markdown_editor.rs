//! MarkdownEditor — markdown editing with preview backed by MarkdownEditorSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::MarkdownEditorSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct MarkdownEditor {
    spec: MarkdownEditorSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for MarkdownEditor {
    type Target = MarkdownEditorSpec;
    fn deref(&self) -> &MarkdownEditorSpec { &self.spec }
}

impl MarkdownEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MarkdownEditorSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: MarkdownEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for MarkdownEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let toolbar_fill = resolve_color(&self.theme, self.spec.toolbar_fill_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let muted = resolve_color(&self.theme, "semantic.color.text.secondary");

        let display = if self.spec.value.is_empty() { self.spec.placeholder.as_deref().unwrap_or("Type here...") } else { &self.spec.value };
        let color = if self.spec.value.is_empty() { muted } else { text_color };

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .flex().flex_col().min_h(px(200.0));
        // Toolbar
        el = el.child(div().bg(toolbar_fill).px(px(8.0)).py(px(4.0))
            .flex().flex_row().gap(px(4.0))
            .child(div().text_xs().text_color(muted).child("B"))
            .child(div().text_xs().text_color(muted).child("I"))
            .child(div().text_xs().text_color(muted).child("H")));
        // Editor area
        el = el.child(div().px(px(12.0)).py(px(8.0)).flex_grow().text_sm().text_color(color).child(display.to_string()));

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(&self.theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
