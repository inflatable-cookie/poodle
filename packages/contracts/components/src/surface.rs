use poodle_tokens::semantic;

use crate::types::{Inset, PaddingScale, SurfaceBorder, SurfaceRole, SurfaceTone};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceSpec {
    pub tone: SurfaceTone,
    pub border: SurfaceBorder,
    pub padding: PaddingScale,
    pub is_elevated: bool,
    pub role: Option<SurfaceRole>,
    pub label: Option<String>,
}

impl Default for SurfaceSpec {
    fn default() -> Self {
        Self {
            tone: SurfaceTone::Panel,
            border: SurfaceBorder::Subtle,
            padding: PaddingScale::Md,
            is_elevated: false,
            role: None,
            label: None,
        }
    }
}

impl SurfaceSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tone(mut self, tone: SurfaceTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_border(mut self, border: SurfaceBorder) -> Self {
        self.border = border;
        self
    }

    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_elevation(mut self, is_elevated: bool) -> Self {
        self.is_elevated = is_elevated;
        self
    }

    pub fn with_role(mut self, role: SurfaceRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// True when the surface renders in its elevated treatment, either via the
    /// explicit `is_elevated` flag or the `Elevated` tone (Svelte: the
    /// `[data-elevated="true"]` selector also matches `[data-tone="elevated"]`).
    pub fn is_elevated_resolved(&self) -> bool {
        self.is_elevated || self.tone == SurfaceTone::Elevated
    }

    /// Background fill token used as the dominant color in the fill color-mix.
    ///
    /// Contract §8 (Svelte `Surface.svelte`):
    /// - elevated → `color.background.elevated` (mixed over panel)
    /// - canvas   → `color.background.canvas`   (mixed with transparent)
    /// - panel/base → `color.background.surface` (mixed with transparent)
    ///
    /// NOTE: this intentionally differs from `SurfaceTone::background_token()`
    /// (which maps panel → `color.background.panel`). The base/panel fill in the
    /// contract mixes the **surface** background, not the panel background.
    pub fn resolved_background_token(&self) -> &'static str {
        if self.is_elevated_resolved() {
            semantic::COLOR_BACKGROUND_ELEVATED
        } else {
            match self.tone {
                SurfaceTone::Canvas => semantic::COLOR_BACKGROUND_CANVAS,
                // Panel tone + any non-elevated default uses the surface fill.
                SurfaceTone::Panel | SurfaceTone::Elevated => semantic::COLOR_BACKGROUND_SURFACE,
            }
        }
    }

    /// Secondary color the fill color-mix blends toward.
    ///
    /// Contract §8: elevated mixes elevated over `color.background.panel`; the
    /// non-elevated tones mix toward transparent (no second color token).
    pub fn fill_mix_over_token(&self) -> Option<&'static str> {
        if self.is_elevated_resolved() {
            Some(semantic::COLOR_BACKGROUND_PANEL)
        } else {
            None
        }
    }

    /// Fraction of `resolved_background_token()` in the fill color-mix.
    ///
    /// Contract §8 percentages (CSS `color-mix` percents — not token-backed;
    /// centralized here so neither runtime hardcodes the literal):
    /// - canvas: 98% (`color-mix(canvas 98%, transparent)`)
    /// - panel/elevated: 96%
    pub fn fill_mix_ratio(&self) -> f32 {
        if !self.is_elevated_resolved() && self.tone == SurfaceTone::Canvas {
            0.98
        } else {
            0.96
        }
    }

    pub fn resolved_border_color(&self) -> Option<&'static str> {
        self.border.color_token()
    }

    pub fn resolved_border_width(&self) -> Option<&'static str> {
        self.border.width_token()
    }

    /// Fraction of the border color retained when the border is `Subtle`.
    ///
    /// Contract §8 / Svelte: `color-mix(border-subtle 74%, transparent)`. A
    /// `Default` border uses the full color (ratio `1.0`). CSS color-mix percent
    /// — centralized here rather than hardcoded in each runtime.
    pub fn border_mix_ratio(&self) -> f32 {
        match self.border {
            SurfaceBorder::Subtle => 0.74,
            _ => 1.0,
        }
    }

    pub fn resolved_shadow_token(&self) -> &'static str {
        // Contract §8 elevated targets `--poodle-elevation-surface`.
        semantic::ELEVATION_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.panel_inset()
    }
}
