//! PugCombobox — real GPUI component backed by ComboboxSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ComboboxSpec;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI combobox component backed by `ComboboxSpec`.
pub struct PugCombobox {
    spec: ComboboxSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_query_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugCombobox {
    pub fn new(spec: ComboboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_query_change: None,
        }
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

impl IntoElement for PugCombobox {
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

        // ── Input field ─────────────────────────────────────────────
        let input_el = {
            let text_el = if show_placeholder {
                div()
                    .text_sm()
                    .text_color(input_placeholder)
                    .child(
                        spec.placeholder
                            .clone()
                            .unwrap_or_default(),
                    )
            } else {
                div()
                    .text_sm()
                    .text_color(input_text)
                    .child(display_text)
            };

            // Chevron arrow indicator
            let chevron = div()
                .text_xs()
                .text_color(input_placeholder)
                .child(if is_open { "\u{25B2}" } else { "\u{25BC}" });

            div()
                .w_full()
                .h(input_height)
                .px(px(10.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .rounded(input_radius)
                .bg(input_fill)
                .border_1()
                .border_color(input_border)
                .when(!is_disabled, |el| el.cursor_pointer())
                .child(text_el.flex_grow().min_w_0().overflow_x_hidden().text_ellipsis())
                .child(chevron)
        };

        // ── Option list (when open) ─────────────────────────────────
        let list_el = if is_open {
            let filtered = spec.filtered_options();

            let mut list = div()
                .w_full()
                .mt(px(4.0))
                .rounded(list_radius)
                .bg(list_fill)
                .border_1()
                .border_color(list_border)
                .flex()
                .flex_col()
                .py(px(4.0))
                .overflow_hidden()
                .max_h(px(240.0));

            if filtered.is_empty() {
                list = list.child(
                    div()
                        .px(px(10.0))
                        .py(px(8.0))
                        .text_sm()
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

                    let mut option_el = div()
                        .w_full()
                        .px(px(10.0))
                        .py(px(6.0))
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
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
                            .text_sm()
                            .text_color(option_text)
                            .child(option.label.clone()),
                    );

                    if let Some(ref desc) = option.description {
                        option_el = option_el.child(
                            div()
                                .text_xs()
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
        let mut root = div()
            .w_full()
            .relative()
            .flex()
            .flex_col()
            .child(input_el);

        if let Some(list) = list_el {
            root = root.child(list);
        }

        if is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
