//! Pagination — real GPUI component backed by PaginationSpec.
//!
//! Contract: button height control-height - 0.125rem, font 0.75rem/600,
//! root gap 0.375rem, page gap 0.25rem, min-width 2.25rem.
//! Focus ring on buttons. Disabled cursor on boundary buttons.
//!
//! Full variant “go to page” and the page-size limit row become interactive when
//! the parent wires the corresponding callbacks (same controlled pattern as
//! `Select`: open state and values come from the spec + handlers).
//! `on_page_change` / `on_page_size_change` pass the page or size by `&usize` so
//! they compose with `cx.listener` in the preview app.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ChoiceOption, ControlDensity, ControlSize, PageItem, PaginationSpec,
    SelectSpec, SemanticControlSizeRole,
};

use super::select::Select;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
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
    /// Contract `.pagination__button` min-width = `var(--poodle-size-control-height)`.
    button_min_width: Pixels,
    font_size: Pixels,
    button_padding: Pixels,
    // Callback
    on_page_change: Option<std::rc::Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
    limit_selector_open: bool,
    on_limit_open_change:
        Option<std::rc::Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_page_size_change: Option<std::rc::Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Pagination {
    type Target = PaginationSpec;
    fn deref(&self) -> &PaginationSpec {
        &self.spec
    }
}

impl Pagination {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(PaginationSpec::new(), theme)
    }

    pub fn from_spec(spec: PaginationSpec, theme: &GpuiThemeProvider) -> Self {
        let surface_fill = resolve_color(theme, spec.button_fill_token());
        let accent_base = resolve_color(theme, spec.current_fill_token());
        let border_default = resolve_color(theme, spec.button_border_token());

        // Svelte: button border = color-mix(border-default 78%, transparent)
        let button_border = Hsla { a: border_default.a * 0.78, ..border_default };
        // Svelte: current fill = color-mix(accent 18%, transparent)
        let current_fill = Hsla { a: accent_base.a * 0.18, ..accent_base };
        // Svelte: current border = color-mix(accent 42%, border-default)
        let current_border = color_mix(accent_base, border_default, 0.42);
        // Svelte: hover fill = color-mix(accent 12%, transparent)
        let hover_fill = Hsla { a: accent_base.a * 0.12, ..accent_base };

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let base_height = resolve_px(theme, "size.control.height");
        // Contract: height = control-height + size offset - 0.125rem
        let button_height =
            base_height + px(rem_to_px(size_height_offset_rem(effective_size))) - px(rem_to_px(0.125));
        // Contract: min-width = control-height (per size, no −0.125rem).
        let button_min_width = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
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
            button_min_width,
            font_size,
            button_padding,
            spec,
            on_page_change: None,
            limit_selector_open: false,
            on_limit_open_change: None,
            on_page_size_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn current_page(mut self, v: usize) -> Self {
        self.spec.current_page = v;
        self
    }
    pub fn total_pages(mut self, v: usize) -> Self {
        self.spec.total_pages = v;
        self
    }
    pub fn sibling_count(mut self, v: usize) -> Self {
        self.spec.sibling_count = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn variant(mut self, v: poodle_specs::PaginationVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn standalone(mut self, v: bool) -> Self {
        self.spec.standalone = Some(v);
        self
    }
    pub fn info_text(mut self, v: impl Into<String>) -> Self {
        self.spec.info_text = Some(v.into());
        self
    }
    pub fn page_size(mut self, v: usize) -> Self {
        self.spec.page_size = Some(v);
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

    pub fn on_page_change(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_change = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn limit_selector_open(mut self, open: bool) -> Self {
        self.limit_selector_open = open;
        self
    }

    pub fn on_limit_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_limit_open_change = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn on_page_size_change(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_size_change = Some(std::rc::Rc::new(handler));
        self
    }

}

mod buttons;

impl IntoElement for Pagination {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        use poodle_specs::PaginationVariant;

        let limit_selector_open = self.limit_selector_open;
        let on_limit_open_change = self.on_limit_open_change.clone();
        let on_page_size_change = self.on_page_size_change.clone();

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

        // Compact mode tightens padding and gap.
        if self.spec.is_compact {
            root = root.gap(gap_sm);
        }

        // Loading state — dim + no interaction.
        if self.spec.is_loading {
            root = root.opacity(self.disabled_opacity);
        }

        // Standalone mode strips the panel chrome.
        if self.spec.resolved_chrome() {
            let pad_x = if self.spec.is_compact {
                resolve_px(theme, "space.inline.sm")
            } else {
                resolve_px(theme, "space.inline.md")
            };
            let pad_y = if self.spec.is_compact {
                resolve_px(theme, "space.inline.xs")
            } else {
                resolve_px(theme, "space.inline.sm")
            };
            root = root
                .bg(surface_bg)
                .border_1()
                .border_color(border_color)
                .rounded(radius_control)
                .px(pad_x)
                .py(pad_y);
        }

        // Info row — Svelte "Showing X to Y of Z". Hidden when total is 0/unknown
        // (info_string returns None). Gated on show_info.
        if self.spec.show_info {
            if let Some(text) = self.spec.info_string() {
                root = root.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(text),
                );
            }
        }

        // Limit selector — contract order: before primary controls. Interactive when
        // parent wires open + page-size callbacks (matches Select state pattern).
        if self.spec.show_limit_selector {
            if let Some(ps) = self.spec.page_size {
                let interactive_limit =
                    on_page_size_change.is_some() && on_limit_open_change.is_some();
                if interactive_limit {
                    let raw = &self.spec.limit_options;
                    let options: Vec<ChoiceOption> = if raw.is_empty() {
                        vec![ChoiceOption::new(ps.to_string(), format!("{ps}"))]
                    } else {
                        raw.iter()
                            .copied()
                            .map(|n| ChoiceOption::new(n.to_string(), format!("{n}")))
                            .collect()
                    };
                    let select_spec = SelectSpec::new(options)
                        .with_value(ps.to_string())
                        .with_placeholder("—")
                        .with_open(limit_selector_open)
                        .with_size(self.spec.size)
                        .with_size_role(self.spec.size_role)
                        .with_density(self.spec.density);

                    let on_toggle_limit = on_limit_open_change.clone();
                    let on_ps = on_page_size_change.clone();
                    let mut limit_select = Select::from_spec(select_spec, theme)
                        .with_id("pagination-limit")
                        .aria_label("Items per page")
                        .on_toggle(move |open, window, cx| {
                            if let Some(ref h) = on_toggle_limit {
                                h(open, window, cx);
                            }
                        })
                        .on_change(move |val, window, cx| {
                            if let Ok(n) = val.parse::<usize>() {
                                if let Some(ref h) = on_ps {
                                    h(&n, window, cx);
                                }
                            }
                        });
                    if self.spec.is_loading {
                        limit_select = limit_select.disabled(true);
                    }

                    root = root.child(
                        div()
                            .flex()
                            .items_center()
                            // Svelte: limit row gap = 0.375rem (space.inline.sm)
                            .gap(gap_sm)
                            .child(
                                div()
                                    .text_size(label_size)
                                    .text_color(text_secondary)
                                    .child("Show"),
                            )
                            .child(limit_select)
                            .child(
                                div()
                                    .text_size(label_size)
                                    .text_color(text_secondary)
                                    .child("per page"),
                            ),
                    );
                } else {
                    root = root.child(
                        div()
                            .text_size(label_size)
                            .text_color(text_secondary)
                            .child(format!("{ps} / page")),
                    );
                }
            }
        }

        let is_full = matches!(self.spec.variant, PaginationVariant::Full);
        let prev_page = current_page.saturating_sub(1).max(1);
        let next_page = (current_page + 1).min(self.spec.total_pages);

        // First button (`««`) — full variant only, when navigation is wired.
        // GPUI's on_page_change is the goToPage analog (contract: shown when
        // controller has goToPage).
        if is_full && self.on_page_change.is_some() {
            root = root.child(self.render_text_nav_button("««", is_first, 1, "poodle-pg-first"));
        }

        // Prev button — chevron for numbered/full, "Prev" text for simple.
        if self.spec.is_simple() {
            root = root.child(self.render_text_nav_button("Prev", is_first, prev_page, "poodle-pg-prev"));
        } else {
            root = root.child(self.render_nav_button("chevron-left", is_first, prev_page, "poodle-pg-prev"));
        }

        // Center content — variant-specific.
        match self.spec.variant {
            PaginationVariant::Numbered => {
                let mut pages_container = div().flex().flex_row().items_center().gap(gap_sm);
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
            }
            PaginationVariant::Full => {
                // Contract: full center summary = "Page X of Y".
                root = root.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(self.spec.full_summary()),
                );
            }
            PaginationVariant::Simple => {
                // Contract: simple center summary = item range "X–Y of Z".
                root = root.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(self.spec.simple_summary()),
                );
            }
        }

        // Next button — chevron for numbered/full, "Next" text for simple.
        if self.spec.is_simple() {
            root = root.child(self.render_text_nav_button("Next", is_last, next_page, "poodle-pg-next"));
        } else {
            root = root.child(self.render_nav_button("chevron-right", is_last, next_page, "poodle-pg-next"));
        }

        // Last button (`»»`) — full variant only, when navigation is wired.
        if is_full && self.on_page_change.is_some() {
            root = root.child(self.render_text_nav_button(
                "»»",
                is_last,
                self.spec.total_pages,
                "poodle-pg-last",
            ));
        }

        root.into_any_element()
    }
}

