use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EmptyStateSpec, ErrorBoundarySpec, RemediationAction};

use crate::EmptyState;

pub struct ErrorBoundary {
    spec: ErrorBoundarySpec,
    theme: GpuiThemeProvider,
    child: Option<AnyElement>,
}

impl ErrorBoundary {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(ErrorBoundarySpec::new(), theme)
    }

    pub fn from_spec(spec: ErrorBoundarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            child: None,
        }
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl IntoElement for ErrorBoundary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        if let Some(message) = self.spec.error_message.clone() {
            return EmptyState::from_spec(
                EmptyStateSpec::new(self.spec.title)
                    .with_message(message)
                    .with_actions(vec![RemediationAction::new("retry", self.spec.retry_label)]),
                &self.theme,
            )
            .into_any_element();
        }

        self.child.unwrap_or_else(|| div().into_any_element())
    }
}
