//! FormActions — real GPUI component backed by FormActionsSpec.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{FormActionAlign, FormActionsSpec};


/// A real GPUI form actions bar backed by `FormActionsSpec`.
///
/// Lays out action buttons (submit, cancel, etc.) with configurable alignment.
pub struct FormActions {
    spec: FormActionsSpec,
    theme: GpuiThemeProvider,
    actions: Vec<AnyElement>,
}

impl std::ops::Deref for FormActions {
    type Target = FormActionsSpec;
    fn deref(&self) -> &FormActionsSpec { &self.spec }
}

impl FormActions {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: FormActionsSpec::new(), theme: theme.clone(), actions: Vec::new() }
    }

    pub fn from_spec(spec: FormActionsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn align(mut self, v: FormActionAlign) -> Self { self.spec.align = v; self }


    /// Add an action element (typically a Button).
    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

impl IntoElement for FormActions {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let gap = theme.resolve_space(self.spec.action_gap_token());
        let separation = theme.resolve_space(self.spec.stack_separation_token());

        // Contract says no border, only padding-top for separation
        let mut row = div()
            .flex()
            .flex_wrap() // contract: wraps on narrow widths
            .items_center()
            .gap(px(gap))
            .pt(px(separation));

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
