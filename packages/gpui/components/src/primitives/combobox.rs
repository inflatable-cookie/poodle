//! Combobox — real GPUI component backed by ComboboxSpec.
//!
//! Contract: grid root min-width 14rem, input with focus ring,
//! absolutely positioned list with overlay shadow.
//! Option padding 0.375rem 0.5rem, radius control-0.125rem.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{ComboboxOption, ComboboxSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI combobox component backed by `ComboboxSpec`.
pub struct Combobox {
    spec: ComboboxSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_query_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Combobox {
    type Target = ComboboxSpec;
    fn deref(&self) -> &ComboboxSpec { &self.spec }
}

impl Combobox {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ComboboxSpec::new(), theme: theme.clone(), id_suffix: None, on_change: None, on_query_change: None }
    }

    pub fn from_spec(spec: ComboboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_query_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn options(mut self, v: Vec<ComboboxOption>) -> Self { self.spec.options = v; self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn query(mut self, v: impl Into<String>) -> Self { self.spec.query = v.into(); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn on_query_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_query_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Combobox {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve input tokens ────────────────────────────────────
        let input_fill = resolve_color(theme, spec.input_fill_token());
        let input_border = resolve_color(theme, spec.input_border_token());
        let input_text = resolve_color(theme, spec.input_text_token());
        let input_placeholder = resolve_color(theme, spec.input_placeholder_token());
        let input_radius = resolve_radius(theme, spec.input_radius_token());
        let input_height = resolve_px(theme, spec.input_height_token());
        let input_padding_x = resolve_px(theme, "semantic.space.control.x");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Resolve list tokens ─────────────────────────────────────
        let elevated = resolve_color(theme, spec.list_fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let list_fill = color_mix(elevated, panel, 0.98);
        let list_border_raw = resolve_color(theme, spec.list_border_token());
        let list_border = color_mix(list_border_raw, panel, 0.72);
        let list_radius = resolve_radius(theme, spec.list_radius_token());
        let option_text = resolve_color(theme, spec.option_text_token());
        let accent = resolve_color(theme, spec.option_highlight_token());
        let option_highlight = color_mix(accent, panel, 0.16);
        let option_desc_color = resolve_color(theme, spec.option_description_token());
        let empty_text = resolve_color(theme, spec.empty_text_token());

        // Contract: option radius = control - 0.125rem
        let option_radius = input_radius - px(2.0);

        let is_disabled = spec.is_disabled;
        let is_open = spec.is_open;
        let current_value = spec.current_value().map(|s| s.to_string());

        // ── Input display text ──────────────────────────────────────
        let display_text = if !spec.query.is_empty() {
            spec.query.clone()
        } else if let Some(label) = spec.selected_label() {
            label.to_string()
        } else {
            String::new()
        };
        let show_placeholder = display_text.is_empty();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-combobox-{}", suffix)
        } else {
            "pug-combobox".to_string()
        };

        // ── Input field ─────────────────────────────────────────────
        let input_el = {
            let text_el = if show_placeholder {
                div()
                    .text_size(px(14.0))
                    .text_color(input_placeholder)
                    .child(
                        spec.placeholder
                            .clone()
                            .unwrap_or_default(),
                    )
            } else {
                div()
                    .text_size(px(14.0))
                    .text_color(input_text)
                    .child(display_text)
            };

            div()
                .id(SharedString::from(id_str))
                .focusable()
                .w_full()
                .min_h(input_height) // Contract: min-height, not fixed
                .px(input_padding_x)
                .flex()
                .flex_row()
                .items_center()
                .rounded(input_radius)
                .bg(input_fill)
                .border_1()
                .border_color(input_border)
                // Contract: focus ring on input
                .focus(move |s| s.border_color(focus_ring))
                .when(!is_disabled, |el| el.cursor_pointer())
                .child(text_el.flex_grow().min_w_0().overflow_x_hidden().text_ellipsis())
        };

        // ── Option list (when open) ─────────────────────────────────
        let list_el = if is_open {
            let filtered = spec.filtered_options();

            let mut list = div()
                .id("pug-combobox-list")
                .w_full()
                .mt(px(6.0)) // Contract: top calc(100% + 0.375rem)
                .rounded(list_radius)
                .bg(list_fill)
                .border_1()
                .border_color(list_border)
                // Contract: elevation-popover shadow
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.10),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(16.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.06),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(4.0),
                        spread_radius: px(0.0),
                    },
                ])
                .flex()
                .flex_col()
                .gap(px(2.0)) // Contract: gap 0.125rem
                .p(px(4.0)) // Contract: padding 0.25rem
                .overflow_y_scroll()
                .max_h(px(240.0));

            if filtered.is_empty() {
                list = list.child(
                    div()
                        .p(px(8.0)) // Contract: padding 0.5rem
                        .text_size(px(11.0)) // Contract: 0.6875rem
                        .text_color(empty_text)
                        .child("No results"),
                );
            } else {
                for option in &filtered {
                    let is_selected = current_value
                        .as_deref()
                        .map(|v| v == option.value)
                        .unwrap_or(false);
                    let is_option_disabled = option.is_disabled;

                    // Contract: option padding 0.375rem 0.5rem
                    let mut option_el = div()
                        .w_full()
                        .px(px(8.0))  // 0.5rem
                        .py(px(6.0)) // 0.375rem
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .rounded(option_radius)
                        .when(is_selected, |el| el.bg(option_highlight))
                        .when(!is_option_disabled, |el| {
                            el.cursor_pointer()
                                .hover(|style| style.bg(option_highlight))
                        })
                        .when(is_option_disabled, |el| {
                            el.opacity(disabled_opacity)
                        });

                    option_el = option_el.child(
                        div()
                            .text_size(px(14.0))
                            .text_color(option_text)
                            .child(option.label.clone()),
                    );

                    if let Some(ref desc) = option.description {
                        // Contract: description 0.6875rem, line-height 1.35
                        option_el = option_el.child(
                            div()
                                .text_size(px(11.0)) // 0.6875rem
                                .text_color(option_desc_color)
                                .child(desc.clone()),
                        );
                    }

                    list = list.child(option_el);
                }
            }

            Some(list)
        } else {
            None
        };

        // ── Root container ──────────────────────────────────────────
        // Contract: min-width 14rem
        let mut root = div()
            .w_full()
            .min_w(px(224.0)) // 14rem
            .relative()
            .flex()
            .flex_col()
            .child(input_el);

        if let Some(list) = list_el {
            root = root.child(list);
        }

        if is_disabled {
            root = root
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        root.into_any_element()
    }
}
