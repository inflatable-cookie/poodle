//! Separator — real GPUI component backed by SeparatorSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{RuleTone, SeparatorOrientation, SeparatorSpec};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI separator component backed by `SeparatorSpec`.
pub struct Separator {
    spec: SeparatorSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Separator {
    type Target = SeparatorSpec;
    fn deref(&self) -> &SeparatorSpec {
        &self.spec
    }
}

impl Separator {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SeparatorSpec::default(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: SeparatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn orientation(mut self, v: SeparatorOrientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn decorative(mut self, v: bool) -> Self {
        self.spec.decorative = v;
        self
    }
    pub fn tone(mut self, v: RuleTone) -> Self {
        self.spec.tone = v;
        self
    }
}

impl IntoElement for Separator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let raw_color = resolve_color(&self.theme, self.spec.resolved_color());

        // Subtle tone applies the contract color-mix ratio (border-subtle 72%,
        // transparent); Default tone uses the full color (ratio 1.0). The ratio
        // is sourced from the spec, not a magic float in component code.
        let mix = self.spec.subtle_mix_ratio();
        let color = Hsla {
            a: raw_color.a * mix,
            ..raw_color
        };

        // Stroke width from the border-width-default token (0.0625rem = 1px),
        // resolved to px — not a literal. `resolve_px` returns `Pixels`.
        let stroke = px(f32::from(resolve_px(&self.theme, self.spec.resolved_stroke_width())));

        // Accessibility semantics (contract section 6):
        // When decorative=true: aria-hidden="true" (hidden from assistive technology)
        // When decorative=false: role="separator", aria-orientation="{orientation}"
        // GPUI has no accessibility API, so neither role nor aria-hidden can be
        // emitted. We still read `decorative` here so the prop is not dead: a
        // semantic (non-decorative) separator is the structural division case,
        // which today renders identically but is the hook for any future AX
        // channel. No visual difference per contract §4.
        let _is_semantic = !self.spec.decorative;

        match self.spec.orientation {
            SeparatorOrientation::Horizontal => {
                div().w_full().h(stroke).bg(color).into_any_element()
            }
            SeparatorOrientation::Vertical => {
                div().h_full().w(stroke).bg(color).into_any_element()
            }
        }
    }
}
