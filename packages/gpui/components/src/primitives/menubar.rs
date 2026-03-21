//! Menubar — real GPUI component backed by MenubarSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{MenuSpec, MenubarEntry, MenubarSpec};

use super::menu::Menu;
use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI menubar component backed by `MenubarSpec`.
///
/// Renders a horizontal bar of menu triggers. When a trigger is active,
/// shows the associated dropdown menu below it.
pub struct Menubar {
    spec: MenubarSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Menubar {
    type Target = MenubarSpec;
    fn deref(&self) -> &MenubarSpec { &self.spec }
}

impl Menubar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MenubarSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_select: None }
    }

    pub fn from_spec(spec: MenubarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-menubar".to_string(),
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<MenubarEntry>) -> Self { self.spec.items = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Menubar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
        let gap = theme.resolve_space(self.spec.trigger_gap_token());

        let current_value = self.spec.current_value().map(|s| s.to_string());

        let mut trigger_row = div()
            .flex()
            .items_center()
            .gap(px(gap));

        for entry in &self.spec.items {
            let is_active = current_value.as_deref() == Some(entry.value.as_str());
            let is_disabled = entry.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, entry.value));

            let mut trigger = div()
                .id(item_id)
                .px(px(8.0))
                .py(px(4.0))
                .rounded(px(4.0))
                .text_sm();

            if is_active {
                trigger = trigger
                    .text_color(accent)
                    .bg(accent.opacity(0.08));
            } else {
                trigger = trigger.text_color(text_secondary);
            }

            if is_disabled {
                trigger = trigger.opacity(disabled_opacity);
            } else {
                trigger = trigger
                    .cursor_pointer()
                    .hover(|s| s.bg(hover_bg));
            }

            trigger = trigger.child(entry.label.clone());
            trigger_row = trigger_row.child(trigger);
        }

        let mut wrapper = div().flex().flex_col();
        wrapper = wrapper.child(trigger_row);

        // Show dropdown menu for active entry
        if let Some(entry) = self.spec.current_menu() {
            if !entry.items.is_empty() {
                let menu_spec = MenuSpec::new(entry.items.clone());
                wrapper = wrapper.child(
                    Menu::from_spec(menu_spec, &self.theme)
                        .with_id(format!("{}-dropdown", self.id_prefix)),
                );
            }
        }

        wrapper.into_any_element()
    }
}
