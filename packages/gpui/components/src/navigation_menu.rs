//! PugNavigationMenu — real GPUI component backed by NavigationMenuSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::NavigationMenuSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI navigation menu component backed by `NavigationMenuSpec`.
///
/// Renders a horizontal nav bar with items, showing active state
/// and optional descriptions for the selected item.
pub struct PugNavigationMenu {
    spec: NavigationMenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugNavigationMenu {
    pub fn new(spec: NavigationMenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-nav".to_string(),
            on_change: None,
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
}

impl IntoElement for PugNavigationMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let gap = theme.resolve_space(self.spec.viewport_gap_token());

        let current_value = self.spec.current_value().map(|s| s.to_string());

        let mut nav_row = div()
            .flex()
            .items_center()
            .gap(px(gap))
            .border_b_1()
            .border_color(border);

        for item in &self.spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(item_id)
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
                    .hover(|s| s.bg(accent.opacity(0.06)));
            }

            tab = tab.child(item.label.clone());
            nav_row = nav_row.child(tab);
        }

        // Show description for active item in a viewport area below
        let mut wrapper = div().flex().flex_col();
        wrapper = wrapper.child(nav_row);

        if let Some(ref current) = current_value {
            if let Some(item) = self.spec.items.iter().find(|i| &i.value == current) {
                if let Some(ref desc) = item.description {
                    wrapper = wrapper.child(
                        div()
                            .p(px(12.0))
                            .text_xs()
                            .text_color(text_secondary)
                            .child(desc.clone()),
                    );
                }
            }
        }

        wrapper.into_any_element()
    }
}
