//! EditableLabel — real GPUI component backed by EditableLabelSpec.
//!
//! Contract: inline-flex, padding 0.375rem 0.5rem,
//! transparent border in display mode, accent border in editing mode.
//! Hover hint in display mode. Focus ring via border.

use gpui::{prelude::FluentBuilder, *};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, EditableLabelSpec, EditableLabelVariant, IconSize, IconSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI editable label component backed by `EditableLabelSpec`.
pub struct EditableLabel {
    spec: EditableLabelSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_commit: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for EditableLabel {
    type Target = EditableLabelSpec;
    fn deref(&self) -> &EditableLabelSpec {
        &self.spec
    }
}

impl EditableLabel {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EditableLabelSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_commit: None,
            on_cancel: None,
        }
    }

    pub fn from_spec(spec: EditableLabelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_commit: None,
            on_cancel: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = v.into();
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = Some(v.into());
        self
    }
    pub fn editing(mut self, v: bool) -> Self {
        self.spec.is_editing = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    /// Called when the value changes during editing.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Called when editing is committed (Enter key).
    pub fn on_commit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_commit = Some(Box::new(handler));
        self
    }

    /// Called when editing is cancelled (Escape key).
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl IntoElement for EditableLabel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let text_color = resolve_color(theme, spec.text_color_token());
        let placeholder_color = resolve_color(theme, spec.placeholder_color_token());
        let border_color = resolve_color(theme, spec.edit_border_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let control_radius = resolve_radius(theme, spec.radius_token());
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let pad_x = px(rem_to_px(0.5 + size_padding_x_offset_rem(effective_size)));
        let pad_y = px(rem_to_px(0.375));

        // Contract: hover hint border in display mode
        let surface_bg = resolve_color(theme, "color.background.surface");
        let default_border = resolve_color(theme, "color.border.default");
        let hover_border = color_mix(default_border, surface_bg, 0.72);
        let hover_bg = color_mix(surface_bg, gpui::transparent_black(), 0.52);

        let is_flush = spec.variant == EditableLabelVariant::Flush;

        let is_empty = spec.value.is_empty();
        // empty_text takes precedence in display mode; placeholder is
        // for the editing input
        let display_text = if is_empty && !spec.is_editing {
            spec.empty_text
                .clone()
                .or_else(|| spec.placeholder.clone())
                .unwrap_or_default()
        } else if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            spec.value.clone()
        };

        let text_col = if is_empty {
            placeholder_color
        } else {
            text_color
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-editable-label-{}", suffix)
        } else {
            "poodle-editable-label".to_string()
        };

        // Contract: padding scales with effective size. Flush variant
        // strips padding, border, and background for inline rendering.
        let mut content_row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(display_text);

        // Show pencil edit icon on hover/focus when spec says so
        if spec.show_edit_icon && !spec.is_editing && !spec.is_disabled {
            let icon_color = resolve_color(theme, "color.text.muted");
            content_row = content_row.child(
                super::icon::Icon::from_spec(
                    IconSpec::new("pencil").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color),
            );
        }

        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .text_size(body_size)
            .text_color(text_col)
            .when(!is_flush, |el| {
                el.px(pad_x).py(pad_y).rounded(control_radius).border_1()
            })
            .when(is_flush, |el| el.border_0())
            .child(content_row);

        if spec.is_editing {
            // Contract: editing mode — accent border + surface bg
            el = el
                .border_color(border_color)
                .bg(surface_bg)
                .focus(move |s| {
                    s.border_color(focus_ring).shadow(vec![gpui::BoxShadow {
                        color: Hsla {
                            a: focus_ring.a * 0.28,
                            ..focus_ring
                        },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(2.0),
                    }])
                });
        } else {
            // Contract: display mode — transparent border, hover hint
            el = el
                .border_color(gpui::transparent_black())
                .when(!spec.is_disabled, |el| {
                    el.cursor_pointer()
                        .hover(move |s| s.border_color(hover_border).bg(hover_bg))
                })
                .focus(move |s| {
                    s.border_color(focus_ring).shadow(vec![gpui::BoxShadow {
                        color: Hsla {
                            a: focus_ring.a * 0.28,
                            ..focus_ring
                        },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(2.0),
                    }])
                });
        }

        if spec.is_disabled {
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Keyboard handlers when editing
        if spec.is_editing && !spec.is_disabled {
            let current_value = spec.value.clone();
            let on_change = self.on_change.clone();
            let on_commit = self.on_commit;
            let on_cancel = self.on_cancel;

            el = el.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if key == "enter" {
                    if let Some(ref handler) = on_commit {
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
                } else if key.len() == 1
                    && !event.keystroke.modifiers.platform
                    && !event.keystroke.modifiers.control
                {
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
