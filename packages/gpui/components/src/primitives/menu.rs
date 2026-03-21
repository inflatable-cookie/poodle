//! Menu — real GPUI component backed by MenuSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{MenuEntry, MenuItemKind, MenuSpec, OverlayPlacement};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI menu component backed by `MenuSpec`.
pub struct Menu {
    spec: MenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    selected_value: Option<String>,
}

impl std::ops::Deref for Menu {
    type Target = MenuSpec;
    fn deref(&self) -> &MenuSpec { &self.spec }
}

impl Menu {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MenuSpec::default(), theme: theme.clone(), id_prefix: String::new(), selected_value: None }
    }

    pub fn from_spec(spec: MenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-menu".to_string(),
            selected_value: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<MenuEntry>) -> Self { self.spec.items = v; self }
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


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

impl IntoElement for Menu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let surface_bg = resolve_color(theme, self.spec.surface_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let mut menu = div()
            .w(px(180.0))
            .rounded(control_radius)
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
                row = row.text_color(text_secondary).opacity(disabled_opacity);
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
