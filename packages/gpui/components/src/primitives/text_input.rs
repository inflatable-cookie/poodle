//! TextInput — real GPUI component backed by TextInputSpec.
//!
//! Note: gpui doesn't provide a native text input widget, so this renders
//! a styled container displaying the current value. Real text editing would
//! require gpui's internal input handling.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{IconSize, IconSpec, TextInputSpec, ValidationState};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI text input component backed by `TextInputSpec`.
pub struct TextInput {
    spec: TextInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_focus: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_submit: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TextInput {
    type Target = TextInputSpec;
    fn deref(&self) -> &TextInputSpec { &self.spec }
}

impl TextInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TextInputSpec::new(), theme: theme.clone(), id_suffix: None, on_focus: None, on_change: None, on_submit: None, on_cancel: None }
    }

    pub fn from_spec(spec: TextInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_focus: None,
            on_change: None,
            on_submit: None,
            on_cancel: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn id(mut self, v: impl Into<String>) -> Self { self.spec.id = Some(v.into()); self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = v.into(); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
    pub fn name(mut self, v: impl Into<String>) -> Self { self.spec.name = Some(v.into()); self }
    pub fn input_type(mut self, v: impl Into<String>) -> Self { self.spec.input_type = v.into(); self }
    pub fn input_mode(mut self, v: impl Into<String>) -> Self { self.spec.input_mode = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn read_only(mut self, v: bool) -> Self { self.spec.is_read_only = v; self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn description_id(mut self, v: impl Into<String>) -> Self { self.spec.description_id = Some(v.into()); self }
    pub fn error_message_id(mut self, v: impl Into<String>) -> Self { self.spec.error_message_id = Some(v.into()); self }
    pub fn leading_icon(mut self, v: impl Into<String>) -> Self { self.spec.leading_icon = Some(v.into()); self }
    pub fn trailing_icon(mut self, v: impl Into<String>) -> Self { self.spec.trailing_icon = Some(v.into()); self }
    pub fn prefix(mut self, v: impl Into<String>) -> Self { self.spec.prefix = Some(v.into()); self }
    pub fn suffix(mut self, v: impl Into<String>) -> Self { self.spec.suffix = Some(v.into()); self }
    pub fn max_length(mut self, v: usize) -> Self { self.spec.max_length = Some(v); self }
    pub fn show_char_count(mut self, v: bool) -> Self { self.spec.show_char_count = v; self }
    pub fn submit_enabled(mut self, v: bool) -> Self { self.spec.submit_enabled = v; self }
    pub fn cancel_enabled(mut self, v: bool) -> Self { self.spec.cancel_enabled = v; self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_focus(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_focus = Some(Box::new(handler));
        self
    }

    /// Called when the input value changes.
    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Called when the user presses Enter (submit).
    pub fn on_submit(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    /// Called when the user presses Escape (cancel).
    pub fn on_cancel(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TextInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Token resolution ──────────────────────────────────
        let control_height = resolve_px(theme, spec.control_height_token());
        let inline_padding = resolve_px(theme, spec.horizontal_padding_token());
        let inline_gap = resolve_px(theme, spec.inline_gap_token());
        let control_radius = resolve_radius(theme, spec.radius_token());
        let body_size = resolve_px(theme, spec.body_size_token());
        let body_line_height = resolve_px(theme, spec.body_line_height_token());

        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let border_default = resolve_color(theme, spec.border_token());
        let surface_raw = resolve_color(theme, spec.fill_token());
        // Svelte treatment-interactive-subtle values:
        //   fill: surface 82%, fill-hover: surface 88%
        //   border: border-default 72%, border-hover: border-default 92%
        let surface_bg = Hsla { a: surface_raw.a * 0.82, ..surface_raw };
        let hover_bg = Hsla { a: surface_raw.a * 0.88, ..surface_raw };
        let border = Hsla { a: border_default.a * 0.72, ..border_default };
        let hover_border = Hsla { a: border_default.a * 0.92, ..border_default };
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.placeholder_color_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let icon_color = resolve_color(theme, spec.icon_color_token());

        let value = spec.current_value();
        let is_empty = value.is_empty();
        let display_text = if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            value.to_string()
        };
        let text_col = if is_empty { text_secondary } else { text_primary };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-input-{}", suffix)
        } else {
            "poodle-input".to_string()
        };

        // Contract: validation state border colors
        let effective_border = match spec.validation_state {
            ValidationState::Invalid => resolve_color(theme, "semantic.color.status.danger"),
            ValidationState::Valid => resolve_color(theme, "semantic.color.status.success"),
            ValidationState::Pending => resolve_color(theme, "semantic.color.accent.base"),
            _ => border,
        };

        let focus_bg = surface_bg;

        // ── Build inner content row ──────────────────────────
        let mut inner = div()
            .flex()
            .items_center()
            .gap(inline_gap)
            .size_full();

        // Prefix affix
        if let Some(ref prefix_text) = spec.prefix {
            let affix_color = resolve_color(theme, spec.affix_color_token());
            let separator_base = resolve_color(theme, spec.affix_separator_color_token());
            let separator_color = color_mix(separator_base, gpui::Hsla::transparent_black(), 0.52);
            inner = inner.child(
                div()
                    .flex().items_center()
                    .pr(inline_gap).mr(inline_gap)
                    .border_r_1().border_color(separator_color)
                    .text_color(affix_color)
                    .whitespace_nowrap()
                    .child(prefix_text.clone()),
            );
        }

        // Leading icon
        if let Some(ref icon) = spec.leading_icon {
            inner = inner.child(
                Icon::from_spec(IconSpec::new(icon).with_size(IconSize::Sm), theme)
                    .with_color(icon_color),
            );
        }

        // Value / placeholder text
        inner = inner.child(
            div()
                .flex_1()
                .overflow_x_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .text_color(text_col)
                .child(SharedString::from(display_text)),
        );

        // Character count
        if spec.show_char_count {
            let char_len = value.len();
            let is_over = spec.max_length.map_or(false, |max| char_len > max);
            let count_color = if is_over {
                resolve_color(theme, spec.char_count_over_color_token())
            } else {
                resolve_color(theme, spec.char_count_color_token())
            };
            let count_text = if let Some(max) = spec.max_length {
                format!("{}/{}", char_len, max)
            } else {
                format!("{}", char_len)
            };
            inner = inner.child(
                div().text_color(count_color).text_size(px(11.0))
                    .whitespace_nowrap().child(count_text),
            );
        }

        // Validation indicator (circle-check, circle-x, or spinner)
        match spec.validation_state {
            ValidationState::Valid => {
                let success_color = resolve_color(theme, "semantic.color.status.success");
                inner = inner.child(
                    Icon::from_spec(IconSpec::new("circle-check").with_size(IconSize::Sm), theme)
                        .with_color(success_color),
                );
            }
            ValidationState::Invalid => {
                let danger_color = resolve_color(theme, "semantic.color.status.danger");
                inner = inner.child(
                    Icon::from_spec(IconSpec::new("circle-x").with_size(IconSize::Sm), theme)
                        .with_color(danger_color),
                );
            }
            ValidationState::Pending => {
                let accent_color = resolve_color(theme, "semantic.color.accent.base");
                inner = inner.child(
                    Icon::from_spec(IconSpec::new("loader").with_size(IconSize::Sm), theme)
                        .with_color(accent_color),
                );
            }
            _ => {}
        }

        // Trailing icon
        if let Some(ref icon) = spec.trailing_icon {
            inner = inner.child(
                Icon::from_spec(IconSpec::new(icon).with_size(IconSize::Sm), theme)
                    .with_color(icon_color),
            );
        }

        // Suffix affix
        if let Some(ref suffix_text) = spec.suffix {
            let affix_color = resolve_color(theme, spec.affix_color_token());
            let separator_base = resolve_color(theme, spec.affix_separator_color_token());
            let separator_color = color_mix(separator_base, gpui::Hsla::transparent_black(), 0.52);
            inner = inner.child(
                div()
                    .flex().items_center()
                    .pl(inline_gap).ml(inline_gap)
                    .border_l_1().border_color(separator_color)
                    .text_color(affix_color)
                    .whitespace_nowrap()
                    .child(suffix_text.clone()),
            );
        }

        // ── Outer container ──────────────────────────────────
        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(effective_border)
            .text_size(body_size)
            .line_height(body_line_height)
            .text_color(text_primary)
            .hover(move |s| s.bg(hover_bg).border_color(hover_border))
            .focus(move |s| s
                .border_color(focus_ring)
                .bg(focus_bg)
                .shadow(vec![gpui::BoxShadow {
                    color: Hsla { a: focus_ring.a * 0.28, ..focus_ring },
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(2.0),
                }])
            )
            .child(inner);

        if spec.is_disabled {
            el = el.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        // ── Keyboard handlers ─────────────────────────────────
        if !spec.is_disabled && !spec.is_read_only {
            let current_value = value.to_string();
            let on_change = self.on_change.clone();
            let on_submit = self.on_submit;
            let on_cancel = self.on_cancel;

            el = el.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if key == "enter" {
                    if let Some(ref handler) = on_submit {
                        handler(&current_value, window, cx);
                    }
                } else if key == "escape" {
                    if let Some(ref handler) = on_cancel {
                        handler(window, cx);
                    }
                } else if key == "backspace" {
                    if let Some(ref handler) = on_change {
                        let mut chars: Vec<char> = current_value.chars().collect();
                        if !chars.is_empty() {
                            chars.pop();
                            let new_val: String = chars.into_iter().collect();
                            handler(&new_val, window, cx);
                        }
                    }
                } else if key.len() == 1 && !event.keystroke.modifiers.platform && !event.keystroke.modifiers.control {
                    if let Some(ref handler) = on_change {
                        let new_val = format!("{}{}", current_value, key);
                        handler(&new_val, window, cx);
                    }
                }
            });
        }

        // Focus click handler
        if let Some(handler) = self.on_focus {
            if !spec.is_disabled {
                el = el.on_click(move |_event, window, cx| {
                    handler(window, cx);
                });
            }
        }

        el.into_any_element()
    }
}
