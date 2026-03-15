//! PugSurfaceTabs — real GPUI component backed by SurfaceTabsSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::SurfaceTabsSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI surface-level tab strip backed by `SurfaceTabsSpec`.
///
/// Renders top-level surface tabs (e.g., Browse / Review) with closable
/// tabs and an optional add-tab button.
pub struct PugSurfaceTabs {
    spec: SurfaceTabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_add: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl PugSurfaceTabs {
    pub fn new(spec: SurfaceTabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-surface-tabs".to_string(),
            on_change: None,
            on_close: None,
            on_add: None,
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

    pub fn on_add(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_add = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugSurfaceTabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let bg_surface = resolve_color(theme, "semantic.color.background.surface");
        let gap = theme.resolve_space(spec.gap_token());

        let current_value = spec.current_value().map(|s| s.to_string());

        let mut strip = div()
            .flex()
            .items_end()
            .gap(px(gap))
            .w_full()
            .bg(bg_surface)
            .border_b_1()
            .border_color(border);

        for item in &spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(tab_id)
                .flex()
                .items_center()
                .gap(px(6.0))
                .px(px(12.0))
                .py(px(6.0))
                .text_sm()
                .cursor_pointer();

            if is_active {
                tab = tab
                    .text_color(text_primary)
                    .font_weight(FontWeight::MEDIUM)
                    .border_b_2()
                    .border_color(accent);
            } else {
                tab = tab
                    .text_color(text_secondary)
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
            }

            tab = tab.child(item.label.clone());

            // Close button
            if item.is_closable {
                let close_id =
                    SharedString::from(format!("{}-close-{}", self.id_prefix, item.value));
                tab = tab.child(
                    div()
                        .id(close_id)
                        .text_xs()
                        .text_color(text_secondary)
                        .cursor_pointer()
                        .hover(|s| s.text_color(text_primary))
                        .child("\u{00D7}"),
                );
            }

            strip = strip.child(tab);
        }

        // Add tab button
        if spec.add_enabled {
            let add_id = SharedString::from(format!("{}-add", self.id_prefix));
            strip = strip.child(
                div()
                    .id(add_id)
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(28.0))
                    .h(px(28.0))
                    .my(px(4.0))
                    .rounded(px(4.0))
                    .text_sm()
                    .text_color(text_secondary)
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.06)))
                    .child("+"),
            );
        }

        strip.into_any_element()
    }
}
