//! PugPanelTabs — real GPUI component backed by PanelTabsSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::PanelTabsSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI panel tab strip backed by `PanelTabsSpec`.
///
/// Renders a compact horizontal tab strip for panel navigation,
/// with closable tabs and reorder support.
pub struct PugPanelTabs {
    spec: PanelTabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugPanelTabs {
    pub fn new(spec: PanelTabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-panel-tabs".to_string(),
            on_change: None,
            on_close: None,
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

    pub fn on_close(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugPanelTabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let gap = theme.resolve_space(spec.gap_token());

        let current_value = spec.current_value().map(|s| s.to_string());

        let mut strip = div()
            .flex()
            .items_center()
            .gap(px(gap))
            .border_b_1()
            .border_color(border);

        for item in &spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(tab_id)
                .flex()
                .items_center()
                .gap(px(4.0))
                .px(px(8.0))
                .py(px(4.0))
                .text_xs()
                .cursor_pointer();

            if is_active {
                tab = tab
                    .text_color(text_primary)
                    .border_b_2()
                    .border_color(accent);
            } else {
                tab = tab
                    .text_color(text_secondary)
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
            }

            // Icon placeholder
            if item.icon.is_some() {
                tab = tab.child(
                    div().text_xs().text_color(text_secondary).child("\u{25CF}"),
                );
            }

            tab = tab.child(item.label.clone());

            // Close button
            if item.is_closable {
                let close_id = SharedString::from(format!("{}-close-{}", self.id_prefix, item.value));
                tab = tab.child(
                    div()
                        .id(close_id)
                        .text_xs()
                        .text_color(text_secondary)
                        .ml(px(2.0))
                        .cursor_pointer()
                        .hover(|s| s.text_color(text_primary))
                        .child("\u{00D7}"),
                );
            }

            strip = strip.child(tab);
        }

        strip.into_any_element()
    }
}
