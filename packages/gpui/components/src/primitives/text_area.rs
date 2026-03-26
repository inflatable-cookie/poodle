//! TextArea — real GPUI component backed by TextAreaSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{TextAreaSpec, ValidationState};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI text area component backed by `TextAreaSpec`.
pub struct TextArea {
    spec: TextAreaSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_submit: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TextArea {
    type Target = TextAreaSpec;
    fn deref(&self) -> &TextAreaSpec { &self.spec }
}

impl TextArea {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TextAreaSpec::new(), theme: theme.clone(), id_suffix: None, on_change: None, on_submit: None, on_cancel: None }
    }

    pub fn from_spec(spec: TextAreaSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_submit: None,
            on_cancel: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = v.into(); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn read_only(mut self, v: bool) -> Self { self.spec.is_read_only = v; self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn description_id(mut self, v: impl Into<String>) -> Self { self.spec.description_id = Some(v.into()); self }
    pub fn error_message_id(mut self, v: impl Into<String>) -> Self { self.spec.error_message_id = Some(v.into()); self }
    pub fn submit_enabled(mut self, v: bool) -> Self { self.spec.submit_enabled = v; self }
    pub fn cancel_enabled(mut self, v: bool) -> Self { self.spec.cancel_enabled = v; self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    /// Called when the text value changes.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Called on Ctrl+Enter / Cmd+Enter submit.
    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    /// Called on Escape.
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TextArea {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Token resolution ──────────────────────────────────
        let control_padding_x = resolve_px(theme, spec.horizontal_padding_token());
        let control_padding_y = resolve_px(theme, spec.vertical_padding_token());
        let control_radius = resolve_radius(theme, spec.radius_token());
        let body_size = resolve_px(theme, spec.body_size_token());
        let line_height_val = resolve_px(theme, spec.body_line_height_token());
        let line_height_f = theme.resolve_space(spec.body_line_height_token());

        let surface_raw = resolve_color(theme, spec.fill_token());
        let border_default = resolve_color(theme, spec.border_token());
        // Svelte treatment-interactive-subtle: fill 82%, border 72%
        let surface_bg = Hsla { a: surface_raw.a * 0.82, ..surface_raw };
        let hover_bg = Hsla { a: surface_raw.a * 0.88, ..surface_raw };
        let border = Hsla { a: border_default.a * 0.72, ..border_default };
        let hover_border = Hsla { a: border_default.a * 0.92, ..border_default };
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.placeholder_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let value = spec.current_value();
        let is_empty = value.is_empty();
        let display_text = if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            value.to_string()
        };
        let text_col = if is_empty { text_secondary } else { text_primary };

        // Contract: min-height = rows × line-height
        let row_height = spec.rows as f32 * line_height_f;

        // Contract: validation state border colors
        let effective_border = match spec.validation_state {
            ValidationState::Invalid => resolve_color(theme, "semantic.color.status.danger"),
            ValidationState::Valid => resolve_color(theme, "semantic.color.status.success"),
            ValidationState::Pending => resolve_color(theme, "semantic.color.accent.base"),
            _ => border,
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-textarea-{}", suffix)
        } else {
            "poodle-textarea".to_string()
        };

        let focus_bg = surface_bg;

        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .min_h(px(row_height))
            .px(control_padding_x)
            .py(control_padding_y)
            .rounded(control_radius);

        // Brand-raised treatment: gradient fill + subtle shadow
        if theme.brand_raised {
            el = el.bg(crate::theme_ext::brand_raised_subtle_fill(surface_bg))
                .shadow(vec![gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 1.0, 0.08),
                    offset: point(px(0.0), px(-1.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(0.0),
                }]);
        } else {
            el = el.bg(surface_bg);
        }

        el = el.border_1()
            .border_color(effective_border)
            .text_size(body_size)
            .line_height(line_height_val)
            .text_color(text_col)
            .hover(move |s| s.bg(hover_bg).border_color(hover_border).shadow(vec![gpui::BoxShadow { color: hsla(0.0, 0.0, 1.0, 0.10), offset: point(px(0.0), px(1.0)), blur_radius: px(0.0), spread_radius: px(0.0) }]))
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
            .child(display_text);

        if spec.is_disabled {
            el = el.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        // Keyboard handlers for text editing
        if !spec.is_disabled && !spec.is_read_only {
            let current_value = value.to_string();
            let on_change = self.on_change.clone();
            let on_submit = self.on_submit;
            let on_cancel = self.on_cancel;

            el = el.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if key == "enter" {
                    if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
                        // Ctrl/Cmd+Enter = submit
                        if let Some(ref handler) = on_submit {
                            handler(&current_value, window, cx);
                        }
                    } else {
                        // Plain Enter = newline
                        if let Some(ref handler) = on_change {
                            let new_val = format!("{}\n", current_value);
                            handler(&new_val, window, cx);
                        }
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

        el.into_any_element()
    }
}
