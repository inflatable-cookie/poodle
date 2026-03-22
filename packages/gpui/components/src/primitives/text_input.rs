//! TextInput — real GPUI component backed by TextInputSpec.
//!
//! Note: gpui doesn't provide a native text input widget, so this renders
//! a styled container displaying the current value. Real text editing would
//! require gpui's internal input handling.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, TextInputSpec, ValidationState};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

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

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.control.x");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.placeholder_color_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let value = spec.current_value();
        let is_empty = value.is_empty();
        let display_text = if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            value.to_string()
        };
        let text_col = if is_empty { text_secondary } else { text_primary };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-input-{}", suffix)
        } else {
            "pug-input".to_string()
        };

        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .min_h(control_height) // contract: min-height, not fixed height
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap(inline_gap)
            .text_size(px(14.0))
            .text_color(text_primary)
            // Contract: focus-within = border switches to focus ring color
            .focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            el = el.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        // Leading icon — render via Icon
        if let Some(ref icon) = spec.leading_icon {
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(icon).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_secondary),
            );
        }

        // Value / placeholder
        el = el.child(
            div()
                .flex_1()
                .text_color(text_col)
                .overflow_x_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(display_text),
        );

        // Trailing icon — render via Icon
        if let Some(ref icon) = spec.trailing_icon {
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(icon).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_secondary),
            );
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
                    // Single printable character
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
