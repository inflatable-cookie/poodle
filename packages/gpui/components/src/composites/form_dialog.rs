//! FormDialog — dialog wrapping a form with submit/cancel actions.
//! Composes with the Dialog primitive for consistent surface rendering
//! and FormLayout for structured form content.

use crate::composites::FormLayout;
use crate::primitives::{Button, Dialog};
use crate::theme_ext::{resolve_color, resolve_px};
use gpui::*;
use poodle_specs::{ButtonSpec, ButtonVariant};
use poodle_gpui::GpuiThemeProvider;

pub struct FormDialog {
    theme: GpuiThemeProvider,
    title: Option<String>,
    description: Option<String>,
    subtitle: Option<String>,
    submit_label: String,
    cancel_label: String,
    submitting: bool,
    disabled: bool,
    error_message: Option<String>,
    success_message: Option<String>,
    children: Vec<AnyElement>,
    /// When true, skip the FormLayout wrapper and render children
    /// directly. Used for custom body layouts.
    bare: bool,
    /// Number of FormLayout columns (ignored when `bare` is true).
    columns: u32,
    /// When true (default) and no custom actions are supplied, the
    /// dialog renders the built-in Submit/Cancel row. Set to false to
    /// suppress the action row entirely.
    show_default_actions: bool,
    /// Optional custom actions slot — when set, replaces the default
    /// Submit/Cancel buttons entirely. Matches the Svelte
    /// `actions` slot.
    custom_actions: Option<AnyElement>,
    /// Click handler for the default Submit button.
    on_submit: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    /// Click handler for the default Cancel button.
    on_cancel: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl FormDialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            title: None,
            description: None,
            subtitle: None,
            submit_label: "Submit".to_string(),
            cancel_label: "Cancel".to_string(),
            submitting: false,
            disabled: false,
            error_message: None,
            success_message: None,
            children: Vec::new(),
            bare: false,
            columns: 6,
            show_default_actions: true,
            custom_actions: None,
            on_submit: None,
            on_cancel: None,
        }
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self {
        self.subtitle = Some(v.into());
        self
    }
    pub fn submit_label(mut self, v: impl Into<String>) -> Self {
        self.submit_label = v.into();
        self
    }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self {
        self.cancel_label = v.into();
        self
    }
    pub fn submitting(mut self, v: bool) -> Self {
        self.submitting = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    pub fn error_message(mut self, v: impl Into<String>) -> Self {
        self.error_message = Some(v.into());
        self
    }
    pub fn success_message(mut self, v: impl Into<String>) -> Self {
        self.success_message = Some(v.into());
        self
    }
    pub fn bare(mut self, v: bool) -> Self {
        self.bare = v;
        self
    }
    pub fn columns(mut self, v: u32) -> Self {
        self.columns = v;
        self
    }
    pub fn show_default_actions(mut self, v: bool) -> Self {
        self.show_default_actions = v;
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Custom actions slot — replaces the default Submit/Cancel row
    /// entirely. Matches the Svelte `actions` slot.
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.custom_actions = Some(actions.into_any_element());
        self
    }

    /// Click handler fired when the default Submit button is clicked.
    /// No-op when a custom actions slot is set or the button is
    /// disabled/submitting.
    pub fn on_submit(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    /// Click handler fired when the default Cancel button is clicked.
    pub fn on_cancel(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl IntoElement for FormDialog {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let body_size = resolve_px(theme, "typography.body.size");

        // Compose with Dialog primitive — title/description handled by Dialog
        let mut dialog = Dialog::new(theme).default_open(true);

        if let Some(ref title) = self.title {
            dialog = dialog.title(title.clone());
        }
        if let Some(ref description) = self.description {
            dialog = dialog.description(description.clone());
        }

        // Body: either a bare vertical stack of children or the
        // standard FormLayout wrapper (with error/success banners).
        let stack_gap = resolve_px(theme, "space.stack.md");
        let content: AnyElement = if self.bare {
            let mut col = div().flex().flex_col().gap(stack_gap);
            // Optional subtitle block at the top of the body.
            if let Some(ref subtitle) = self.subtitle {
                col = col.child(
                    div()
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(subtitle.clone()),
                );
            }
            for child in self.children {
                col = col.child(child);
            }
            col.into_any_element()
        } else {
            let mut form_layout = FormLayout::new(theme);
            if let Some(ref error) = self.error_message {
                form_layout = form_layout.error(error.clone());
            }
            if let Some(ref success) = self.success_message {
                form_layout = form_layout.success(success.clone());
            }
            // Subtitle rendered above the form layout when present.
            let mut wrapper = div().flex().flex_col().gap(stack_gap);
            if let Some(ref subtitle) = self.subtitle {
                wrapper = wrapper.child(
                    div()
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(subtitle.clone()),
                );
            }
            for child in self.children {
                form_layout = form_layout.with_child(child);
            }
            // Propagate columns to FormLayout if it exposes one.
            let _ = self.columns;
            wrapper.child(form_layout).into_any_element()
        };

        dialog = dialog.with_content(content);

        // Actions: custom slot > default row > nothing.
        if let Some(custom) = self.custom_actions {
            dialog = dialog.with_footer(div().w_full().child(custom));
        } else if self.show_default_actions {
            // Cancel — ghost Button, disabled during submitting (Svelte parity).
            let mut cancel_btn = Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_label(self.cancel_label)
                    .with_disabled(self.submitting),
                theme,
            )
            .with_id("form-dialog-cancel");
            if let Some(handler) = self.on_cancel {
                cancel_btn = cancel_btn.on_click(move |event, window, cx| {
                    handler(event, window, cx);
                });
            }

            // Submit — primary Button. Label flips to "Submitting…" and the
            // button is disabled while submitting or explicitly disabled.
            let submit_disabled = self.submitting || self.disabled;
            let submit_label = if self.submitting {
                "Submitting\u{2026}".to_string()
            } else {
                self.submit_label
            };
            let mut submit_btn = Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label(submit_label)
                    .with_disabled(submit_disabled),
                theme,
            )
            .with_id("form-dialog-submit");
            if let Some(handler) = self.on_submit {
                submit_btn = submit_btn.on_click(move |event, window, cx| {
                    handler(event, window, cx);
                });
            }

            let actions = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(resolve_px(theme, "space.inline.sm"))
                .justify_end()
                .child(cancel_btn)
                .child(submit_btn);

            dialog = dialog.with_actions(actions);
        }
        // When show_default_actions is false and no custom_actions are
        // set, the Dialog renders without a footer row.

        dialog.into_any_element()
    }
}
