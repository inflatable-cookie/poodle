//! Breadcrumbs — real GPUI component backed by BreadcrumbsSpec.
//!
//! Contract: flex-wrap, separator opacity 0.4, body font size,
//! current item in primary color, links in secondary.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BreadcrumbItem, BreadcrumbsSpec, ControlDensity, ControlSize, IconSize, IconSpec,
    SemanticControlSizeRole,
};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::resolve_color;

use super::icon::Icon;

pub struct Breadcrumbs {
    spec: BreadcrumbsSpec,
    theme: poodle_gpui::GpuiThemeProvider,
    text_color: Hsla,
    current_text_color: Hsla,
    separator_color: Hsla,
    hover_color: Hsla,
    gap: Pixels,
    body_size: Pixels,
    on_navigate: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>>,
}

impl std::ops::Deref for Breadcrumbs {
    type Target = BreadcrumbsSpec;
    fn deref(&self) -> &BreadcrumbsSpec {
        &self.spec
    }
}

impl Breadcrumbs {
    pub fn new(items: Vec<BreadcrumbItem>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(BreadcrumbsSpec::new(items), theme)
    }

    pub fn from_spec(spec: BreadcrumbsSpec, theme: &GpuiThemeProvider) -> Self {
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let text_color = resolve_color(theme, spec.text_color_token());
        let current_text_color = resolve_color(theme, spec.current_text_color_token());
        let separator_color = resolve_color(theme, spec.separator_color_token());
        let hover_color = resolve_color(theme, spec.hover_color_token());
        // Svelte: gap is size-based (xs=0.25, sm/md=0.375, lg=0.625, xl=0.75rem)
        let gap = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.25,
            ControlSize::Sm | ControlSize::Md => 0.375,
            ControlSize::Lg => 0.625,
            ControlSize::Xl => 0.75,
        }));
        let body_size = px(rem_to_px(size_font_rem(effective_size)));

        Self {
            spec,
            theme: theme.clone(),
            text_color,
            current_text_color,
            separator_color,
            hover_color,
            gap,
            body_size,
            on_navigate: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<BreadcrumbItem>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_navigate(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_navigate = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Breadcrumbs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = self.theme.clone();
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
            .text_size(self.body_size); // body size per contract

        let on_navigate = self.on_navigate;

        for (i, item) in visible_items.iter().enumerate() {
            if i > 0 {
                // Svelte: separator is chevron-right icon at 0.4 opacity
                container = container.child(
                    div().opacity(0.4).child(
                        Icon::from_spec(
                            IconSpec::new("chevron-right").with_size(IconSize::Sm),
                            &theme,
                        )
                        .with_color(separator_color),
                    ),
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
                let crumb_id = SharedString::from(format!("poodle-crumb-{}", i));
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
