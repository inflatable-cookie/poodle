//! Separator — real GPUI component backed by SeparatorSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{RuleTone, SeparatorOrientation, SeparatorSpec};

use crate::theme_ext::{color_mix, resolve_color};

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
        let panel = resolve_color(&self.theme, "semantic.color.background.panel");

        // Contract: subtle tone applies 72% color-mix
        let color = match self.spec.tone {
            RuleTone::Subtle => color_mix(raw_color, panel, 0.72),
            RuleTone::Default => raw_color,
        };

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
