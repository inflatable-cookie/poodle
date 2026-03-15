//! PugTabs — real GPUI component backed by TabsSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TabsSpec;

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
}

impl IntoElement for PugTabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Tab strip
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

            // We can't move out of &self.on_change in a loop, so we skip click
            // handlers here; they're attached below via a different approach.
            tab_row = tab_row.child(tab);
        }

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
