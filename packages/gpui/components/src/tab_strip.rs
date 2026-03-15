//! PugTabStrip — real GPUI component backed by TabStripSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{Orientation, TabStripSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI tab strip component backed by `TabStripSpec`.
///
/// Renders closable file/document tabs (like editor tabs),
/// distinct from content-switching `PugTabs`.
pub struct PugTabStrip {
    spec: TabStripSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugTabStrip {
    pub fn new(spec: TabStripSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-tabstrip".to_string(),
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

impl IntoElement for PugTabStrip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let gap = theme.resolve_space(self.spec.item_gap_token());

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_vertical = self.spec.orientation == Orientation::Vertical;

        let mut strip = div().gap(px(gap));

        if is_vertical {
            strip = strip.flex().flex_col();
        } else {
            strip = strip
                .flex()
                .items_center()
                .border_b_1()
                .border_color(border);
        }

        for item in &self.spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(item_id)
                .flex()
                .items_center()
                .gap(px(6.0))
                .px(px(10.0))
                .py(px(6.0))
                .text_xs();

            if is_active {
                if is_vertical {
                    tab = tab
                        .text_color(accent)
                        .bg(accent.opacity(0.08));
                } else {
                    tab = tab
                        .border_b_1()
                        .border_color(accent);
                }
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

            tab = tab.child(item.label.clone());

            // Close button for closable tabs
            if item.is_closable {
                tab = tab.child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .ml(px(4.0))
                        .child("×"),
                );
            }

            strip = strip.child(tab);
        }

        strip.into_any_element()
    }
}
