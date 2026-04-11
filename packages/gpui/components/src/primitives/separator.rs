//! Separator — real GPUI component backed by SeparatorSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{RuleTone, SeparatorOrientation, SeparatorSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI separator component backed by `SeparatorSpec`.
pub struct Separator {
    spec: SeparatorSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Separator {
    type Target = SeparatorSpec;
    fn deref(&self) -> &SeparatorSpec { &self.spec }
}

impl Separator {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SeparatorSpec::default(), theme: theme.clone() }
    }

    pub fn from_spec(spec: SeparatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn orientation(mut self, v: SeparatorOrientation) -> Self { self.spec.orientation = v; self }
    pub fn decorative(mut self, v: bool) -> Self { self.spec.decorative = v; self }
    pub fn tone(mut self, v: RuleTone) -> Self { self.spec.tone = v; self }
}

impl IntoElement for Separator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let raw_color = resolve_color(&self.theme, self.spec.resolved_color());

        // Contract: subtle tone applies 72% opacity approximation
        let color = match self.spec.tone {
            RuleTone::Subtle => Hsla { a: raw_color.a * 0.72, ..raw_color },
            RuleTone::Default => raw_color,
        };

        // Accessibility semantics (contract section 6):
        // When decorative=true: aria-hidden="true" (hidden from assistive technology)
        // When decorative=false: role="separator", aria-orientation="{orientation}"
        // GPUI: separator is never focusable in either mode

        match self.spec.orientation {
            SeparatorOrientation::Horizontal => {
                div().w_full().h(px(1.0)).bg(color).into_any_element()
            }
            SeparatorOrientation::Vertical => {
                div().h_full().w(px(1.0)).bg(color).into_any_element()
            }
        }
    }
}
