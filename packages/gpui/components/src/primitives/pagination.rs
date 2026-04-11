//! Pagination — real GPUI component backed by PaginationSpec.
//!
//! Contract: button height control-height - 0.125rem, font 0.75rem/600,
//! root gap 0.375rem, page gap 0.25rem, min-width 2.25rem.
//! Focus ring on buttons. Disabled cursor on boundary buttons.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{ControlDensity, ControlSize, IconSize, IconSpec, PageItem, PaginationSpec, SemanticControlSizeRole};

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem};
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
    font_size: Pixels,
    button_padding: Pixels,
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

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let base_height = resolve_px(theme, "size.control.height");
        // Contract: height = control-height + size offset - 0.125rem
        let button_height = base_height + px(rem_to_px(size_height_offset_rem(effective_size))) - px(2.0);
        let font_size = px(rem_to_px(size_font_rem(effective_size)));
        let base_pad = resolve_px(theme, "space.control.x");
        let button_padding = base_pad + px(rem_to_px(size_padding_x_offset_rem(effective_size)));

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
            font_size,
            button_padding,
            spec,
            on_page_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn current_page(mut self, v: usize) -> Self { self.spec.current_page = v; self }
    pub fn total_pages(mut self, v: usize) -> Self { self.spec.total_pages = v; self }
    pub fn sibling_count(mut self, v: usize) -> Self { self.spec.sibling_count = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn variant(mut self, v: poodle_components::PaginationVariant) -> Self { self.spec.variant = v; self }
    pub fn standalone(mut self, v: bool) -> Self { self.spec.standalone = v; self }
    pub fn info_text(mut self, v: impl Into<String>) -> Self { self.spec.info_text = Some(v.into()); self }
    pub fn page_size(mut self, v: usize) -> Self { self.spec.page_size = Some(v); self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

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
            .px(self.button_padding)
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

    /// Text-label variant of the nav button used by the Simple variant
    /// ("Prev" / "Next" instead of chevron icons).
    fn render_text_nav_button(
        &self,
        label: &'static str,
        disabled: bool,
        target_page: usize,
        id: &str,
    ) -> AnyElement {
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
            .h(button_height)
            .px(self.button_padding)
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_size(crate::theme_ext::resolve_px(&self.theme, "typography.label.size"))
            .text_color(text_color)
            .focus(move |s| s.border_color(focus_ring))
            .when(disabled, |el| {
                el.opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed)
            })
            .when(!disabled, |el| {
                el.cursor_pointer().hover(|style| style.bg(hover_fill))
            })
            .child(label);

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

        let page_id = SharedString::from(format!("poodle-pg-page-{}", page));

        let mut btn = div()
            .id(page_id)
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(36.0)) // 2.25rem
            .h(button_height)
            .px(self.button_padding)
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_color(text_color)
            // Contract: label font per effective size
            .text_size(self.font_size)
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
            .text_size(self.font_size)
            .font_weight(FontWeight::SEMIBOLD)
            .child("...")
            .into_any_element()
    }
}

impl IntoElement for Pagination {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        use poodle_components::PaginationVariant;

        let visible = self.spec.visible_pages();
        let is_first = self.spec.is_first_page();
        let is_last = self.spec.is_last_page();
        let current_page = self.spec.current_page;
        let theme = &self.theme;

        let text_secondary = crate::theme_ext::resolve_color(theme, "color.text.secondary");
        let border_color = crate::theme_ext::resolve_color(theme, "color.border.subtle");
        let surface_bg = crate::theme_ext::resolve_color(theme, "color.background.surface");
        let label_size = crate::theme_ext::resolve_px(theme, "typography.label.size");
        let radius_control = crate::theme_ext::resolve_radius(theme, "radius.control");
        let gap_sm = crate::theme_ext::resolve_px(theme, "space.inline.sm");
        let gap_md = crate::theme_ext::resolve_px(theme, "space.inline.md");

        // Contract: root gap 0.375rem, pages gap 0.25rem
        let mut root = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(gap_md); // 0.375rem

        // Standalone mode strips the panel chrome.
        if !self.spec.standalone {
            root = root
                .bg(surface_bg)
                .border_1()
                .border_color(border_color)
                .rounded(radius_control)
                .px(px(12.0))
                .py(px(8.0));
        }

        // Info text block (shown on simple/full variants).
        if self.spec.info_text.is_some() || self.spec.page_size.is_some() {
            let mut info = div().flex().items_center().gap(gap_md);

            if let Some(ref text) = self.spec.info_text {
                info = info.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(text.clone()),
                );
            }

            if let Some(page_size) = self.spec.page_size {
                info = info.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(format!("{page_size} per page")),
                );
            }

            root = root.child(info);
        }

        // Prev button — label differs between simple and numbered variants.
        let prev_page = if current_page > 1 { current_page - 1 } else { 1 };
        let prev_id = "poodle-pg-prev";
        if self.spec.is_simple() {
            root = root.child(self.render_text_nav_button("Prev", is_first, prev_page, prev_id));
        } else {
            root = root.child(self.render_nav_button("chevron-left", is_first, prev_page, prev_id));
        }

        // Numbered pages only render on Numbered and Full variants.
        if !self.spec.is_simple() {
            let mut pages_container = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(gap_sm); // 0.25rem

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
        } else {
            // Simple variant: show "Page X of Y" text between prev/next.
            root = root.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(format!("Page {} of {}", current_page, self.spec.total_pages)),
            );
        }

        // Next button
        let next_page = if current_page < self.spec.total_pages {
            current_page + 1
        } else {
            self.spec.total_pages
        };
        let next_id = "poodle-pg-next";
        if self.spec.is_simple() {
            root = root.child(self.render_text_nav_button("Next", is_last, next_page, next_id));
        } else {
            root = root.child(self.render_nav_button("chevron-right", is_last, next_page, next_id));
        }

        // Full variant: "Go to page" input text (non-interactive stub for now —
        // matches the visual affordance from Svelte without wiring a handler).
        if matches!(self.spec.variant, PaginationVariant::Full) {
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .gap(gap_md)
                    .child(
                        div()
                            .text_size(label_size)
                            .text_color(text_secondary)
                            .child("Go to"),
                    )
                    .child(
                        div()
                            .w(px(48.0))
                            .h(px(24.0))
                            .px(gap_md)
                            .border_1()
                            .border_color(border_color)
                            .rounded(radius_control)
                            .flex()
                            .items_center()
                            .text_size(label_size)
                            .child(format!("{current_page}")),
                    ),
            );
        }

        root.into_any_element()
    }
}
