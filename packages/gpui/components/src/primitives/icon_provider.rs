use gpui::*;

pub struct IconProvider {
    child: Option<AnyElement>,
}

impl IconProvider {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl Default for IconProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for IconProvider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.child.unwrap_or_else(|| div().into_any_element())
    }
}
