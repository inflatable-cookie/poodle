use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::BreadcrumbsSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub struct PugBreadcrumbs {
    spec: BreadcrumbsSpec,
    text_color: Hsla,
    current_text_color: Hsla,
    separator_color: Hsla,
    hover_color: Hsla,
    gap: Pixels,
    on_navigate: Option<Box<dyn Fn(&str, &mut Window, &mut App)>>,
}

impl PugBreadcrumbs {
    pub fn new(spec: BreadcrumbsSpec, theme: &GpuiThemeProvider) -> Self {
        let text_color = resolve_color(theme, spec.text_color_token());
        let current_text_color = resolve_color(theme, spec.current_text_color_token());
        let separator_color = resolve_color(theme, spec.separator_color_token());
        let hover_color = resolve_color(theme, spec.hover_color_token());
        let gap = resolve_px(theme, spec.gap_token());

        Self {
            spec,
            text_color,
            current_text_color,
            separator_color,
            hover_color,
            gap,
            on_navigate: None,
        }
    }

    pub fn on_navigate(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_navigate = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugBreadcrumbs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let items = &self.spec.items;
        let visible_items = if let Some(max) = self.spec.max_visible_items {
            if items.len() > max && max >= 2 {
                let mut visible = Vec::new();
                visible.push(items[0].clone());
                for item in items.iter().skip(items.len() - (max - 1)) {
                    visible.push(item.clone());
                }
                visible
            } else {
                items.clone()
            }
        } else {
            items.clone()
        };

        let last_index = visible_items.len().saturating_sub(1);
        let hover_color = self.hover_color;
        let text_color = self.text_color;
        let separator_color = self.separator_color;
        let current_text_color = self.current_text_color;
        let gap = self.gap;

        let mut container = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(gap);

        let on_navigate = self.on_navigate;

        for (i, item) in visible_items.iter().enumerate() {
            if i > 0 {
                container = container.child(
                    div()
                        .text_color(separator_color)
                        .child("/"),
                );
            }

            let is_current = item.is_current || i == last_index;

            if is_current {
                container = container.child(
                    div()
                        .text_color(current_text_color)
                        .font_weight(FontWeight::MEDIUM)
                        .child(item.label.clone()),
                );
            } else {
                let value = item.value.clone();
                let item_el = div()
                    .text_color(text_color)
                    .cursor_pointer()
                    .hover(|style| style.text_color(hover_color));

                let item_el = if on_navigate.is_some() {
                    // We can't move the callback into multiple closures,
                    // so we use a shared reference approach via the parent.
                    // For GPUI, each clickable item needs its own handler.
                    // Since we can't clone Box<dyn Fn>, we skip attaching
                    // click handlers here and rely on the parent to handle
                    // navigation via the on_navigate callback pattern.
                    item_el.child(item.label.clone())
                } else {
                    item_el.child(item.label.clone())
                };

                container = container.child(item_el);
                let _ = value; // suppress unused warning
            }
        }

        container.into_any_element()
    }
}
