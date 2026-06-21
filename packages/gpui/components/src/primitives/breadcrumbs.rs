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

use crate::presentation::{
    breadcrumbs_density_gap_rem, breadcrumbs_font_rem, breadcrumbs_gap_rem, rem_to_px,
    resolve_semantic_size,
};
use crate::theme_ext::resolve_color;
use poodle_specs::BREADCRUMBS_ELLIPSIS_VALUE as ELLIPSIS_VALUE;

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
        // Contract §8: gap is size-driven (xs 0.25, sm 0.375, md space.inline.sm
        // (0.5), lg 0.625, xl 0.75rem), overridden by density when not default
        // (compact 0.25, comfortable 0.75rem).
        let gap_rem = breadcrumbs_density_gap_rem(spec.density)
            .unwrap_or_else(|| breadcrumbs_gap_rem(effective_size));
        let gap = px(rem_to_px(gap_rem));
        // Contract §8 font ladder: md == typography.body.size (0.875rem).
        let body_size = px(rem_to_px(breadcrumbs_font_rem(effective_size)));

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
        // Contract §3 truncation: first + ellipsis + last N-1 items.
        let visible_items = self.spec.visible_items();
        let visible_len = visible_items.len();

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
        let spec = self.spec;

        for (i, item) in visible_items.iter().enumerate() {
            if i > 0 {
                // Contract §2/§9: separator is the chevron-right icon at 0.4 opacity.
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

            let is_ellipsis = item.value == ELLIPSIS_VALUE;
            let is_current = spec.is_current_at(item, i, visible_len);

            if is_current {
                // Contract §8: current is color-only (text-primary), no weight bump.
                container = container.child(
                    div()
                        .text_color(current_text_color)
                        .child(item.label.clone()),
                );
            } else if is_ellipsis {
                // Contract: ellipsis crumb is non-interactive (aria-hidden in Svelte).
                container = container.child(div().text_color(text_color).child(item.label.clone()));
            } else {
                let crumb_id = SharedString::from(format!("poodle-crumb-{}", i));
                let mut item_el = div()
                    .id(crumb_id)
                    .text_color(text_color)
                    .cursor_pointer()
                    .hover(|style| style.text_color(hover_color))
                    .child(item.label.clone());

                // Contract §5: href items navigate via anchor; GPUI has no anchor,
                // so href items emit no callback (router-hook gap). Non-href items
                // fire on_navigate.
                if item.href.is_none() {
                    if let Some(ref handler) = on_navigate {
                        let handler = handler.clone();
                        let value = item.value.clone();
                        item_el = item_el.on_click(move |_event, window, cx| {
                            handler(&value, window, cx);
                        });
                    }
                }

                container = container.child(item_el);
            }
        }

        container.into_any_element()
    }
}
