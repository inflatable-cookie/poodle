use gpui::*;
use poodle_specs::UiPresentationProviderSpec;

pub struct UiPresentationProvider {
    pub spec: UiPresentationProviderSpec,
    child: Option<AnyElement>,
}

impl UiPresentationProvider {
    pub fn new() -> Self {
        Self {
            spec: UiPresentationProviderSpec::new(),
            child: None,
        }
    }

    pub fn from_spec(spec: UiPresentationProviderSpec) -> Self {
        Self { spec, child: None }
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl Default for UiPresentationProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for UiPresentationProvider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.child.unwrap_or_else(|| div().into_any_element())
    }
}
