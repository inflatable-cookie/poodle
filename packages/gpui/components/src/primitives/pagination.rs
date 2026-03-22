//! Pagination — real GPUI component backed by PaginationSpec.
//!
//! Contract: button height control-height - 0.125rem, font 0.75rem/600,
//! root gap 0.375rem, page gap 0.25rem, min-width 2.25rem.
//! Focus ring on buttons. Disabled cursor on boundary buttons.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, PageItem, PaginationSpec};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct Pagination {
    spec: PaginationSpec,
    theme: GpuiThemeProvider,
    // Pre-resolved values
    button_fill: Hsla,
    button_border: Hsla,
    button_text: Hsla,
    current_fill: Hsla,
    current_border: Hsla,
    hover_fill: Hsla,
    ellipsis_color: Hsla,
    focus_ring: Hsla,
    disabled_opacity: f32,
    radius: Pixels,
    button_height: Pixels,
    // Callback
    on_page_change: Option<std::rc::Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Pagination {
    type Target = PaginationSpec;
    fn deref(&self) -> &PaginationSpec { &self.spec }
}

impl Pagination {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(PaginationSpec::new(), theme)
    }

    pub fn from_spec(spec: PaginationSpec, theme: &GpuiThemeProvider) -> Self {
        let surface_fill = resolve_color(theme, spec.button_fill_token());
        let accent_base = resolve_color(theme, spec.current_fill_token());
        let border_default = resolve_color(theme, spec.button_border_token());

        // Contract: 78% border-default mix
        let button_border = color_mix(border_default, surface_fill, 0.22);
        // Contract: 18% accent mix with surface for current fill
        let current_fill = color_mix(accent_base, surface_fill, 0.82);
        // Contract: 42% accent with border-default for current border
        let current_border = color_mix(accent_base, border_default, 0.58);
        // Contract: 12% accent mix for hover
        let hover_fill = color_mix(accent_base, surface_fill, 0.88);

        // Contract: height = control-height - 0.125rem
        let control_height = resolve_px(theme, "semantic.size.control.height");
        let button_height = control_height - px(2.0);

        Self {
            theme: theme.clone(),
            button_fill: surface_fill,
            button_border,
            button_text: resolve_color(theme, spec.button_text_token()),
            current_fill,
            current_border,
            hover_fill,
            ellipsis_color: resolve_color(theme, spec.ellipsis_color_token()),
            focus_ring: resolve_color(theme, spec.focus_ring_color_token()),
            disabled_opacity: resolve_opacity(theme, spec.disabled_opacity_token()),
            radius: resolve_radius(theme, spec.radius_token()),
            button_height,
            spec,
            on_page_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn current_page(mut self, v: usize) -> Self { self.spec.current_page = v; self }
    pub fn total_pages(mut self, v: usize) -> Self { self.spec.total_pages = v; self }
    pub fn sibling_count(mut self, v: usize) -> Self { self.spec.sibling_count = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    pub fn on_page_change(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_change = Some(std::rc::Rc::new(handler));
        self
    }

    fn render_nav_button(
        &self,
        icon_name: &str,
        disabled: bool,
        target_page: usize,
        id: &str,
    ) -> AnyElement {
        let theme = &self.theme;
        let fill = self.button_fill;
        let border = self.button_border;
        let text_color = self.button_text;
        let hover_fill = self.hover_fill;
        let focus_ring = self.focus_ring;
        let radius = self.radius;
        let button_height = self.button_height;
        let disabled_opacity = self.disabled_opacity;

        let mut btn = div()
            .id(SharedString::from(id.to_string()))
            .focusable()
            .flex()
            .items_center()
            .justify_center()
            // Contract: min-width 2.25rem
            .min_w(px(36.0))
            .h(button_height)
            .px(px(8.0)) // 0.5rem
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            // Focus ring
            .focus(move |s| s.border_color(focus_ring))
            .when(disabled, |el| {
                el.opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed)
            })
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(|style| style.bg(hover_fill))
            })
            .child(
                Icon::from_spec(
                    IconSpec::new(icon_name).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_color),
            );

        // Wire click handler for navigation
        if !disabled {
            if let Some(ref handler) = self.on_page_change {
                let handler = handler.clone();
                btn = btn.on_click(move |_event, window, cx| {
                    handler(target_page, window, cx);
                });
            }
        }

        btn.into_any_element()
    }

    fn render_page_button(&self, page: usize) -> AnyElement {
        let is_current = page == self.spec.current_page;
        let fill = if is_current {
            self.current_fill
        } else {
            self.button_fill
        };
        let border = if is_current {
            self.current_border
        } else {
            self.button_border
        };
        let text_color = self.button_text;
        let hover_fill = self.hover_fill;
        let focus_ring = self.focus_ring;
        let radius = self.radius;
        let button_height = self.button_height;

        let page_id = SharedString::from(format!("pug-pg-page-{}", page));

        let mut btn = div()
            .id(page_id)
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(36.0)) // 2.25rem
            .h(button_height)
            .px(px(12.0)) // 0.75rem
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_color(text_color)
            // Contract: label font 0.75rem / 600
            .text_size(px(12.0))
            .font_weight(FontWeight::SEMIBOLD)
            // Focus ring
            .focus(move |s| s.border_color(focus_ring))
            .when(is_current, |el| {
                el.font_weight(FontWeight::BOLD)
            })
            .when(!is_current, |el| {
                el.cursor_pointer()
                    .hover(|style| style.bg(hover_fill))
            })
            .child(page.to_string());

        // Wire click handler for page selection
        if !is_current {
            if let Some(ref handler) = self.on_page_change {
                let handler = handler.clone();
                btn = btn.on_click(move |_event, window, cx| {
                    handler(page, window, cx);
                });
            }
        }

        btn.into_any_element()
    }

    fn render_ellipsis(&self) -> AnyElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            // Contract: ellipsis min-width 1.5rem
            .min_w(px(24.0))
            .h(self.button_height)
            .text_color(self.ellipsis_color)
            // Contract: same font as buttons
            .text_size(px(12.0))
            .font_weight(FontWeight::SEMIBOLD)
            .child("...")
            .into_any_element()
    }
}

impl IntoElement for Pagination {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let visible = self.spec.visible_pages();
        let is_first = self.spec.is_first_page();
        let is_last = self.spec.is_last_page();
        let current_page = self.spec.current_page;

        // Contract: root gap 0.375rem, pages gap 0.25rem
        let mut root = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(6.0)); // 0.375rem

        // Prev button
        let prev_page = if current_page > 1 { current_page - 1 } else { 1 };
        root = root.child(self.render_nav_button("chevron-left", is_first, prev_page, "pug-pg-prev"));

        // Page buttons container
        let mut pages_container = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0)); // 0.25rem

        for item in &visible {
            match item {
                PageItem::Page(page) => {
                    pages_container = pages_container.child(self.render_page_button(*page));
                }
                PageItem::Ellipsis => {
                    pages_container = pages_container.child(self.render_ellipsis());
                }
            }
        }

        root = root.child(pages_container);

        // Next button
        let next_page = if current_page < self.spec.total_pages {
            current_page + 1
        } else {
            self.spec.total_pages
        };
        root = root.child(self.render_nav_button("chevron-right", is_last, next_page, "pug-pg-next"));

        root.into_any_element()
    }
}
