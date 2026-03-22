//! TimeField — real GPUI component backed by TimeFieldSpec.
//!
//! Contract: input-like control with min-height, focus ring via border,
//! no hover, no clock icon (contract doesn't specify one).

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{TimeFieldSpec, ValidationState};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI time field component backed by `TimeFieldSpec`.
pub struct TimeField {
    spec: TimeFieldSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl std::ops::Deref for TimeField {
    type Target = TimeFieldSpec;
    fn deref(&self) -> &TimeFieldSpec { &self.spec }
}

impl TimeField {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TimeFieldSpec::new(), theme: theme.clone(), id_suffix: None }
    }

    pub fn from_spec(spec: TimeFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn min(mut self, v: impl Into<String>) -> Self { self.spec.min = Some(v.into()); self }
    pub fn max(mut self, v: impl Into<String>) -> Self { self.spec.max = Some(v.into()); self }
    pub fn step(mut self, v: u32) -> Self { self.spec.step = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn described_by(mut self, v: impl Into<String>) -> Self { self.spec.described_by = Some(v.into()); self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }
}

impl IntoElement for TimeField {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_radius = resolve_radius(theme, spec.radius_token());

        let surface_bg = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.placeholder_color_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        let time_value = spec.current_value();
        let display_text = time_value.unwrap_or("HH:MM").to_string();
        let is_placeholder = time_value.is_none();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-time-field-{}", suffix)
        } else {
            "pug-time-field".to_string()
        };

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        let mut field = div()
            .id(SharedString::from(id_str))
            .min_h(control_height) // Contract: min-height, not fixed height
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .text_size(px(14.0))
            .text_color(text_primary)
            // Contract: focus = outline/border change to focus ring
            .focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            field = field
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Time value display
        field = field.child(
            div()
                .text_color(text_col)
                .child(display_text),
        );

        field.into_any_element()
    }
}
