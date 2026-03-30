//! SearchField — real GPUI component backed by SearchFieldSpec.
//!
//! Delegates rendering to TextInput via `as_text_input_spec()`.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlSize, SearchFieldSpec, ValidationState};

use super::text_input::TextInput;

/// A real GPUI search field component backed by `SearchFieldSpec`.
pub struct SearchField {
    spec: SearchFieldSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_focus: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_submit: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_clear: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SearchField {
    type Target = SearchFieldSpec;
    fn deref(&self) -> &SearchFieldSpec { &self.spec }
}

impl SearchField {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SearchFieldSpec::new(), theme: theme.clone(), id_suffix: None, on_focus: None, on_change: None, on_submit: None, on_cancel: None, on_clear: None }
    }

    pub fn from_spec(spec: SearchFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_focus: None,
            on_change: None,
            on_submit: None,
            on_cancel: None,
            on_clear: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = v.into(); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = v.into(); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = v.into(); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn read_only(mut self, v: bool) -> Self { self.spec.is_read_only = v; self }
    pub fn show_clear_button(mut self, v: bool) -> Self { self.spec.show_clear_button = v; self }
    pub fn submit_enabled(mut self, v: bool) -> Self { self.spec.submit_enabled = v; self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn described_by(mut self, v: impl Into<String>) -> Self { self.spec.described_by = Some(v.into()); self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn size_role(mut self, v: poodle_primitives::SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn density(mut self, v: poodle_primitives::ControlDensity) -> Self { self.spec.density = v; self }


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

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler)); self
    }

    pub fn on_submit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Box::new(handler)); self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler)); self
    }

    pub fn on_clear(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_clear = Some(Box::new(handler)); self
    }
}

impl IntoElement for SearchField {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut input_spec = self.spec.as_text_input_spec();

        // Forward size/size_role/density so TextInput can resolve via presentation helpers
        input_spec.size = self.spec.size;
        input_spec.size_role = self.spec.size_role;
        input_spec.density = self.spec.density;

        // Contract fix: the spec emits trailing_icon("clear") but the actual
        // icon asset is "x". Remap so the Icon component resolves correctly.
        if input_spec.trailing_icon.as_deref() == Some("clear") {
            input_spec.trailing_icon = Some("x".to_string());
        }

        let mut input = TextInput::from_spec(input_spec, &self.theme);

        if let Some(suffix) = self.id_suffix {
            input = input.with_id(suffix);
        }

        if let Some(handler) = self.on_focus {
            input = input.on_focus(handler);
        }
        if let Some(handler) = self.on_change {
            input = input.on_change(handler);
        }
        if let Some(handler) = self.on_submit {
            input = input.on_submit(handler);
        }
        if let Some(handler) = self.on_cancel {
            input = input.on_cancel(handler);
        }

        input.into_element()
    }
}
