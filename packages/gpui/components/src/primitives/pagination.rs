use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{PageItem, PaginationSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

pub struct Pagination {
    spec: PaginationSpec,
    // Pre-resolved values
    button_fill: Hsla,
    button_border: Hsla,
    button_text: Hsla,
    current_fill: Hsla,
    current_border: Hsla,
    hover_fill: Hsla,
    ellipsis_color: Hsla,
    disabled_opacity: f32,
    radius: Pixels,
    surface_fill: Hsla,
    accent_base: Hsla,
    border_default: Hsla,
    // Callback
    on_page_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Pagination {
    type Target = PaginationSpec;
    fn deref(&self) -> &PaginationSpec { &self.spec }
}

impl Pagination {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        let spec = PaginationSpec::new();
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

        Self {
            button_fill: surface_fill,
            button_border,
            button_text: resolve_color(theme, spec.button_text_token()),
            current_fill,
            current_border,
            hover_fill,
            ellipsis_color: resolve_color(theme, spec.ellipsis_color_token()),
            disabled_opacity: resolve_opacity(theme, spec.disabled_opacity_token()),
            radius: resolve_radius(theme, spec.radius_token()),
            surface_fill,
            accent_base,
            border_default,
            spec,
            on_page_change: None,
        }
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

        Self {
            button_fill: surface_fill,
            button_border,
            button_text: resolve_color(theme, spec.button_text_token()),
            current_fill,
            current_border,
            hover_fill,
            ellipsis_color: resolve_color(theme, spec.ellipsis_color_token()),
            disabled_opacity: resolve_opacity(theme, spec.disabled_opacity_token()),
            radius: resolve_radius(theme, spec.radius_token()),
            surface_fill,
            accent_base,
            border_default,
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
        self.on_page_change = Some(Box::new(handler));
        self
    }

    fn render_nav_button(
        &self,
        label: &str,
        disabled: bool,
        target_page: usize,
    ) -> AnyElement {
        let fill = self.button_fill;
        let border = self.button_border;
        let text_color = self.button_text;
        let hover_fill = self.hover_fill;
        let radius = self.radius;
        let disabled_opacity = self.disabled_opacity;

        div()
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(36.0))
            .h(px(34.0))
            .px(px(12.0))
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_color(text_color)
            .when(disabled, |el| {
                el.opacity(disabled_opacity)
                    .cursor(CursorStyle::default())
            })
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(|style| style.bg(hover_fill))
            })
            .child(label.to_string())
            .into_any_element()
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
        let radius = self.radius;

        div()
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(36.0))
            .h(px(34.0))
            .px(px(12.0))
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_color(text_color)
            .when(is_current, |el| {
                el.font_weight(FontWeight::SEMIBOLD)
            })
            .when(!is_current, |el| {
                el.cursor_pointer()
                    .hover(|style| style.bg(hover_fill))
            })
            .child(page.to_string())
            .into_any_element()
    }

    fn render_ellipsis(&self) -> AnyElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(36.0))
            .h(px(34.0))
            .text_color(self.ellipsis_color)
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
        let page_gap = 4.0_f32;
        let nav_gap = 6.0_f32;
        let current_page = self.spec.current_page;

        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(nav_gap));

        // Prev button
        let prev_page = if current_page > 1 {
            current_page - 1
        } else {
            1
        };
        root = root.child(self.render_nav_button("\u{2039}", is_first, prev_page));

        // Page buttons container
        let mut pages_container = div().flex().flex_row().items_center().gap(px(page_gap));

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
        root = root.child(self.render_nav_button("\u{203A}", is_last, next_page));

        root.into_any_element()
    }
}
