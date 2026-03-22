//! FormLayout — form layout container with columns and validation display.
//! No contract spec — implemented from Svelte reference.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use crate::theme_ext::{resolve_color, resolve_px};

pub struct FormLayout {
    theme: GpuiThemeProvider,
    description: Option<String>,
    error: Option<String>,
    success: Option<String>,
    children: Vec<AnyElement>,
    actions: Option<AnyElement>,
}

impl FormLayout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            description: None,
            error: None,
            success: None,
            children: Vec::new(),
            actions: None,
        }
    }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn error(mut self, v: impl Into<String>) -> Self { self.error = Some(v.into()); self }
    pub fn success(mut self, v: impl Into<String>) -> Self { self.success = Some(v.into()); self }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element()); self
    }
}

impl IntoElement for FormLayout {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let desc_color = resolve_color(&self.theme, "semantic.color.text.secondary");
        let error_color = resolve_color(&self.theme, "semantic.color.status.danger");
        let success_color = resolve_color(&self.theme, "semantic.color.status.success");
        let gap = resolve_px(&self.theme, "semantic.space.stack.md");

        let mut el = div().flex().flex_col().gap(gap);

        if let Some(ref desc) = self.description {
            el = el.child(div().text_size(px(14.0)).text_color(desc_color).child(desc.clone()));
        }
        if let Some(ref error) = self.error {
            el = el.child(div().text_size(px(14.0)).text_color(error_color).child(error.clone()));
        }
        if let Some(ref success) = self.success {
            el = el.child(div().text_size(px(14.0)).text_color(success_color).child(success.clone()));
        }

        // Form fields
        let mut fields = div().flex().flex_col().gap(gap);
        for child in self.children {
            fields = fields.child(child);
        }
        el = el.child(fields);

        if let Some(actions) = self.actions {
            el = el.child(actions);
        }

        el.into_any_element()
    }
}
