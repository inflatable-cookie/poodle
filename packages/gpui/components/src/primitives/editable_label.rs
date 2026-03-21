//! EditableLabel — real GPUI component backed by EditableLabelSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::EditableLabelSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI editable label component backed by `EditableLabelSpec`.
pub struct EditableLabel {
    spec: EditableLabelSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EditableLabel {
    type Target = EditableLabelSpec;
    fn deref(&self) -> &EditableLabelSpec { &self.spec }
}

impl EditableLabel {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: EditableLabelSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: EditableLabelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = v.into(); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
    pub fn editing(mut self, v: bool) -> Self { self.spec.is_editing = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }

}

impl IntoElement for EditableLabel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_color = resolve_color(theme, spec.text_color_token());
        let border_color = resolve_color(theme, spec.edit_border_token());
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let display_text = if spec.value.is_empty() {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            spec.value.clone()
        };

        let text_opacity = if spec.value.is_empty() { 0.5 } else { 1.0 };

        let mut el = div()
            .px(px(4.0))
            .py(px(2.0))
            .text_sm()
            .text_color(text_color)
            .child(
                div()
                    .opacity(text_opacity as f32)
                    .child(display_text),
            );

        if spec.is_editing {
            el = el
                .border_1()
                .border_color(border_color)
                .rounded(px(4.0));
        } else {
            el = el
                .border_1()
                .border_color(gpui::transparent_black());
        }

        if spec.is_disabled {
            el = el.opacity(disabled_opacity);
        }

        el.into_any_element()
    }
}
