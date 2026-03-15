//! PugSpacer — flexible space filler for flex layouts.

use gpui::*;

/// A flexible space filler that expands to fill available space in flex layouts.
pub struct PugSpacer;

impl PugSpacer {
    pub fn new() -> Self {
        Self
    }
}

impl IntoElement for PugSpacer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        div().flex_grow().into_any_element()
    }
}
