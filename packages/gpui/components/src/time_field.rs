//! PugTimeField — real GPUI component backed by TimeFieldSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TimeFieldSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI time field component backed by `TimeFieldSpec`.
///
/// Renders an input-like display showing the current time value or a
/// "HH:MM" placeholder. Styled like a text input with border and
/// elevated background.
pub struct PugTimeField {
    spec: TimeFieldSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl PugTimeField {
    pub fn new(spec: TimeFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }
}

impl IntoElement for PugTimeField {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let border_error = resolve_color(theme, "semantic.color.border.error");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let time_value = spec.current_value();
        let display_text = time_value.unwrap_or("HH:MM").to_string();
        let is_placeholder = time_value.is_none();
        let is_disabled = spec.is_disabled;

        let border_color = match spec.validation_state {
            pug_gpui_primitives::ValidationState::Invalid => border_error,
            _ => border,
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-time-field-{}", suffix)
        } else {
            "pug-time-field".to_string()
        };

        let mut field = div()
            .id(SharedString::from(id_str))
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(elevated_bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .gap(px(8.0))
            .text_sm();

        if is_disabled {
            field = field.opacity(0.48);
        }

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        // Time icon
        field = field.child(
            div()
                .text_xs()
                .text_color(text_secondary)
                .child("🕐"),
        );

        // Time value
        field = field.child(
            div()
                .text_color(text_col)
                .child(display_text),
        );

        field.into_any_element()
    }
}
