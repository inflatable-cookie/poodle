//! Breadcrumbs — real GPUI component backed by BreadcrumbsSpec.
//!
//! Contract: flex-wrap, separator opacity 0.4, body font size,
//! current item in primary color, links in secondary.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{BreadcrumbItem, BreadcrumbsSpec};

use crate::theme_ext::{resolve_color, resolve_px};

pub struct Breadcrumbs {
    spec: BreadcrumbsSpec,
    text_color: Hsla,
    current_text_color: Hsla,
    separator_color: Hsla,
    hover_color: Hsla,
    gap: Pixels,
    on_navigate: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>>,
}

impl std::ops::Deref for Breadcrumbs {
    type Target = BreadcrumbsSpec;
    fn deref(&self) -> &BreadcrumbsSpec { &self.spec }
}

impl Breadcrumbs {
    pub fn new(items: Vec<BreadcrumbItem>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(BreadcrumbsSpec::new(items), theme)
    }

    pub fn from_spec(spec: BreadcrumbsSpec, theme: &GpuiThemeProvider) -> Self {
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

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<BreadcrumbItem>) -> Self { self.spec.items = v; self }

    pub fn on_navigate(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_navigate = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Breadcrumbs {
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

        // Contract: flex-wrap, body font size
        let mut container = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(gap)
            .text_size(px(14.0)); // 0.875rem — body size per contract

        let on_navigate = self.on_navigate;

        for (i, item) in visible_items.iter().enumerate() {
            if i > 0 {
                // Contract: separator opacity 0.4
                container = container.child(
                    div()
                        .text_color(separator_color)
                        .opacity(0.4)
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
                let crumb_id = SharedString::from(format!("pug-crumb-{}", i));
                let mut item_el = div()
                    .id(crumb_id)
                    .text_color(text_color)
                    .cursor_pointer()
                    .hover(|style| style.text_color(hover_color))
                    .child(item.label.clone());

                if let Some(ref handler) = on_navigate {
                    let handler = handler.clone();
                    let value = item.value.clone();
                    item_el = item_el.on_click(move |_event, window, cx| {
                        handler(&value, window, cx);
                    });
                }

                container = container.child(item_el);
            }
        }

        container.into_any_element()
    }
}
