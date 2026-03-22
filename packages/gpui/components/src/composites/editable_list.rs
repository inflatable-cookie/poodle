//! EditableList — list with add/remove capabilities.
//! No contract spec exists — implemented from Svelte reference.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

pub struct EditableList {
    theme: GpuiThemeProvider,
    add_label: String,
    is_disabled: bool,
    items: Vec<String>,
    children: Vec<AnyElement>,
}

impl EditableList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            add_label: "Add item".to_string(),
            is_disabled: false,
            items: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn add_label(mut self, v: impl Into<String>) -> Self { self.add_label = v.into(); self }
    pub fn disabled(mut self, v: bool) -> Self { self.is_disabled = v; self }
    pub fn items(mut self, items: Vec<String>) -> Self { self.items = items; self }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for EditableList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let muted_color = resolve_color(theme, "semantic.color.text.muted");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let gap = resolve_px(theme, "semantic.space.stack.sm");
        let border_color = resolve_color(theme, "semantic.color.border.default");

        let mut el = div().flex().flex_col().gap(gap);

        // Item count display
        let total = if !self.items.is_empty() { self.items.len() } else { self.children.len() };
        let counter = div()
            .text_size(px(12.0))
            .text_color(muted_color)
            .child(format!("{} item{}", total, if total == 1 { "" } else { "s" }));
        el = el.child(counter);

        // Render spec items (if provided via .items())
        for item_text in &self.items {
            let remove_icon = Icon::from_spec(
                IconSpec::new("x").with_size(IconSize::Sm),
                theme,
            ).with_color(muted_color);

            let remove_btn = div()
                .cursor_pointer()
                .flex_shrink_0()
                .child(remove_icon);

            let row = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(px(8.0))
                .py(px(4.0))
                .rounded(px(4.0))
                .border_1()
                .border_color(border_color)
                .child(
                    div().text_size(px(14.0)).text_color(text_color).child(item_text.clone()),
                )
                .child(remove_btn);
            el = el.child(row);
        }

        // Render children (custom elements) with remove buttons
        for child in self.children {
            let remove_icon = Icon::from_spec(
                IconSpec::new("x").with_size(IconSize::Sm),
                theme,
            ).with_color(muted_color);

            let remove_btn = div()
                .cursor_pointer()
                .flex_shrink_0()
                .child(remove_icon);

            let row = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(px(8.0))
                .py(px(4.0))
                .rounded(px(4.0))
                .border_1()
                .border_color(border_color)
                .child(div().flex_grow().child(child))
                .child(remove_btn);
            el = el.child(row);
        }

        // Add button
        let add_btn = div().flex().flex_row().items_center().gap(px(4.0))
            .cursor_pointer()
            .child(div().text_size(px(14.0)).text_color(accent).child(format!("+ {}", self.add_label)));
        el = el.child(add_btn);

        if self.is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
