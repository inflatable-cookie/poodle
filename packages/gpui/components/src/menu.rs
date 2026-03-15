//! PugMenu — real GPUI component backed by MenuSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{MenuItemKind, MenuSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI menu component backed by `MenuSpec`.
pub struct PugMenu {
    spec: MenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    selected_value: Option<String>,
}

impl PugMenu {
    pub fn new(spec: MenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-menu".to_string(),
            selected_value: None,
        }
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    /// Highlight a specific item as selected/active.
    pub fn with_selected(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }
}

impl IntoElement for PugMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let surface_bg = resolve_color(theme, self.spec.surface_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut menu = div()
            .w(px(180.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .shadow_md()
            .py(px(4.0));

        for item in &self.spec.items {
            // Separator
            if item.kind == MenuItemKind::Separator {
                menu = menu.child(
                    div()
                        .h(px(1.0))
                        .mx(px(4.0))
                        .my(px(2.0))
                        .bg(border),
                );
                continue;
            }

            let is_active = self
                .selected_value
                .as_deref()
                .map(|s| s == item.value)
                .unwrap_or(false);
            let is_disabled = item.is_disabled;
            let is_checked = item.is_checked;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut row = div()
                .id(item_id)
                .px(px(10.0))
                .py(px(6.0))
                .text_sm()
                .flex()
                .items_center()
                .justify_between();

            if is_active {
                row = row
                    .bg(accent.opacity(0.1))
                    .text_color(accent);
            } else if is_disabled {
                row = row.text_color(text_secondary).opacity(0.48);
            } else {
                row = row
                    .cursor_pointer()
                    .hover(|s| s.bg(accent.opacity(0.08)));
            }

            // Label + checkbox indicator
            let mut label_row = div().flex().items_center().gap(px(6.0));
            if is_checked {
                label_row = label_row.child(
                    div().text_xs().text_color(accent).child("✓"),
                );
            }
            label_row = label_row.child(item.label.clone());
            row = row.child(label_row);

            // Shortcut hint
            if let Some(ref shortcut) = item.shortcut_label {
                row = row.child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .child(shortcut.clone()),
                );
            }

            menu = menu.child(row);
        }

        menu.into_any_element()
    }
}
