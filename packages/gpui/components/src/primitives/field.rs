//! Field — real GPUI component backed by FieldSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{FieldSpec, ValidationState};

use crate::theme_ext::resolve_color;

/// A real GPUI field wrapper component backed by `FieldSpec`.
///
/// Renders label, optional indicator, description, error message,
/// and a slot for the control (input, select, etc.).
pub struct Field {
    spec: FieldSpec,
    theme: GpuiThemeProvider,
    /// The form control to wrap.
    control: Option<AnyElement>,
}

impl std::ops::Deref for Field {
    type Target = FieldSpec;
    fn deref(&self) -> &FieldSpec { &self.spec }
}

impl Field {
    pub fn new(id: impl Into<String>, label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: FieldSpec::new(id, label), theme: theme.clone(), control: None }
    }

    pub fn from_spec(spec: FieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            control: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn id(mut self, v: impl Into<String>) -> Self { self.spec.id = v.into(); self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn error(mut self, v: impl Into<String>) -> Self { self.spec.error = Some(v.into()); self }
    pub fn pending_message(mut self, v: impl Into<String>) -> Self { self.spec.pending_message = Some(v.into()); self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn required(mut self, v: bool) -> Self { self.spec.is_required = v; self }
    pub fn optional_label(mut self, v: impl Into<String>) -> Self { self.spec.optional_label = Some(v.into()); self }


    /// Set the form control element that this field wraps.
    pub fn with_control(mut self, control: impl IntoElement) -> Self {
        self.control = Some(control.into_any_element());
        self
    }
}

impl IntoElement for Field {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let description_color = resolve_color(theme, spec.description_color_token());
        let error_color = resolve_color(theme, spec.error_color_token());

        let mut col = div().flex().flex_col().gap(px(4.0));

        // Label row
        let mut label_row = div().flex().items_center().gap(px(4.0));
        label_row = label_row.child(
            div()
                .text_sm()
                .text_color(text_primary)
                .child(spec.label.clone()),
        );

        if spec.is_required {
            label_row = label_row.child(
                div().text_sm().text_color(error_color).child("*"),
            );
        } else if spec.shows_optional_label() {
            if let Some(ref opt_label) = spec.optional_label {
                label_row = label_row.child(
                    div()
                        .text_xs()
                        .text_color(description_color)
                        .child(opt_label.clone()),
                );
            }
        }

        col = col.child(label_row);

        // Description
        if let Some(ref description) = spec.description {
            col = col.child(
                div()
                    .text_xs()
                    .text_color(description_color)
                    .child(description.clone()),
            );
        }

        // Control slot
        if let Some(control) = self.control {
            col = col.child(control);
        }

        // Error message
        if spec.validation_state == ValidationState::Invalid {
            if let Some(ref error) = spec.error {
                col = col.child(
                    div()
                        .text_xs()
                        .text_color(error_color)
                        .child(error.clone()),
                );
            }
        }

        // Pending message
        if spec.validation_state == ValidationState::Pending {
            if let Some(ref pending) = spec.pending_message {
                col = col.child(
                    div()
                        .text_xs()
                        .text_color(description_color)
                        .child(pending.clone()),
                );
            }
        }

        col.into_any_element()
    }
}
