//! PugSearchField — real GPUI component backed by SearchFieldSpec.
//!
//! Delegates rendering to PugTextInput via `as_text_input_spec()`.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SearchFieldSpec;

use crate::text_input::PugTextInput;

/// A real GPUI search field component backed by `SearchFieldSpec`.
pub struct PugSearchField {
    spec: SearchFieldSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_focus: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl PugSearchField {
    pub fn new(spec: SearchFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_focus: None,
        }
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_focus(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_focus = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugSearchField {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let input_spec = self.spec.as_text_input_spec();
        let mut input = PugTextInput::new(input_spec, &self.theme);

        if let Some(suffix) = self.id_suffix {
            input = input.with_id(suffix);
        }

        if let Some(handler) = self.on_focus {
            input = input.on_focus(handler);
        }

        input.into_element()
    }
}
