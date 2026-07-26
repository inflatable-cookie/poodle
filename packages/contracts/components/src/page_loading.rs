use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Presentation mode for PageLoading. Controls whether the loader
/// renders as a full-viewport overlay (the default) or as an inline
/// panel suitable for embedding inside a page region while its
/// content is loading.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PageLoadingPresentation {
    /// Full-viewport scrim with the progress block centred on top.
    #[default]
    Overlay,
    /// In-flow block that takes the width of its container and has
    /// no backdrop. Suitable for "loading this region" states.
    Inline,
}

impl PageLoadingPresentation {
    pub fn is_overlay(self) -> bool {
        matches!(self, Self::Overlay)
    }

    pub fn is_inline(self) -> bool {
        matches!(self, Self::Inline)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PageLoadingSpec {
    pub is_visible: bool,
    pub value: Option<f64>,
    pub max: f64,
    pub message: Option<String>,
    pub can_cancel: bool,
    /// Rendering mode — overlay (default) or inline. Matches the
    /// Svelte `presentation` prop.
    pub presentation: PageLoadingPresentation,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
}

impl PageLoadingSpec {
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new() -> Self {
        Self {
            is_visible: true,
            value: None,
            max: 100.0,
            message: None,
            can_cancel: false,
            presentation: PageLoadingPresentation::Overlay,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            aria_label: None,
        }
    }

    pub fn with_presentation(mut self, presentation: PageLoadingPresentation) -> Self {
        self.presentation = presentation;
        self
    }

    pub fn is_inline(&self) -> bool {
        self.presentation.is_inline()
    }

    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.is_visible = is_visible;
        self
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_can_cancel(mut self, can_cancel: bool) -> Self {
        self.can_cancel = can_cancel;
        self
    }

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    pub fn progress_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
