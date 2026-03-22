//! TextArea — real GPUI component backed by TextAreaSpec.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{TextAreaSpec, ValidationState};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI text area component backed by `TextAreaSpec`.
pub struct TextArea {
    spec: TextAreaSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl std::ops::Deref for TextArea {
    type Target = TextAreaSpec;
    fn deref(&self) -> &TextAreaSpec { &self.spec }
}

impl TextArea {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TextAreaSpec::new(), theme: theme.clone(), id_suffix: None }
    }

    pub fn from_spec(spec: TextAreaSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
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
}

impl IntoElement for TextArea {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_padding_y = resolve_px(theme, "semantic.space.control.y");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        // Line height from typography token
        let line_height_f = theme.resolve_space("semantic.typography.body.lineHeight");
        let line_height = px(line_height_f);

        let surface_bg = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let value = spec.current_value();
        let is_empty = value.is_empty();
        let display_text = if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            value.to_string()
        };
        let text_col = if is_empty { text_secondary } else { text_primary };

        let row_height_f = spec.rows as f32 * line_height_f;
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-textarea-{}", suffix)
        } else {
            "pug-textarea".to_string()
        };

        let mut el = div()
            .id(SharedString::from(id_str))
            .min_h(px(row_height_f))
            .px(control_padding_x)
            .py(control_padding_y)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .text_size(px(14.0))
            .text_color(text_col)
            // Contract: focus-within = border switches to focus ring
            .focus(move |s| s.border_color(focus_ring))
            .child(display_text);

        if spec.is_disabled {
            el = el.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        el.into_any_element()
    }
}
