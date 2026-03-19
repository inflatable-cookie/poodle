//! PugTimeField — real GPUI component backed by TimeFieldSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{IconSize, IconSpec, TimeFieldSpec};

use crate::icon::PugIcon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

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

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let border_error = resolve_color(theme, "semantic.color.border.error");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let hover_bg = color_mix(surface_bg, elevated, 0.84);

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
            .h(control_height)
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .gap(inline_gap)
            .text_sm();

        if is_disabled {
            field = field.opacity(disabled_opacity);
        } else {
            field = field
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg));
        }

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        // Time icon
        field = field.child(
            PugIcon::new(IconSpec::new("clock").with_size(IconSize::Sm), theme)
                .with_color(text_secondary),
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
