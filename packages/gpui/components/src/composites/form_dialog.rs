//! FormDialog — dialog wrapping a form with submit/cancel actions.
//! Composes with the Dialog primitive for consistent surface rendering
//! and FormLayout for structured form content.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use crate::primitives::Dialog;
use crate::composites::FormLayout;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

pub struct FormDialog {
    theme: GpuiThemeProvider,
    title: Option<String>,
    description: Option<String>,
    submit_label: String,
    cancel_label: String,
    submitting: bool,
    disabled: bool,
    error_message: Option<String>,
    success_message: Option<String>,
    children: Vec<AnyElement>,
}

impl FormDialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            title: None,
            description: None,
            submit_label: "Submit".to_string(),
            cancel_label: "Cancel".to_string(),
            submitting: false,
            disabled: false,
            error_message: None,
            success_message: None,
            children: Vec::new(),
        }
    }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn submit_label(mut self, v: impl Into<String>) -> Self { self.submit_label = v.into(); self }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self { self.cancel_label = v.into(); self }
    pub fn submitting(mut self, v: bool) -> Self { self.submitting = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.disabled = v; self }
    pub fn error_message(mut self, v: impl Into<String>) -> Self { self.error_message = Some(v.into()); self }
    pub fn success_message(mut self, v: impl Into<String>) -> Self { self.success_message = Some(v.into()); self }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for FormDialog {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let title_color = resolve_color(theme, "semantic.color.text.primary");
        let muted_color = resolve_color(theme, "semantic.color.text.muted");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let panel_bg = resolve_color(theme, "semantic.color.background.panel");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let actions_gap = resolve_px(theme, "semantic.space.inline.sm");

        // Cancel button — text-style action
        let cancel_btn = div()
            .text_size(px(14.0))
            .text_color(title_color)
            .cursor_pointer()
            .px(px(12.0))
            .py(px(6.0))
            .rounded(control_radius)
            .hover(|s| s.bg(color_mix(title_color, panel_bg, 0.08)))
            .when(self.submitting, |el| el.opacity(0.5));

        let cancel_btn = cancel_btn.child(self.cancel_label);

        // Submit button — accent filled action
        let submit_disabled = self.submitting || self.disabled;
        let submit_bg = if submit_disabled {
            color_mix(accent, panel_bg, 0.5)
        } else {
            accent
        };

        let mut submit_btn = div()
            .text_size(px(14.0))
            .text_color(gpui::white())
            .bg(submit_bg)
            .rounded(control_radius)
            .px(px(12.0))
            .py(px(6.0))
            .cursor_pointer()
            .when(submit_disabled, |el| el.cursor_default());

        if self.submitting {
            submit_btn = submit_btn.child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .items_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(color_mix(gpui::white(), accent, 0.8))
                            .child("Submitting\u{2026}"),
                    ),
            );
        } else {
            submit_btn = submit_btn.child(self.submit_label);
        }

        let actions = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(actions_gap)
            .justify_end()
            .child(cancel_btn)
            .child(submit_btn);

        // Build form layout for the content area
        let mut form_layout = FormLayout::new(theme);
        if let Some(ref error) = self.error_message {
            form_layout = form_layout.error(error.clone());
        }
        if let Some(ref success) = self.success_message {
            form_layout = form_layout.success(success.clone());
        }
        for child in self.children {
            form_layout = form_layout.with_child(child);
        }

        // Compose with Dialog primitive — title/description handled by Dialog
        let mut dialog = Dialog::new(theme).default_open(true);

        if let Some(title) = self.title {
            dialog = dialog.title(title);
        }
        if let Some(description) = self.description {
            dialog = dialog.description(description);
        }

        dialog = dialog.with_content(form_layout);
        dialog = dialog.with_actions(actions);

        dialog.into_any_element()
    }
}
