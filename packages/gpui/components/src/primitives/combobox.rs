//! Combobox — real GPUI component backed by ComboboxSpec.
//!
//! Contract: grid root min-width 14rem, input with focus ring,
//! absolutely positioned list with overlay shadow.
//! Option padding uses semantic.space.control.{x,y}, radius control-0.125rem.
//! Keyboard: Enter selects first match, Escape closes, Arrow opens list.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ComboboxOption, ComboboxSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI combobox component backed by `ComboboxSpec`.
pub struct Combobox {
    spec: ComboboxSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_query_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_open_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Combobox {
    type Target = ComboboxSpec;
    fn deref(&self) -> &ComboboxSpec { &self.spec }
}

impl Combobox {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ComboboxSpec::new(), theme: theme.clone(), id_suffix: None, on_change: None, on_query_change: None, on_open_change: None }
    }

    pub fn from_spec(spec: ComboboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_query_change: None,
            on_open_change: None,
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

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Combobox {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve input tokens ────────────────────────────────────
        let input_fill_raw = resolve_color(theme, spec.input_fill_token());
        let input_border_raw = resolve_color(theme, spec.input_border_token());
        // Svelte treatment-interactive-subtle: fill 82%, border 72%
        let input_fill = Hsla { a: input_fill_raw.a * 0.82, ..input_fill_raw };
        let input_hover_fill = Hsla { a: input_fill_raw.a * 0.88, ..input_fill_raw };
        let input_border = Hsla { a: input_border_raw.a * 0.72, ..input_border_raw };
        let input_hover_border = Hsla { a: input_border_raw.a * 0.92, ..input_border_raw };
        let input_text = resolve_color(theme, spec.input_text_token());
        let input_placeholder = resolve_color(theme, spec.input_placeholder_token());
        let input_radius = resolve_radius(theme, spec.input_radius_token());
        let input_height = resolve_px(theme, spec.input_height_token());
        let input_padding_x = resolve_px(theme, "semantic.space.control.x");
        let body_size = resolve_px(theme, spec.body_size_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Resolve list tokens (elevated treatment) ──────────────────
        let elevated_raw = resolve_color(theme, spec.list_fill_token());
        let border_default = resolve_color(theme, "semantic.color.border.default");
        // Svelte treatment-surface-elevated: fill 94%, border 22%
        let list_fill = Hsla { a: elevated_raw.a * 0.94, ..elevated_raw };
        let list_border = Hsla { a: border_default.a * 0.22, ..border_default };
        let list_radius = resolve_radius(theme, spec.list_radius_token());
        let option_text = resolve_color(theme, spec.option_text_token());
        let accent = resolve_color(theme, spec.option_highlight_token());
        let option_highlight = color_mix(accent, elevated_raw, 0.16);
        let option_desc_color = resolve_color(theme, spec.option_description_token());
        let empty_text = resolve_color(theme, spec.empty_text_token());

        // Contract: option radius = control - 0.125rem
        let option_radius = resolve_radius(theme, "semantic.radius.control") - px(2.0);
        let option_padding_x = resolve_px(theme, "semantic.space.control.x");
        let option_padding_y = resolve_px(theme, "semantic.space.control.y");

        let is_disabled = spec.is_disabled;
        let is_open = spec.is_open;
        let current_value = spec.current_value().map(|s| s.to_string());

        // Wrap callbacks in Rc for sharing across closures
        let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_change.map(|h| std::rc::Rc::from(h));
        let on_open_change_rc: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App)>> =
            self.on_open_change.map(|h| std::rc::Rc::from(h));

        // Collect non-disabled filtered option values for keyboard navigation
        let selectable_values: Vec<String> = if is_open {
            spec.filtered_options()
                .iter()
                .filter(|o| !o.is_disabled)
                .map(|o| o.value.clone())
                .collect()
        } else {
            Vec::new()
        };
        let first_selectable_value = selectable_values.first().cloned();

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
            format!("poodle-combobox-{}", suffix)
        } else {
            "poodle-combobox".to_string()
        };

        // ── Input field ─────────────────────────────────────────────
        let input_el = {
            let text_el = if show_placeholder {
                div()
                    .text_size(body_size)
                    .text_color(input_placeholder)
                    .child(
                        spec.placeholder
                            .clone()
                            .unwrap_or_default(),
                    )
            } else {
                div()
                    .text_size(body_size)
                    .text_color(input_text)
                    .child(display_text)
            };

            let mut input = div()
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
                .hover(move |s| s.bg(input_hover_fill).border_color(input_hover_border).shadow(vec![gpui::BoxShadow { color: hsla(0.0, 0.0, 1.0, 0.10), offset: point(px(0.0), px(1.0)), blur_radius: px(0.0), spread_radius: px(0.0) }]))
                .focus(move |s| s
                    .border_color(focus_ring)
                    .shadow(vec![gpui::BoxShadow {
                        color: Hsla { a: focus_ring.a * 0.28, ..focus_ring },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(2.0),
                    }])
                )
                .when(!is_disabled, |el| el.cursor_pointer())
                .child(text_el.flex_grow().min_w_0().overflow_x_hidden().text_ellipsis());

            // Keyboard navigation on focused input
            if !is_disabled {
                let key_change = on_change_rc.clone();
                let key_open = on_open_change_rc.clone();
                let enter_value = first_selectable_value.clone();
                let nav_values = selectable_values.clone();
                let current_sel = current_value.clone();
                let on_query_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
                    self.on_query_change.map(|h| std::rc::Rc::from(h));
                let current_query = spec.query.clone();

                input = input.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    match event.keystroke.key.as_str() {
                        "enter" => {
                            // Select first matching non-disabled option
                            if let Some(ref val) = enter_value {
                                if let Some(ref handler) = key_change {
                                    handler(val, window, cx);
                                }
                                // Close after selection
                                if let Some(ref handler) = key_open {
                                    handler(false, window, cx);
                                }
                            }
                        }
                        "escape" => {
                            if is_open {
                                if let Some(ref handler) = key_open {
                                    handler(false, window, cx);
                                }
                            }
                        }
                        "down" | "up" => {
                            if !is_open {
                                // Open the list if closed
                                if let Some(ref handler) = key_open {
                                    handler(true, window, cx);
                                }
                            } else if !nav_values.is_empty() {
                                // Navigate between options
                                let current_idx = current_sel.as_deref()
                                    .and_then(|cv| nav_values.iter().position(|v| v == cv));
                                let next_idx = match event.keystroke.key.as_str() {
                                    "down" => match current_idx {
                                        Some(i) => (i + 1) % nav_values.len(),
                                        None => 0,
                                    },
                                    _ => match current_idx {
                                        Some(0) | None => nav_values.len() - 1,
                                        Some(i) => i - 1,
                                    },
                                };
                                if let Some(ref handler) = key_change {
                                    handler(&nav_values[next_idx], window, cx);
                                }
                            }
                        }
                        "backspace" => {
                            // Delete last char from query
                            if let Some(ref handler) = on_query_rc {
                                let mut chars: Vec<char> = current_query.chars().collect();
                                if !chars.is_empty() {
                                    chars.pop();
                                    let new_q: String = chars.into_iter().collect();
                                    handler(&new_q, window, cx);
                                }
                            }
                            // Open on typing
                            if !is_open {
                                if let Some(ref handler) = key_open {
                                    handler(true, window, cx);
                                }
                            }
                        }
                        key if key.len() == 1 && !event.keystroke.modifiers.platform && !event.keystroke.modifiers.control => {
                            // Type into query
                            if let Some(ref handler) = on_query_rc {
                                let new_q = format!("{}{}", current_query, key);
                                handler(&new_q, window, cx);
                            }
                            // Open on typing
                            if !is_open {
                                if let Some(ref handler) = key_open {
                                    handler(true, window, cx);
                                }
                            }
                        }
                        _ => {}
                    }
                });

                // Click to toggle open state
                let click_open = on_open_change_rc.clone();
                input = input.on_click(move |_event, window, cx| {
                    if let Some(ref handler) = click_open {
                        handler(!is_open, window, cx);
                    }
                });
            }

            input
        };

        // ── Option list (when open) ─────────────────────────────────
        let list_el = if is_open {
            let filtered = spec.filtered_options();

            let mut list = div()
                .id("poodle-combobox-list")
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
                        .text_size(body_size) // Contract: body text size
                        .text_color(empty_text)
                        .child("No results"),
                );
            } else {
                for (idx, option) in filtered.iter().enumerate() {
                    let is_selected = current_value
                        .as_deref()
                        .map(|v| v == option.value)
                        .unwrap_or(false);
                    let is_option_disabled = option.is_disabled;
                    let option_id = SharedString::from(format!("poodle-combobox-opt-{}", idx));

                    // Contract: option padding uses resolved tokens
                    let mut option_el = div()
                        .id(option_id)
                        .w_full()
                        .px(option_padding_x)
                        .py(option_padding_y)
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

                    // Click handler to select option
                    if !is_option_disabled {
                        let val = option.value.clone();
                        let change_handler = on_change_rc.clone();
                        let close_handler = on_open_change_rc.clone();
                        option_el = option_el.on_click(move |_event, window, cx| {
                            if let Some(ref handler) = change_handler {
                                handler(&val, window, cx);
                            }
                            if let Some(ref handler) = close_handler {
                                handler(false, window, cx);
                            }
                        });
                    }

                    option_el = option_el.child(
                        div()
                            .text_size(body_size)
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
