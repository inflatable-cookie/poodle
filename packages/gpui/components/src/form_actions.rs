//! PugFormActions — real GPUI component backed by FormActionsSpec.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{FormActionAlign, FormActionsSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI form actions bar backed by `FormActionsSpec`.
///
/// Lays out action buttons (submit, cancel, etc.) with configurable alignment.
pub struct PugFormActions {
    spec: FormActionsSpec,
    theme: GpuiThemeProvider,
    actions: Vec<AnyElement>,
}

impl PugFormActions {
    pub fn new(spec: FormActionsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: Vec::new(),
        }
    }

    /// Add an action element (typically a PugButton).
    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

impl IntoElement for PugFormActions {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let border = resolve_color(theme, "semantic.color.border.default");
        let gap = theme.resolve_space(self.spec.action_gap_token());
        let separation = theme.resolve_space(self.spec.stack_separation_token());

        let mut row = div()
            .flex()
            .items_center()
            .gap(px(gap))
            .pt(px(separation))
            .border_t_1()
            .border_color(border);

        // Alignment
        match self.spec.align {
            FormActionAlign::Start => {
                row = row.justify_start();
            }
            FormActionAlign::End => {
                row = row.justify_end();
            }
            FormActionAlign::Between => {
                row = row.justify_between();
            }
        }

        for action in self.actions {
            row = row.child(action);
        }

        row.into_any_element()
    }
}
