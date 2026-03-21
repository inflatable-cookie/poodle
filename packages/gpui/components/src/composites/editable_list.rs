//! EditableList — list with add/remove capabilities.
//! No contract spec exists — implemented from Svelte reference.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

pub struct EditableList {
    theme: GpuiThemeProvider,
    add_label: String,
    is_disabled: bool,
    children: Vec<AnyElement>,
}

impl EditableList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            add_label: "Add item".to_string(),
            is_disabled: false,
            children: Vec::new(),
        }
    }
    pub fn add_label(mut self, v: impl Into<String>) -> Self { self.add_label = v.into(); self }
    pub fn disabled(mut self, v: bool) -> Self { self.is_disabled = v; self }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for EditableList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let accent = resolve_color(&self.theme, "semantic.color.accent.base");
        let gap = resolve_px(&self.theme, "semantic.space.stack.sm");

        let mut el = div().flex().flex_col().gap(gap);
        for child in self.children {
            el = el.child(child);
        }
        let add_btn = div().flex().flex_row().items_center().gap(px(4.0))
            .cursor_pointer()
            .child(div().text_sm().text_color(accent).child(format!("+ {}", self.add_label)));
        el = el.child(add_btn);
        if self.is_disabled {
            let opacity = resolve_opacity(&self.theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
