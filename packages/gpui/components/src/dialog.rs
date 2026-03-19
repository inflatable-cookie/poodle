//! PugDialog — real GPUI component backed by DialogSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::DialogSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI dialog component backed by `DialogSpec`.
///
/// Renders the dialog surface (elevated card with title/description).
/// The parent is responsible for conditionally rendering based on `spec.current_open()`.
pub struct PugDialog {
    spec: DialogSpec,
    theme: GpuiThemeProvider,
    /// Actions slot — typically buttons rendered by the parent.
    actions: Option<AnyElement>,
}

impl PugDialog {
    pub fn new(spec: DialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: None,
        }
    }

    /// Add an actions row (e.g., Cancel + Confirm buttons).
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }
}

impl IntoElement for PugDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let surface_bg = resolve_color(theme, spec.surface_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let mut dialog = div()
            .p(px(16.0))
            .rounded(px(8.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(12.0));

        // Title
        if let Some(ref title) = spec.title {
            dialog = dialog.child(div().text_base().child(title.clone()));
        }

        // Description
        if let Some(ref description) = spec.description {
            dialog = dialog.child(
                div()
                    .text_sm()
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Actions slot
        if let Some(actions) = self.actions {
            dialog = dialog.child(
                div()
                    .flex()
                    .gap(inline_gap)
                    .justify_end()
                    .child(actions),
            );
        }

        dialog.into_any_element()
    }
}
