//! PugTabs — real GPUI component backed by TabsSpec.
//!
//! Supports three variants matching the Svelte Tabs component:
//! - Underline (default): bottom border with accent indicator
//! - Card: bordered tabs
//! - Pill: rounded pill container with tinted active state

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{TabsSpec, TabVariant};

use crate::theme_ext::resolve_color;

/// A real GPUI tabs component backed by `TabsSpec`.
pub struct PugTabs {
    spec: TabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Content elements keyed by tab value.
    content: Vec<(String, AnyElement)>,
}

impl PugTabs {
    pub fn new(spec: TabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-tabs".to_string(),
            on_change: None,
            content: Vec::new(),
        }
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Add content for a specific tab value.
    pub fn with_content(mut self, value: impl Into<String>, content: impl IntoElement) -> Self {
        self.content
            .push((value.into(), content.into_any_element()));
        self
    }

    fn render_underline(&self) -> Div {
        let theme = &self.theme;
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let current_value = self.spec.current_value().map(|s| s.to_string());

        let mut tab_row = div()
            .flex()
            .border_b_1()
            .border_color(border);

        for tab_def in &self.spec.tabs {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(px(12.0))
                .py(px(8.0))
                .text_sm();

            if is_active {
                tab = tab
                    .text_color(accent)
                    .border_b_2()
                    .border_color(accent);
            } else {
                tab = tab.text_color(text_secondary);
            }

            if is_disabled {
                tab = tab.opacity(0.48);
            } else {
                tab = tab
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
            }

            tab = tab.child(tab_def.label.clone());
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_pill(&self) -> Div {
        let theme = &self.theme;
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border_subtle = resolve_color(theme, self.spec.list_border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Container border: border-subtle with 68% opacity mix (matching Svelte color-mix)
        let container_border = border_subtle.opacity(border_subtle.a * self.spec.pill_border_opacity());

        // Outer pill container
        let mut tabs = div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .rounded(px(999.0))
            .border_2()
            .border_color(container_border)
            .p(px(3.0));

        for tab_def in &self.spec.tabs {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(px(10.0))
                .py(px(3.0))
                .rounded(px(999.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD);

            if is_active {
                let active_bg = accent.opacity(self.spec.pill_active_bg_opacity());
                tab = tab
                    .bg(active_bg)
                    .text_color(text_primary);
            } else {
                tab = tab.text_color(text_secondary);
            }

            if is_disabled {
                tab = tab.opacity(0.48);
            } else {
                tab = tab.cursor_pointer();
            }

            tab = tab.child(tab_def.label.clone());
            tabs = tabs.child(tab);
        }

        tabs
    }
}

impl IntoElement for PugTabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let tab_row = match self.spec.variant {
            TabVariant::Pill => self.render_pill(),
            _ => self.render_underline(),
        };

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Content pane
        let mut wrapper = div().flex().flex_col().child(tab_row);

        // Show content for active tab
        for (value, content) in self.content {
            if current_value.as_deref() == Some(&value) {
                wrapper = wrapper.child(div().p(px(12.0)).child(content));
                break;
            }
        }

        wrapper.into_any_element()
    }
}
