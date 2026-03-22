//! FormDialog — dialog wrapping a form with submit/cancel actions.
//! Composes with the Dialog primitive for consistent surface rendering.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use crate::primitives::Dialog;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct FormDialog {
    theme: GpuiThemeProvider,
    title: Option<String>,
    description: Option<String>,
    submit_label: String,
    cancel_label: String,
    submitting: bool,
    content: Option<AnyElement>,
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
            content: None,
        }
    }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn submit_label(mut self, v: impl Into<String>) -> Self { self.submit_label = v.into(); self }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self { self.cancel_label = v.into(); self }
    pub fn submitting(mut self, v: bool) -> Self { self.submitting = v; self }
    pub fn with_content(mut self, c: impl IntoElement) -> Self { self.content = Some(c.into_any_element()); self }
}

impl IntoElement for FormDialog {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let title_color = resolve_color(theme, "semantic.color.text.primary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let muted_color = resolve_color(theme, "semantic.color.text.muted");

        // Build the actions row
        let cancel_btn = div()
            .text_size(px(14.0))
            .text_color(title_color)
            .cursor_pointer()
            .when(self.submitting, |el| el.opacity(0.5))
            .child(self.cancel_label);

        let mut submit_btn = div()
            .text_size(px(14.0))
            .text_color(gpui::white())
            .bg(accent)
            .rounded(control_radius)
            .px(px(12.0)).py(px(6.0))
            .cursor_pointer();

        if self.submitting {
            submit_btn = submit_btn
                .opacity(0.6)
                .child(div().flex().flex_row().gap(px(6.0)).items_center()
                    .child(div().text_size(px(12.0)).text_color(muted_color).child("Submitting\u{2026}"))
                );
        } else {
            submit_btn = submit_btn.child(self.submit_label);
        }

        let actions = div()
            .flex().flex_row().gap(px(8.0)).justify_end()
            .child(cancel_btn)
            .child(submit_btn);

        // Compose with the Dialog primitive — title/description handled by Dialog
        let mut dialog = Dialog::new(theme)
            .default_open(true);

        if let Some(title) = self.title {
            dialog = dialog.title(title);
        }
        if let Some(description) = self.description {
            dialog = dialog.description(description);
        }

        // Form content slot
        if let Some(content) = self.content {
            dialog = dialog.with_content(content);
        }

        dialog = dialog.with_actions(actions);

        dialog.into_any_element()
    }
}
